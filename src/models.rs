use serde::{Deserialize, Serialize};
use utilites::{Date, Url, http::Bytes, http::Uri, empty_string_as_none};
use crate::{DocumentType, error::PublicationApiError, SignatoryAuthority};

use super::deserialization::deserialize_date;
///Карточка документа получаемая при поиске на портале 
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct PublicationDocumentCard
{
    /// Номер электронного опубликования  
    /// "0001202406220019"
    pub eo_number: String, 
    /// Признак: у документа есть прикрепленные SVG файлы  
    /// false
    pub has_svg: bool, 
    /// Размер ZIP файла с приложениями, если таковой имеется  
    /// null
    pub zip_file_length: Option<u32>, 
    #[serde(deserialize_with="deserialize_date")]
    /// Дата публикации НПА в формате ISO 8601  
    /// "2024-06-22T00:00:00"
    pub publish_date_short: Date,	
    /// Полное составное название документа, включающее Вид, Дату и Номер документа, Принявший орган  
    /// 'Федеральный закон от 22.06.2024 № 160-ФЗ\n "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
    pub complex_name: String, 
    ///	Количество страниц в PDF файле документа  
    /// 4
    pub pages_count: u32,
    ///мое свойство для получения текущей страницы
    #[serde(skip)]
    pub curr_page: u32,
    ///	Размер PDF файла документа  
    /// 169841
    pub pdf_file_length: u32,
    /// Номер регистрации НПА в Минюсте  
    /// null
    pub jd_reg_number: Option<String>,	
    /// Дата регистрации НПА в Минюсте  
    /// null
    #[serde(deserialize_with="empty_string_as_none")]
    pub jd_reg_date: Option<String>,	
    /// Заголовок документа  
    /// 'Федеральный закон от 22.06.2024 № 160-ФЗ<br /> "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
    pub title: String,	
    /// Дата публикации в формате DD.MM.YYYY  
    /// "22.06.2024"
    #[serde(deserialize_with="deserialize_date")]
    pub view_date: Date,	
    /// GUID документа  
    /// "118e71c6-7e90-495c-9afb-56b38edea17a"
    pub id: String,
    /// sa id
    pub signatory_authority_id: String,
    /// id типа документа
    pub document_type_id: String,
    #[serde(deserialize_with="deserialize_date")]
    /// дата подписания документа
    pub document_date: Date,
    /// номер документа
    pub number: String
}
impl From<Bytes> for PublicationDocumentCard
{
    fn from(value: Bytes) -> Self 
    {
        serde_json::from_slice(&value).unwrap()
    }
}



///Список карточек документов получаемых при поиске на портале 
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SearchResult
{
    /// Список документов - представлен как набор нескольких вложенных объектов (записей):
    pub items: Vec<PublicationDocumentCard>,
    /// Всего элементов (документов) в итоговой выборке
    pub items_total_count: u32,
    /// количество элементов отображаемых на одной странице
    pub items_per_page: u32,
    /// текущая страница
    pub current_page: u32,
    /// всего страниц в найденом
    pub pages_total_count: u32
}
impl From<Bytes> for SearchResult
{
    fn from(value: Bytes) -> Self 
    {
        serde_json::from_slice(&value).unwrap()
    }
}

/// ответ на http://publication.pravo.gov.ru/api/Document?eoNumber=0001202406220019
/// Полная карточка документа с портала опубликования, возможно получить только по id PublicationDocumentCard
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ExtendedPublicationDocumentCard
{
    pub id: String,
    pub document_type: DocumentType,
    pub signatory_authorities: Vec<SignatoryAuthority>,
    /// Номер электронного опубликования  
    /// "0001202406220019"
    pub eo_number: String, 
     /// Размер в байтах ZIP файла с приложениями, если таковой имеется
     pub zip_file_length: Option<u32>,
     /// Признак: у документа есть прикрепленные SVG файлы
     pub has_svg: bool,
    /// Полное составное название документа, включающее Вид, Дату и Номер документа, Принявший орган  
    /// 'Федеральный закон от 22.06.2024 № 160-ФЗ\n "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
    pub complex_name: String, 
    /// Количество страниц в PDF файле документа
    pub pages_count: u32,
    /// Размер в байтах PDF файла документа
    pub pdf_file_length: u32,
    /// Номер регистрации НПА в Минюсте  
    /// null
    pub jd_reg_number: Option<u32>,	
    /// Дата регистрации НПА в Минюсте  
    /// null
    pub jd_reg_date: Option<String>,	
    #[serde(deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
    pub number: String,
    #[serde(deserialize_with="deserialize_date")]
    ///дата подписания документа
    pub document_date: Date,
    #[serde(deserialize_with="deserialize_date")]
    /// Дата публикации НПА в формате ISO 8601  
    /// "2024-06-22T00:00:00"
    pub publish_date_short: Date,	
	/// Дата публикации в формате DD.MM.YYYY  
    /// "22.06.2024"
    #[serde(deserialize_with="deserialize_date")]
    pub view_date: Date,
	/// id словаря signatory authority
    pub signatory_authority_id: String,	
    /// id словаря signatory authority
    pub document_type_id: String,	
}

impl From<Bytes> for ExtendedPublicationDocumentCard
{
    fn from(value: Bytes) -> Self 
    {
        serde_json::from_slice(&value).unwrap()
    }
}

