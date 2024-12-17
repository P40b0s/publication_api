mod publication_api;
mod deserialization;
mod models;
mod error;
mod document_type;
mod signatory_authority;


use utilites::Url;
pub use publication_api::PublicationApi;
pub use document_type::DocumentType;
pub use signatory_authority::SignatoryAuthority;
pub use models::{PublicationDocumentCard, SearchResult, ExtendedPublicationDocumentCard};
pub use error::PublicationApiError;


#[cfg(test)]
mod tests
{

    use logger::StructLogger;
    use utilites::{Date, Url};

    use super::{SearchResult, error::PublicationApiError};

    #[tokio::test]
    async fn test_api()
    {
        StructLogger::new_default();
       
    }

    #[tokio::test]
    async fn test_api_1()
    {
        StructLogger::new_default();
       
    }
}