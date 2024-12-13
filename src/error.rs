use thiserror::Error;

#[derive(Error, Debug)]
pub enum PublicationApiError 
{
    // #[error(transparent)]
    // HyperError(#[from] hyper::Error),
    // #[error(transparent)]
    // HyperHttpError(#[from] hyper::http::Error),
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Ошибка подключения к сервису `{0}` при отправке сообщения")]
    SendError(String),
    #[error("Ошибка получения информации с api сервиса опубликования -> `{0}`")]
    ApiError(String),
    #[error(transparent)]
    UtilitesError(#[from] utilites::error::Error),
}

impl serde::Serialize for PublicationApiError 
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
  S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}