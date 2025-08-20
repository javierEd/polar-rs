#![doc = include_str!("../README.md")]

use std::error::Error;
use std::fmt::Display;

use reqwest::{IntoUrl, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod enums;
mod models;

pub use enums::*;
pub use models::*;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub enum PolarError {
    NotFound,
    Request(String),
    Unauthorized,
    Unknown(String),
    Validation(String),
}

impl Display for PolarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolarError::Request(msg) => write!(f, "Request error: {msg}"),
            PolarError::NotFound => write!(f, "Not found"),
            PolarError::Unauthorized => write!(f, "Unauthorized"),
            PolarError::Unknown(msg) => write!(f, "Unknown error: {msg}"),
            PolarError::Validation(msg) => write!(f, "Validation error: {msg}"),
        }
    }
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

impl Error for PolarError {}

pub type PolarResult<T> = Result<T, PolarError>;

pub struct Polar {
    base_url: reqwest::Url,
    access_token: String,
}

impl Polar {
    pub fn new<U: IntoUrl, T: Display>(base_url: U, access_token: T) -> PolarResult<Self> {
        if access_token.to_string().is_empty() {
            return Err(PolarError::Request("access_token cannot be empty".to_owned()));
        }

        let base_url = if let Ok(mut url) = base_url.into_url() {
            if !url.path().ends_with('/') {
                url.set_path(&format!("{}/", url.path()))
            }

            url
        } else {
            return Err(PolarError::Request("base_url is not a valid URL".to_owned()));
        };

        Ok(Self {
            base_url,
            access_token: access_token.to_string(),
        })
    }

    pub async fn delete<T>(&self, path: &str) -> PolarResult<T>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .delete(self.base_url.join(path)?)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            StatusCode::NOT_FOUND => Err(PolarError::NotFound),
            StatusCode::UNPROCESSABLE_ENTITY => Err(PolarError::Validation(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(PolarError::Unauthorized),
            _ => Err(PolarError::Unknown(response.text().await?)),
        }
    }

    pub async fn get<T>(&self, path: &str) -> PolarResult<T>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .get(self.base_url.join(path)?)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            StatusCode::NOT_FOUND => Err(PolarError::NotFound),
            StatusCode::UNPROCESSABLE_ENTITY => Err(PolarError::Validation(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(PolarError::Unauthorized),
            _ => Err(PolarError::Unknown(response.text().await?)),
        }
    }

    pub async fn patch<P, T>(&self, path: &str, params: &P) -> PolarResult<T>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .patch(self.base_url.join(path)?)
            .bearer_auth(&self.access_token)
            .json(params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            StatusCode::NOT_FOUND => Err(PolarError::NotFound),
            StatusCode::UNPROCESSABLE_ENTITY => Err(PolarError::Validation(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(PolarError::Unauthorized),
            _ => Err(PolarError::Unknown(response.text().await?)),
        }
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
            StatusCode::UNPROCESSABLE_ENTITY => Err(PolarError::Validation(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(PolarError::Unauthorized),
            _ => Err(PolarError::Unknown(response.text().await?)),
        }
    }

    /// **Create a checkout session.**
    ///
    /// Scopes: `checkouts:write`
    ///
    /// Reference: <https://docs.polar.sh/api-reference/checkouts/create-session>
    pub async fn create_checkout_session(&self, params: &CheckoutSessionParams) -> PolarResult<CheckoutSession> {
        self.post("checkouts", params).await
    }

    /// **Get a checkout session by ID.**
    ///
    /// Scopes: `checkouts:read` `checkouts:write`
    ///
    /// Reference: <https://docs.polar.sh/api-reference/checkouts/get-session>
    pub async fn get_checkout_session(&self, id: Uuid) -> PolarResult<CheckoutSession> {
        self.get(&format!("checkouts/{id}")).await
    }

    /// **Update a subscription.**
    ///
    /// Scopes: `subscriptions:write`
    ///
    /// Reference: <https://docs.polar.sh/api-reference/subscriptions/update>
    pub async fn update_subscription(&self, id: Uuid, params: &SubscriptionParams) -> PolarResult<Subscription> {
        self.patch(&format!("subscriptions/{id}"), params).await
    }

    /// **Revoke a subscription, i.e cancel immediately.**
    ///
    /// Scopes: `subscriptions:write`
    ///
    /// Reference: <https://docs.polar.sh/api-reference/subscriptions/revoke>
    pub async fn revoke_subscription(&self, id: Uuid) -> PolarResult<Subscription> {
        self.delete(&format!("subscriptions/{id}")).await
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

    #[tokio::test]
    async fn should_get_checkout_session() {
        let checkout_id = Uuid::new_v4();
        let mock = get_mock(
            "GET",
            &format!("/checkouts/{}", checkout_id),
            200,
            get_fixture::<Value>("checkout_session"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let result = polar.get_checkout_session(checkout_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_checkout_session() {
        let checkout_id = Uuid::new_v4();
        let mock = get_mock(
            "GET",
            &format!("/checkouts/{}", checkout_id),
            404,
            get_fixture::<Value>("not_found"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let result = polar.get_checkout_session(checkout_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_update_subscription() {
        let subscription_id = Uuid::new_v4();
        let mock = get_mock(
            "PATCH",
            &format!("/subscriptions/{}", subscription_id),
            200,
            get_fixture::<Value>("subscription"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let params = get_fixture("subscription_params");

        let result = polar.update_subscription(subscription_id, &params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_subscription() {
        let subscription_id = Uuid::new_v4();
        let mock = get_mock(
            "PATCH",
            &format!("/subscriptions/{}", subscription_id),
            422,
            get_fixture::<Value>("unprocessable_entity"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let params = get_fixture("subscription_params");

        let result = polar.update_subscription(subscription_id, &params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_revoke_subscription() {
        let subscription_id = Uuid::new_v4();
        let mock = get_mock(
            "DELETE",
            &format!("/subscriptions/{}", subscription_id),
            200,
            get_fixture::<Value>("subscription"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let result = polar.revoke_subscription(subscription_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_revoke_subscription() {
        let subscription_id = Uuid::new_v4();
        let mock = get_mock(
            "DELETE",
            &format!("/subscriptions/{}", subscription_id),
            422,
            get_fixture::<Value>("unprocessable_entity"),
        )
        .await;

        let polar = get_test_polar(mock.uri());

        let result = polar.revoke_subscription(subscription_id).await;

        assert!(result.is_err());
    }
}
