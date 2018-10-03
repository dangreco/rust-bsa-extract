use bsa::*;

pub struct BSAParser {
    data: Vec<u8>
}

impl BSAParser {

    pub fn new(data: Vec<u8>) -> BSAParser
    {
        return BSAParser {
            data
        }
    }

    pub fn parse() -> Option<BSA>
    {

        return None;
    }

}
