mod prezident_ukaz;

use std::borrow::Cow;

use utilites::Date;

use crate::PublicationDocumentCard;

pub trait Parser
{
    const NAME: &str;
    const SIGNATORY: &str;
    const TYPE_ID: &str;
    async fn document_is_exists();
    async fn get_docment() -> Self;
    fn get_publication_date(&self) -> Date;
    fn get_name<'a>(&'a self) -> Cow<'a, str>;
}

pub trait Extractor<'a>
{
    const NAME: &'static str;
    const SIGNATORY: &'static str;
    const TYPE_ID: &'static str;
    fn from(cards: &'a [PublicationDocumentCard]) -> Self;
    fn get_numbers(&'a self) -> Vec<String>;
    fn get_skip_numbers(&'a self) -> Vec<String>;
}


#[cfg(test)]
mod tests
{
    #[test]
    fn test_prez()
    {

    }
}