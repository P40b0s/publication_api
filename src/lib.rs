mod deserialization;
mod document_type;
mod error;
mod models;
mod publication_api;
mod reqwest_client;
mod signatory_authority;
mod client;
mod logger;

pub use document_type::DocumentType;
//pub use error::PublicationApiError;
pub use models::{ExtendedPublicationDocumentCard, PublicationDocumentCard, SearchResult};
//pub use publication_api::PublicationApi;
pub use signatory_authority::SignatoryAuthority;
pub use client::PublicationApiClient;
pub use reqwest_client::ReqwestPublicationApiClient;