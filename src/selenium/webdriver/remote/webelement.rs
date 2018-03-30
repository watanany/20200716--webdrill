use serde_json::Value;
use selenium::common::{Error, Result};
use selenium::webdriver::common::By;
use super::command::Command;
use super::webdriver::WebDriver;

#[derive(Debug, Clone)]
pub struct WebElement {
    pub parent: WebDriver,
    pub id: String,
}

impl WebElement {
    pub fn new(parent: WebDriver, id: &str) -> WebElement {
        WebElement {
            parent: parent,
            id: id.to_owned(),
        }
    }
    pub fn execute(&self, driver_command: Command, params: &Value) -> Result<Value> {
        let mut params = params.clone();
        if let Some(ref mut map) = params.as_object_mut() {
            map.insert("id".to_owned(), Value::String(self.id.clone()));
        }
        self.parent.execute(driver_command, &params)
    }
    pub fn find_element(&self, q: By, value: &str) -> Result<WebElement> {
        let (q, value) = match q {
            By::ID => (By::CSS_SELECTOR, format!("[id=\"{}\"]", value)),
            By::TAG_NAME => (By::CSS_SELECTOR, value.to_owned()),
            By::CLASS_NAME => (By::CSS_SELECTOR, format!(".{}", value)),
            By::NAME => (By::CSS_SELECTOR, format!("[name=\"{}\"]", value)),
            _ => (q, value.to_owned()),
        };
        let json = self.execute(
            Command::FIND_CHILD_ELEMENT,
            &json!({
                "using": q.to_string(),
                "value": value,
            }),
        )?;
        match json["value"]["ELEMENT"].as_str() {
            Some(element) => Ok(WebElement::new(self.parent.clone(), element)),
            None => panic!("Server response doesn't include ELEMENT ID"),
        }
    }
}
