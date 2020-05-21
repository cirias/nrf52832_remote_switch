pub mod types;

pub struct TelegramBot {
    client: reqwest::Client,
    token: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<types::ResponseBody<T>> for Result<T> {
    fn from(body: types::ResponseBody<T>) -> Result<T> {
        if body.result.is_some() {
            return Ok(body.result.unwrap())
        }

        Err(Error::ResponseError(body.description))
    }
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ResponseError(Option<String>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::RequestError(ref e) => e.fmt(f),
            Error::ResponseError(ref e) => {
                match e {
                    Some(ref s) => write!(f, "{}", s),
                    None => write!(f, "unknown error"),
                }
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::RequestError(ref e) => Some(e),
            Error::ResponseError(_) => None,
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
        let body = self.client.get(&url).send()
            .await?
            .error_for_status()?
            .json::<types::ResponseBody<types::User>>()
            .await?;
        body.into()
    }

    pub async fn get_updates(&self, params: &types::GetUpdatesParams) -> Result<Vec<types::Update>> {
        let url = self.get_url("getUpdates");
        let body = self.client.post(&url)
            .json(params)
            .send()
            .await?
            .error_for_status()?
            .json::<types::ResponseBody<Vec<types::Update>>>()
            .await?;
        body.into()
    }

    pub async fn send_message(&self, params: &types::SendMessageParams) -> Result<types::Message> {
        let url = self.get_url("sendMessage");
        let body = self.client.post(&url)
            .json(params)
            .send()
            .await?
            .error_for_status()?
            .json::<types::ResponseBody<types::Message>>()
            .await?;
        body.into()
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
                    Error::ResponseError(_) => assert!(false),
                }
            },
            Ok(_) => assert!(false),
        }
    }
}
