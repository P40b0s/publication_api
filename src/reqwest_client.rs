use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use utilites::Date;
use tracing::debug;

use crate::{PublicationDocumentCard, SearchResult, SignatoryAuthority, client::PublicationApiClient};

pub struct ReqwestPublicationApiClient
{
    client: ClientWithMiddleware,
}
impl ReqwestPublicationApiClient
{
    pub fn new() -> Self
    {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(8);
        let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();
        Self
        {
            client: client,
        }
    }
    fn apply_params(
        url: &mut reqwest::Url,
        date_from: Option<&Date>,
        date_to: Option<&Date>,
        doc_types: &[String],
        signatory_authority: Option<&[String]>,
        page_size: Option<u32>,
        page_number: Option<u32>)
    {
        for dt in doc_types
        {
            url.query_pairs_mut().append_pair("DocumentTypes", dt);
        }
        if let Some(sa) = signatory_authority
        {
            for sa in sa
            {
                url.query_pairs_mut().append_pair("SignatoryAuthorityId", sa);
            }
        }
        if let Some(df) = date_from
        {
            url.query_pairs_mut().append_pair("DocumentDateFrom", &df.format(utilites::DateFormat::DotDate));
        }
        if let Some(dt) = date_to
        {
            url.query_pairs_mut().append_pair("DocumentDateTo", &dt.format(utilites::DateFormat::DotDate));
        }
        if let Some(ps) = page_size
        {
            url.query_pairs_mut().append_pair("PageSize", &ps.to_string());
        }
        else 
        {
            url.query_pairs_mut().append_pair("PageSize", "200");
        }
      
        if let Some(pn) = page_number
        {
            url.query_pairs_mut().append_pair("index", &pn.to_string());
        }
    }
}

