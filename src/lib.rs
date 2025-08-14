use std::fmt::Display;

pub enum PolarError {
    Request(String),
}

pub type PolarResult<T> = Result<T, PolarError>;

#[allow(dead_code)]
pub struct Polar {
    base_url: reqwest::Url,
    access_token: String,
}

trait IntoPolarResult<T> {
    fn into_polar_result(self) -> PolarResult<T>;
}

impl<T> IntoPolarResult<T> for reqwest::Result<T> {
    fn into_polar_result(self) -> PolarResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(PolarError::Request(err.to_string())),
        }
    }
}

impl Polar {
    pub fn new<U: reqwest::IntoUrl, T: Display>(base_url: U, access_token: T) -> PolarResult<Self> {
        Ok(Self {
            base_url: base_url.into_url().into_polar_result()?,
            access_token: access_token.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_polar_when_arguments_are_valid() {
        let result = Polar::new("https://sandbox-api.polar.sh/v1", "123");

        assert!(result.is_ok());
    }

    #[test]
    fn should_not_get_polar_when_base_url_is_invalid() {
        let result = Polar::new("/v1", "123");

        assert!(result.is_err());
    }
}
