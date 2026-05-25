use bytes::Bytes;
use utilites::Date;

use crate::{DocumentType, ExtendedPublicationDocumentCard, PublicationDocumentCard, SearchResult, SignatoryAuthority};

pub trait PublicationApiClient
{
    const BASE_URL: &'static str;
    const API_URL: &'static str;
    fn get_documents(&self,
        date_from: Option<&Date>,
        date_to: Option<&Date>,
        doc_types: &[String],
        signatory_authority: Option<&String>,
        page_number: Option<u32>,
        page_size: Option<u32>,
        sender: Option<tokio::sync::mpsc::Sender<u32>>) -> impl std::future::Future<Output = anyhow::Result<Vec<PublicationDocumentCard>>> + Send;

    fn search_documents(&self,
        publication_date: &Date,
        signatory_authority: &String,
        page_size: Option<u32>
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<PublicationDocumentCard>>> + Send;
    
    /// Получить PDF по номеру электронного опубликования
    /// загрузка pdf http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202308040071"
    fn get_pdf_by_eo_number(&self, eo_number: &str) -> impl std::future::Future<Output = anyhow::Result<Bytes>> + Send;
    ///получение картинки
    /// http://publication.pravo.gov.ru/GetImage?documentId=dbf8d1c9-ed98-46ae-8cfb-1f7eb0fa066e&pageNumber=1
    fn get_image_by_id(&self, id: &str, page: u32) -> impl std::future::Future<Output = anyhow::Result<Bytes>> + Send;
    /// получение первого документа (для оценки что у него за номер итд....)
    /// если документов не найдено возвращает None
    ///http://95.173.157.131/api/Documents?SignatoryAuthorityId=8d31525e-fafc-4590-8580-422f588d20c9&DocumentTypes=2dddb344-d3e2-4785-a899-7aa12bd47b6f&pageSize=10&index=1
    fn get_first_document(&self, sa: &str, doc_type: &str) -> impl std::future::Future<Output = anyhow::Result<Option<PublicationDocumentCard>>> + Send;
    /// подробная карточка документа 
    ///  http://publication.pravo.gov.ru/api/Document?eoNumber=0001202406220019
    fn get_document_by_eo_number(&self, eo_number: &str) -> impl std::future::Future<Output = anyhow::Result<PublicationDocumentCard>> + Send;
    /// Получить расширенную карточку документа по id
    fn get_extended_document_card(&self, id: &str) -> impl std::future::Future<Output = anyhow::Result<ExtendedPublicationDocumentCard>> + Send;
    /// Получить список органов подписи
    fn get_signatory_authorites(&self) -> impl std::future::Future<Output = anyhow::Result<Vec<SignatoryAuthority>>> + Send;
    /// Получить список типов документов
    fn get_documents_types(&self, signatory_authority: Option<&str>) -> impl std::future::Future<Output = anyhow::Result<Vec<DocumentType>>> + Send;
}