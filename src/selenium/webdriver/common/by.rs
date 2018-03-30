use std::string::ToString;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum By {
    ID,
    XPATH,
    LINK_TEXT,
    PARTIAL_LINK_TEXT,
    NAME,
    TAG_NAME,
    CLASS_NAME,
    CSS_SELECTOR,
}

impl ToString for By {
    fn to_string(&self) -> String {
        match *self {
            By::ID => "id",
            By::XPATH => "xpath",
            By::LINK_TEXT => "link text",
            By::PARTIAL_LINK_TEXT => "partial link text",
            By::NAME => "name",
            By::TAG_NAME => "tag name",
            By::CLASS_NAME => "class name",
            By::CSS_SELECTOR => "css selector",
        }.to_string()
    }
}
