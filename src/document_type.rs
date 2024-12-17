use serde::{Deserialize, Serialize};
use utilites::http::Bytes;

/// запрос видов документов для конкретного органа api/DocumentTypes?SignatoryAuthorityId=8005d8c9-4b6d-48d3-861a-2a37e69fccb3 
/// ответ на http://publication.pravo.gov.ru/api/DocumentTypes
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct DocumentType
{
    pub id: String,
    /// Вес типа документа при сортировке (0-100)
    pub weight: u32, 
    /// Наименование вида документа  
    /// "Закон Российской Федерации о поправке к Конституции Российской Федерации"
    pub name: String
}

impl From<Bytes> for DocumentType
{
    fn from(value: Bytes) -> Self 
    {
        serde_json::from_slice(&value).unwrap()
    }
}
