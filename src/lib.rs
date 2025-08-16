use std::fmt::Display;

use reqwest::{IntoUrl, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod enums;
mod models;

pub use enums::*;
pub use models::*;

#[derive(Debug, Deserialize)]
pub struct PolarValidationError {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}

#[derive(Debug)]
pub enum PolarError {
    Request(String),
    Unauthorized,
    Unknown(String),
    Validation { details: Vec<PolarValidationError> },
}

pub type PolarResult<T> = Result<T, PolarError>;

pub struct Polar {
    base_url: reqwest::Url,
    access_token: String,
}

impl From<reqwest::Error> for PolarError {
    fn from(err: reqwest::Error) -> Self {
        PolarError::Request(err.to_string())
    }
}

impl From<url::ParseError> for PolarError {
    fn from(err: url::ParseError) -> Self {
        PolarError::Request(err.to_string())
    }
}

impl Polar {
    pub fn new<U: IntoUrl, T: Display>(base_url: U, access_token: T) -> PolarResult<Self> {
        if access_token.to_string().is_empty() {
            return Err(PolarError::Request("access_token cannot be empty".to_owned()));
        }

        Ok(Self {
            base_url: base_url.into_url()?,
            access_token: access_token.to_string(),
        })
    }

    pub async fn post<P, T>(&self, path: &str, params: &P) -> PolarResult<T>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .post(self.base_url.join(path)?)
            .bearer_auth(&self.access_token)
            .json(params)
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json().await.unwrap()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(PolarError::Validation {
                details: response.json().await?,
            }),
            StatusCode::UNAUTHORIZED => Err(PolarError::Unauthorized),
            _ => Err(PolarError::Unknown(response.text().await?)),
        }
    }

    pub async fn create_checkout_session(&self, params: &CheckoutSessionParams) -> PolarResult<CheckoutSession> {
        self.post("checkouts", params).await
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use serde_json::{Value, from_reader};
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

    use super::*;

    fn get_fixture<T: DeserializeOwned>(name: &str) -> T {
        let file = File::open(format!("fixtures/{name}.json")).unwrap();
        let reader = BufReader::new(file);

        from_reader(reader).unwrap()
    }

    async fn get_mock<B: Serialize>(method: &str, path: &str, status_code: u16, body: B) -> MockServer {
        let mock_server = MockServer::start().await;

        Mock::given(matchers::method(method))
            .and(matchers::path(path))
            .respond_with(ResponseTemplate::new(status_code).set_body_json(body))
            .mount(&mock_server)
            .await;

        mock_server
    }

    fn get_test_polar(base_url: String) -> Polar {
        Polar::new(base_url, "123").ok().unwrap()
    }

    #[test]
    fn should_get_polar_when_arguments_are_valid() {
        let result = Polar::new("https://sandbox-api.polar.sh/v1/", "123");

        assert!(result.is_ok());
    }

    #[test]
    fn should_not_get_polar_when_base_url_is_invalid() {
        let result = Polar::new("/v1/", "123");

        assert!(result.is_err());
    }

    #[test]
    fn should_not_get_polar_when_access_token_is_empty() {
        let result = Polar::new("https://sandbox-api.polar.sh/v1/", "");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_create_checkout_session() {
        let mock = get_mock("POST", "/checkouts", 201, get_fixture::<Value>("checkout_session")).await;

        let polar = get_test_polar(mock.uri());

        let params = get_fixture("checkout_session_params");

        let result = polar.create_checkout_session(&params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_create_checkout_session() {
        let mock = get_mock("POST", "/checkouts", 422, get_fixture::<Value>("unprocessable_entity")).await;

        let polar = get_test_polar(mock.uri());

        let params = get_fixture("checkout_session_params");

        let result = polar.create_checkout_session(&params).await;

        assert!(result.is_err());
    }
}
