use serde::{Deserialize, Serialize};
use utilites::http::Bytes;

///http://publication.pravo.gov.ru/api/SignatoryAuthorities?
/// список всех органов
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SignatoryAuthority
{
    pub id: String,
   // pub is_main: bool,
    pub weight: u32,
    pub name: String,
}

impl From<Bytes> for SignatoryAuthority
{
    fn from(value: Bytes) -> Self 
    {
        serde_json::from_slice(&value).unwrap()
    }
}