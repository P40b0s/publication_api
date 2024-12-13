use crate::PublicationDocumentCard;

use super::Extractor;

pub struct PrezidentUkaz<'a>
{
    cards: &'a [PublicationDocumentCard]
}
impl<'a> Extractor<'a> for PrezidentUkaz<'a>
{
    const NAME: &'static str = "Президент Российской Федерации";
    const SIGNATORY: &'static str = "225698f1-cfbc-4e42-9caa-32f9f7403211";
    const TYPE_ID: &'static str = "0790e34b-784b-4372-884e-3282622a24bd";
    fn from(cards: &'a [PublicationDocumentCard]) -> Self 
    {
        Self
        {
            cards
        }
    }
    fn get_numbers(&'a self) -> Vec<String> 
    {
        vec![]
    }
    fn get_skip_numbers(&'a self) -> Vec<String> 
    {
        vec![]
    }
}
