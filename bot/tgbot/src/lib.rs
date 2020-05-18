mod types;

pub struct TelegramBot {
    client: reqwest::Client,
    token: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            /*
             * DoubleError::EmptyVec =>
             *     write!(f, "please use a vector with at least one element"),
             */
            Error::RequestError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::RequestError(ref e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::RequestError(error)
    }
}

impl TelegramBot {
    pub fn new(token: &str) -> Result<TelegramBot> {
        let client = reqwest::Client::builder().build()?;
        Ok(TelegramBot {
            client,
            token: String::from(token),
        })
    }

    pub async fn get_me(&self) -> Result<types::User> {
        let url = self.get_url("getMe");
        let data = self.client.get(&url).send()
            .await?
            .error_for_status()?
            .json::<types::User>()
            .await?;
        Ok(data)
    }

    pub async fn get_updates(&self, params: &types::GetUpdatesParams) -> Result<Vec<types::Message>> {
        let url = self.get_url("getUpdates");
        let data = self.client.post(&url)
            .json(params)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<types::Message>>()
            .await?;
        Ok(data)
    }

    fn get_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", &self.token, method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_me() {
        let bot = TelegramBot::new("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11").expect("could not new telegram bot");
        let result = bot.get_me().await;
        assert!(result.is_err(), "");
        match result {
            Err(e) => {
                match e {
                    Error::RequestError(e) => assert_eq!(e.status(), Some(reqwest::StatusCode::UNAUTHORIZED)),
                }
            },
            Ok(_) => assert!(false),
        }
    }
}
