use serde_json::{Map, Value};
use selenium::common::Result;
use selenium::webdriver::common::By;
use super::remote_connection::RemoteConnection;
use super::command::Command;
use super::webelement::WebElement;

#[derive(Debug, Clone)]
pub struct WebDriver {
    pub command_executor: RemoteConnection,
    pub session_id: String,
}

impl WebDriver {
    pub fn new(remote_server_addr: &str) -> Result<Self> {
        let command_executor = RemoteConnection::new(remote_server_addr);
        let json = command_executor.execute(
            Command::NEW_SESSION,
            &json!({
                "desiredCapabilities": {"browserName": "chrome"},
            }),
        )?;
        match json["sessionId"].as_str() {
            Some(session_id) => Ok(Self {
                command_executor: command_executor,
                session_id: session_id.to_owned(),
            }),
            None => panic!("Server response doesn't include Session ID"),
        }
    }
    pub fn execute(&self, driver_command: Command, params: &Value) -> Result<Value> {
        let mut params = params.clone();
        if let Some(ref mut map) = params.as_object_mut() {
            map.insert(
                "sessionId".to_owned(),
                Value::String(self.session_id.clone()),
            );
        }
        self.command_executor.execute(driver_command, &params)
    }
    pub fn get(&self, url: &str) -> Result<Value> {
        self.execute(Command::GET, &json!({ "url": url }))
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
            Command::FIND_ELEMENT,
            &json!({
                "using": q.to_string(),
                "value": value,
            }),
        )?;
        match json["value"]["ELEMENT"].as_str() {
            Some(element) => Ok(WebElement::new(self.clone(), element)),
            None => panic!("Server response doesn't include ELEMENT ID"),
        }
    }
}

//pub struct WebDriverBuilder {
//    pub command_executor: RemoteConnection,
//    pub session_id: String,
//}
//
//impl WebDriverBuilder {
//    pub fn new(remote_server_addr: &str) -> Self {
//        let command_executor = RemoteConnection::new(remote_server_addr);
//        Self {
//            command_executor: command_executor,
//            session_id: "".to_owned(),
//        }
//    }
//    pub fn start_session(&mut self, desired_capabilities: &Value) -> Result<&mut Self> {
//        let mut map = Map::new();
//        map.insert(
//            "desiredCapabilities".to_owned(),
//            desired_capabilities.clone(),
//        );
//        let json = self.command_executor
//            .execute(Command::NEW_SESSION, &Value::Object(map))?;
//        match json["sessionId"].as_str() {
//            Some(session_id) => {
//                self.session_id = session_id.to_owned();
//                Ok(self)
//            }
//            None => Err(Error::NotImplemented),
//        }
//    }
//    pub fn build(self) -> WebDriver {
//        WebDriver {
//            command_executor: self.command_executor,
//            session_id: self.session_id,
//        }
//    }
//}