impl PublicationApiClient for ReqwestPublicationApiClient 
{
    const BASE_URL: &'static str = "http://publication.pravo.gov.ru/";
    const API_URL: &'static str = "http://publication.pravo.gov.ru/api/";
    async fn get_documents(&self,
        date_from: Option<&Date>,
        date_to: Option<&Date>,
        doc_types: &[String],
        signatory_authority: Option<&[String]>,
        page_number: Option<u32>,
        page_size: Option<u32>,
        sender: Option<tokio::sync::mpsc::Sender<u32>>) -> anyhow::Result<Vec<PublicationDocumentCard>>
    {
        let url = [Self::API_URL, "Documents?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        Self::apply_params(&mut url, date_from, date_to, doc_types, signatory_authority, page_size, page_number);
        debug!("URL: {}", url);
        let body: SearchResult = self.client
            .get(url.clone())
            .send()
            .await?
            .json()
            .await?;
        //let resp: SearchResult = serde_json::from_slice(&body)?;
        let mut result_vec = Vec::with_capacity(body.items_total_count as usize);
        result_vec.extend(body.items);
        let total = if body.pages_total_count == 0
        {
            1
        }
        else
        {
            body.pages_total_count
        };
        let percentage_mul = 100 / total;
        if let Some(c) = sender.as_ref()
        {
            let _ = c.send(percentage_mul * 1).await;
        }
        for page in (body.current_page+1)..=total
        {
            url.query_pairs_mut().clear();
            Self::apply_params(&mut url, date_from, date_to, doc_types, signatory_authority, None, page_number);
            let body: SearchResult = self.client
                .get(url.clone())
                .send()
                .await?
                .json()
                .await?;
            result_vec.extend(body.items);
            if let Some(c) = sender.as_ref()
            {
                let _ = c.send(percentage_mul * page).await;
            }
        }
        Ok(result_vec)
    }
    
    async fn get_pdf_by_eo_number(&self, eo_number: &str) -> anyhow::Result<bytes::Bytes> 
    {
        let url = [Self::BASE_URL, "file/pdf?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        url.query_pairs_mut().append_pair("eoNumber", eo_number);
        let bytes = self.client.get(url).send().await?.bytes().await?;
        Ok(bytes)

    }
    
    async fn get_image_by_id(&self, id: &str, page: u32) -> anyhow::Result<bytes::Bytes> 
    {
        let url = [Self::BASE_URL, "GetImage?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        url.query_pairs_mut().append_pair("documentId", id);
        url.query_pairs_mut().append_pair("pageNumber", &page.to_string());
        let bytes = self.client.get(url).send().await?.bytes().await?;
        Ok(bytes)
    }
    
    async fn get_first_document(&self, sa: &str, doc_type: &str) -> anyhow::Result<Option<PublicationDocumentCard>> 
    {
        let url = [Self::API_URL, "Documents?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        Self::apply_params(&mut url, None, None, &vec![doc_type.to_owned()], Some(&vec![sa.to_owned()]), Some(1), Some(1));
        let body: SearchResult = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(body.items.into_iter().next())   
    }
    
   async fn get_document_by_eo_number(&self, eo_number: &str) -> anyhow::Result<crate::PublicationDocumentCard> 
   {
        let url = [Self::API_URL, "Document?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        url.query_pairs_mut().append_pair("eoNumber", eo_number);
        let body: crate::ExtendedPublicationDocumentCard = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(crate::PublicationDocumentCard::from(body))
    }

    async fn get_extended_document_card(&self, id: &str) -> anyhow::Result<crate::ExtendedPublicationDocumentCard> 
    {
        let url = [Self::API_URL, "Document?"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        url.query_pairs_mut().append_pair("id", id);
        let body: crate::ExtendedPublicationDocumentCard = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(body)
    }
    
    async fn get_signatory_authorites(&self) -> anyhow::Result<Vec<crate::SignatoryAuthority>> 
    {
        let url = [Self::API_URL, "SignatoryAuthorities"].concat();
        let url = reqwest::Url::parse(&url)?;
        let body: Vec<SignatoryAuthority> = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(body)
    }
    async fn get_documents_types(&self, signatory_authority: Option<&str>) -> anyhow::Result<Vec<crate::DocumentType>> 
    {
        let url = [Self::API_URL, "DocumentTypes"].concat();
        let mut url = reqwest::Url::parse(&url)?;
        if let Some(sa) = signatory_authority
        {
            url.query_pairs_mut().append_pair("SignatoryAuthorityId", sa);
        }
         let body: Vec<crate::DocumentType> = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(body)
    }
    
}



mod tests
{
    use tracing::info;

use super::*;
    #[tokio::test]
    async fn test_get_documents()
    {
        crate::logger::init();
        let client = ReqwestPublicationApiClient::new();
        let date_from = Date::parse("01.01.2023").unwrap();
        let date_to = Date::parse("31.12.2023").unwrap();
        let doc_types = vec!["0790e34b-784b-4372-884e-3282622a24bd".to_owned()];
        let signatory_authority = Some(vec!["225698f1-cfbc-4e42-9caa-32f9f7403211".to_owned()]);
        let result = client.get_documents(Some(&date_from), Some(&date_to), &doc_types, signatory_authority.as_ref().map(|v| v.as_slice()), None, None, None).await.unwrap();
        info!("Total documents: {}", result.len());
        assert!(!result.is_empty());
    }

        #[tokio::test]
    async fn test_get_images()
    {
        crate::logger::init();
        let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(7);
        tokio::spawn(
            async move 
            {
                while let Some(p) = receiver.recv().await 
                {
                    println!("текущий процент выполнения: {}%", p);
                }
            });
        let client = ReqwestPublicationApiClient::new();
        let u = client.get_documents(Some(&Date::parse("01.04.2024").unwrap()), None, &["82a8bf1c-3bc7-47ed-827f-7affd43a7f27".to_owned()], None, None, None, Some(sender)).await.unwrap();
        //let mut d = PublicationDocumentCard { eo_number: "0001202406220019".to_owned(), has_svg: false, zip_file_length: None, publish_date_short:  Date::parse("2024-06-22T00:00:00").unwrap(), complex_name: "Федеральный закон от 22.06.2024 № 160-ФЗ\n \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), pages_count: 4, curr_page: 0, pdf_file_length: 169841, jd_reg_number: None, jd_reg_date: None, title: "Федеральный закон от 22.06.2024 № 160-ФЗ<br /> \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), view_date: Date::parse("2024-06-22T00:00:00").unwrap(), id: "118e71c6-7e90-495c-9afb-56b38edea17a".to_owned() };
        let mut d: PublicationDocumentCard = u[0].clone();
        info!("{:?}", &d);
        

                let mut page_number = 1;
                while let Ok(p) = d.next_image(&client).await
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
                    info!("листаем старницы: {}", d.curr_page);
                }
            
    }
    
    #[tokio::test]
    async fn test_get_pdf()
    {
        let d = PublicationDocumentCard { eo_number: "0001202406220019".to_owned(), has_svg: false, zip_file_length: None, publish_date_short:  Date::parse("2024-06-22T00:00:00").unwrap(), complex_name: "Федеральный закон от 22.06.2024 № 160-ФЗ\n \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), pages_count: 4, curr_page: 0, pdf_file_length: 169841, jd_reg_number: None, jd_reg_date: None, title: "Федеральный закон от 22.06.2024 № 160-ФЗ<br /> \"О внесении изменений в статью 19 Федерального закона \"О крестьянском (фермерском) хозяйстве\" и Федеральный закон \"О развитии сельского хозяйства\"".to_owned(), view_date: Date::parse("2024-06-22T00:00:00").unwrap(), id: "118e71c6-7e90-495c-9afb-56b38edea17a".to_owned(), signatory_authority_id: "".to_owned(), document_type_id: "".to_owned(), document_date: Date::parse("2024-06-22T00:00:00").unwrap(), number: "123".to_owned() };
        let pdf = ReqwestPublicationApiClient::new();
        let pdf_data = pdf.get_pdf_by_eo_number(&d.eo_number).await.unwrap();
        std::fs::write([&d.eo_number, ".pdf"].concat(), pdf_data).unwrap();
    }

    #[tokio::test]
    async fn test_get_signatory_authorities()
    {
        crate::logger::init();
        let client = ReqwestPublicationApiClient::new();
        let signatory_authorities = client.get_signatory_authorites().await.unwrap();
        std::fs::write("./api/signatory_authorities.json", serde_json::to_string(&signatory_authorities).unwrap()).unwrap();
        info!("{:?}", signatory_authorities);
    
    }
     #[tokio::test]
    async fn test_get_documents_types()
    {
        crate::logger::init();
        let client = ReqwestPublicationApiClient::new();
        let document_types = client.get_documents_types(None).await.unwrap();
        std::fs::write("./api/document_types.json", serde_json::to_string(&document_types).unwrap()).unwrap();
        info!("{:?}", document_types);
    
    }

}