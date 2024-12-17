use utilites::http::{HeaderName, StatusCode, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HOST, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT};
use utilites::{http::{Bytes, HyperClient, Uri}, Date, Url};

use crate::{DocumentType, SignatoryAuthority};
use crate::{error::PublicationApiError, ExtendedPublicationDocumentCard};
use super::models::{PublicationDocumentCard, SearchResult};
const API: &str = "http://95.173.157.131:80/api/";
const SITE: &str = "http://95.173.157.131:80/";
///Вроде бы при увеличении количества попыток, все заработало
pub struct PublicationApi
{
    
}
impl PublicationApi
{
    fn client() -> HyperClient
    {
        HyperClient::new_with_timeout(API.parse().unwrap(), 1000, 3000, 12).with_headers(Self::headers())
    }
    fn png_client() -> HyperClient
    {
        HyperClient::new_with_timeout(SITE.parse().unwrap(), 300, 500, 15).with_headers(Self::headers())
    }
    fn pdf_client() -> HyperClient
    {
        HyperClient::new_with_timeout(SITE.parse().unwrap(), 10000, 14000, 10).with_headers(Self::headers())
    }
    fn headers() -> Vec<(HeaderName, String)>
    {
        let mut h= Vec::new();
        h.push((HOST, "publication.pravo.gov.ru".to_owned()));
        h.push((USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0".to_owned()));
        h.push((ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_owned()));
        h.push((ACCEPT_ENCODING, "gzip, deflate".to_owned()));
        h.push((ACCEPT_LANGUAGE, "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3".to_owned()));
        h.push((REFERER, "http:://publication.pravo.gov.ru".to_owned()));
        h.push((UPGRADE_INSECURE_REQUESTS, "1".to_owned()));
        h
    }

    ///Проверка что пришел код 200 на запрос
    fn code_error_check(response: (StatusCode, Bytes)) -> Result<Bytes, PublicationApiError>
    {
        if response.0 != utilites::http::StatusCode::OK
        {
            let e = ["Сервер ответил кодом ", response.0.as_str(), " ожидался код 200"].concat();
            logger::warn!("{}", &e);
            return Err(PublicationApiError::ApiError(e));
        }
        else 
        {
            Ok(response.1)
        }
    }
    fn get_params(date_from: &Date, date_to: Option<&Date>, doc_types: &[String], signatory_authority: Option<&String>, page_number: Option<u32>) -> Vec<(String, String)>
    {
        let mut params : Vec<(String, String)> = Vec::new();
        for dt in doc_types
        {
            params.push(("DocumentTypes".to_owned(), dt.to_owned()));
        }
        if let Some(sa) = signatory_authority
        {
            params.push(("SignatoryAuthorityId".to_owned(), sa.to_owned()));
        }
        params.push(("DocumentDateFrom".to_owned(), date_from.format(utilites::DateFormat::DotDate)));
        if let Some(dt) = date_to
        {
            params.push(("DocumentDateTo".to_owned(), dt.format(utilites::DateFormat::DotDate)));
        }
        params.push(("PageSize".to_owned(), "200".to_owned()));
        if let Some(pn) = page_number
        {
            params.push(("index".to_owned(), pn.to_string()));
        }
        params
    }
   
    ///Получение списка документов за определенную дату
    pub async fn get_documents(date_from: &Date, date_to: Option<&Date>, doc_types: &[String], signatory_authority: Option<&String>, page_number: Option<u32>) -> Result<Vec<PublicationDocumentCard>, PublicationApiError>
    {
        let mut client = Self::client();
        client = client.add_path("Documents");
        let params = Self::get_params(date_from, date_to, doc_types, signatory_authority, page_number);
        let value = client.get_with_params(&params).await?;
        let body = Self::code_error_check(value)?;
        let resp: SearchResult = serde_json::from_slice(&body)?;
        let mut result_vec = Vec::with_capacity(resp.items_total_count as usize);
        result_vec.extend(resp.items);
        for page in (resp.current_page+1)..=resp.pages_total_count
        {
            let params = Self::get_params(date_from, date_to, doc_types, signatory_authority, Some(page));
            let value = client.get_with_params(&params).await?;
            let body = Self::code_error_check(value)?;
            let resp: SearchResult = serde_json::from_slice(&body)?;
            result_vec.extend(resp.items);
        }
        Ok(result_vec)
    }
    ///получение списка документов с определенной даты
    pub async fn get_documents_from_date(date_from: &Date, doc_types: &[String], signatory_authority: Option<&String>, page_number: Option<u32>) -> Result<Vec<PublicationDocumentCard>, PublicationApiError>
    {
        let resp = Self::get_documents(date_from, None, doc_types, signatory_authority, page_number).await?;
        Ok(resp)
    }
    ///получение списка документов за период
    pub async fn get_documents_for_period(date_from: &Date, date_to: &Date, doc_types: &[String], signatory_authority: Option<&String>, page_number: Option<u32>) -> Result<Vec<PublicationDocumentCard>, PublicationApiError>
    {
        let resp = Self::get_documents(date_from, Some(date_to), doc_types, signatory_authority, page_number).await?;
        Ok(resp)
    }
    /// список видов документов  
    /// http://publication.pravo.gov.ru/api/DocumentTypes
    pub async fn get_documents_types() -> Result<Vec<DocumentType>, PublicationApiError>
    {
        let mut client = Self::client();
        client = client.add_path("DocumentTypes");
        let _p : Vec<(&str, &str)> = Vec::new();
        let value = client.get_with_params(&_p).await?;
        let body = Self::code_error_check(value)?;
        let resp: Vec<DocumentType> = serde_json::from_slice(&body)?;
        Ok(resp)

    }
     /// список видов документов у конкретного принявшего органа
    /// http://publication.pravo.gov.ru/api/DocumentTypes
    pub async fn get_documents_types_by_signatory_authority(signatory_authority: &str) -> Result<Vec<DocumentType>, PublicationApiError>
    {
        let mut client = Self::client();
        client = client.add_path("DocumentTypes");
        let papams : Vec<(&str, &str)> = vec![("SignatoryAuthorityId", signatory_authority)];
        let value = client.get_with_params(&papams).await?;
        let body = Self::code_error_check(value)?;
        let resp: Vec<DocumentType> = serde_json::from_slice(&body)?;
        Ok(resp)

    }
    /// список принявших органов  
    /// http://publication.pravo.gov.ru/api/SignatoryAuthorities?
    pub async fn get_signatory_authorites() -> Result<Vec<SignatoryAuthority>, PublicationApiError>
    {
        let mut client = Self::client();
        client = client.add_path("SignatoryAuthorities");
        let _p : Vec<(&str, &str)> = Vec::new();
        let value = client.get_with_params(&_p).await?;
        let body = Self::code_error_check(value)?;
        let resp: Vec<SignatoryAuthority> = serde_json::from_slice(&body)?;
        Ok(resp)

    }
    /// подробная карточка документа 
    ///  http://publication.pravo.gov.ru/api/Document?eoNumber=0001202406220019
    pub async fn get_document_extended_card(eo_number: &str) -> Result<ExtendedPublicationDocumentCard, PublicationApiError>
    {
        let mut client = Self::client();
        client = client.add_path("Document");
        let params : Vec<(&str, &str)> = vec![("eoNumber", eo_number)];
        let value = client.get_with_params(&params).await?;
        let body = Self::code_error_check(value)?;
        let resp: ExtendedPublicationDocumentCard = serde_json::from_slice(&body)?;
        Ok(resp)
    }
    ///получение картинки
    /// http://publication.pravo.gov.ru/GetImage?documentId=dbf8d1c9-ed98-46ae-8cfb-1f7eb0fa066e&pageNumber=1
    pub  async fn get_image_by_id(id: &str, page: u32) -> Result<Bytes, PublicationApiError>
    {
        let mut client = Self::png_client();
        client = client.add_path("GetImage");
        let page = page.to_string();
        let params : Vec<(&str, &str)> = 
        vec![
            ("documentId", id),
            ("pageNumber", &page)
        ];
        let value = client.get_with_params(&params).await?;
        let body = Self::code_error_check(value)?;
        Ok(body)
    }
    /// загрузка pdf http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202308040071"
    pub  async fn get_pdf_by_eo_number(eo_number: &str) -> Result<Bytes, PublicationApiError>
    {
        let mut client = Self::pdf_client();
        client = client.add_path("file/pdf");
        let params : Vec<(&str, &str)> = 
        vec![
            ("eoNumber", eo_number),
        ];
        let value = client.get_with_params(&params).await?;
        let body = Self::code_error_check(value)?;
        Ok(body)
    }
    // Document?eoNumber=0001202406220019
    // Document?eoNumиer=2600202407050002

}

impl PublicationDocumentCard
{
    async fn next_image(&mut self) -> Result<Option<Bytes>, PublicationApiError> 
    {
        {
            if self.curr_page == 0
            {
                self.curr_page = 1;
            }
            if self.curr_page <= self.pages_count
            {
                let png = PublicationApi::get_image_by_id(&self.id, self.curr_page).await?;
                self.curr_page +=1;
                Ok(Some(png))
            }
            else
            {
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use std::time::Duration;

    use logger::StructLogger;
    use utilites::Date;

    pub use super::super::PublicationDocumentCard;
    use super::PublicationApi;

    #[tokio::test]
    async fn test_get_images()
    {
        StructLogger::new_default();
        let u = PublicationApi::get_documents_from_date(&Date::parse("01.04.2024").unwrap(), &["82a8bf1c-3bc7-47ed-827f-7affd43a7f27".to_owned()],None, None).await.unwrap();
        //let mut d = PublicationDocumentCard { eo_number: "0001202406220019".to_owned(), has_svg: false, zip_file_length: None, publish_date_short:  Date::parse("2024-06-22T00:00:00").unwrap(), complex_name: "Федеральный закон от 22.06.2024 № 160-ФЗ\n \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), pages_count: 4, curr_page: 0, pdf_file_length: 169841, jd_reg_number: None, jd_reg_date: None, title: "Федеральный закон от 22.06.2024 № 160-ФЗ<br /> \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), view_date: Date::parse("2024-06-22T00:00:00").unwrap(), id: "118e71c6-7e90-495c-9afb-56b38edea17a".to_owned() };
        let mut d: PublicationDocumentCard = u[0].clone();
        logger::info!("{:?}", &d);
        

                let mut page_number = 1;
                while let Ok(p) = d.next_image().await
                {
                    if let Some(page) = p
                    {
                        let _ = std::fs::write([&d.eo_number, "_", &page_number.to_string(), ".png"].concat(), page);
                        page_number += 1;
                    }
                    else
                    {
                        break;
                    }
                    logger::info!("листаем старницы: {}", d.curr_page);
                }
            
    }
    #[tokio::test]
    async fn test_get_pdf()
    {
        StructLogger::new_default();
        //let u = PublicationApi::get_documents_list_from_date(Date::parse("01.04.2024").unwrap()).await.unwrap();
        let mut d = PublicationDocumentCard { eo_number: "0001202406220019".to_owned(), has_svg: false, zip_file_length: None, publish_date_short:  Date::parse("2024-06-22T00:00:00").unwrap(), complex_name: "Федеральный закон от 22.06.2024 № 160-ФЗ\n \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), pages_count: 4, curr_page: 0, pdf_file_length: 169841, jd_reg_number: None, jd_reg_date: None, title: "Федеральный закон от 22.06.2024 № 160-ФЗ<br /> \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), view_date: Date::parse("2024-06-22T00:00:00").unwrap(), id: "118e71c6-7e90-495c-9afb-56b38edea17a".to_owned() };
        //let mut d = u[0].clone();
        let pdf = PublicationApi::get_pdf_by_eo_number(&d.eo_number).await.unwrap();
        std::fs::write([&d.eo_number, ".pdf"].concat(),  pdf);
    }
    #[tokio::test]
    async fn test_get_extended_card()
    {
        StructLogger::new_default();
        let u = PublicationApi::get_document_extended_card("2600202407050002").await.unwrap();
        assert_eq!(u.publish_date_short.format(utilites::DateFormat::DotDate), "05.07.2024");
    }

    #[tokio::test]
    async fn test_get_organs_and_types()
    {
        StructLogger::new_default();
        let types  = PublicationApi::get_documents_types().await.unwrap();
        let organs  = PublicationApi::get_signatory_authorites().await.unwrap();
        logger::debug!("{} {:?} ----- {} {:?}", types.len(), types.first().unwrap(), organs.len(), organs.first().unwrap());
    }

    #[tokio::test]
    async fn test_get_documents_types_by_signatory_authority()
    {
        StructLogger::new_default();
        let types  = PublicationApi::get_documents_types_by_signatory_authority("8005d8c9-4b6d-48d3-861a-2a37e69fccb3").await.unwrap();
        logger::debug!("{:?}",  types);
    }
    #[tokio::test]
    async fn test_one_year_request()
    {
        StructLogger::new_default();
        let u = PublicationApi::get_documents_for_period(
            &Date::parse("01.01.2024").unwrap(),
            &Date::parse("13.12.2024").unwrap(),
            &["0790e34b-784b-4372-884e-3282622a24bd".to_owned()],
            Some("225698f1-cfbc-4e42-9caa-32f9f7403211".to_owned()).as_ref(), None
        ).await.unwrap();
        logger::debug!("Найдено {} документов", u.len());
    }
    #[tokio::test]
    async fn test_params()
    {
        StructLogger::new_default();
        let u = PublicationApi::get_params(
            &Date::parse("01.01.2024").unwrap(),
            Date::parse("13.12.2024").as_ref(),
            &["2dddb344-d3e2-4785-a899-7aa12bd47b6f".to_owned()],
            Some("3b24703c-c62f-4027-99ac-9eee99180df5".to_owned()).as_ref(), None
        );
        
    }


    //http://publication.pravo.gov.ru/Documents/search?pageSize=30&index=1&SignatoryAuthorityId=3b24703c-c62f-4027-99ac-9eee99180df5&DocumentTypes=2dddb344-d3e2-4785-a899-7aa12bd47b6f&PublishDateSearchType=0&PublishDateFrom=01.01.2024&PublishDateTo=13.12.2024&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
}