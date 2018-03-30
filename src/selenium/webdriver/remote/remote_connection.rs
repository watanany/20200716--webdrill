use std::str::FromStr;
use std::time::Duration;
use regex::{Captures, Regex};
use serde_json::Value;
use reqwest::{self, Method};
use selenium::common::{Error, Result, Status};
use super::command::Command;
use super::errorhandler;

#[derive(Debug, Clone)]
pub struct RemoteConnection {
    pub url: String,
}

impl RemoteConnection {
    pub fn new(remote_server_addr: &str) -> RemoteConnection {
        RemoteConnection {
            url: remote_server_addr.to_owned(),
        }
    }
    pub fn execute(&self, driver_command: Command, params: &Value) -> Result<Value> {
        let mut params = params.clone();
        let command_info = Self::command(driver_command);
        let path = Self::replace_path(command_info.1, &params);
        let url = format!("{}{}", self.url, path);
        if let Some(ref mut map) = params.as_object_mut() {
            map.remove("sessionId");
        }
        self.request(command_info.0, &url, &params.to_string())
    }

    fn command(driver_command: Command) -> (Method, &'static str) {
        match driver_command {
            Command::STATUS => (Method::Get, "/status"),
            Command::NEW_SESSION => (Method::Post, "/session"),
            Command::GET_ALL_SESSIONS => (Method::Get, "/sessions"),
            Command::QUIT => (Method::Delete, "/session/$sessionId"),
            Command::GET_CURRENT_WINDOW_HANDLE => {
                (Method::Get, "/session/$sessionId/window_handle")
            }
            Command::W3C_GET_CURRENT_WINDOW_HANDLE => (Method::Get, "/session/$sessionId/window"),
            Command::GET_WINDOW_HANDLES => (Method::Get, "/session/$sessionId/window_handles"),
            Command::W3C_GET_WINDOW_HANDLES => (Method::Get, "/session/$sessionId/window/handles"),
            Command::GET => (Method::Post, "/session/$sessionId/url"),
            Command::GO_FORWARD => (Method::Post, "/session/$sessionId/forward"),
            Command::GO_BACK => (Method::Post, "/session/$sessionId/back"),
            Command::REFRESH => (Method::Post, "/session/$sessionId/refresh"),
            Command::EXECUTE_SCRIPT => (Method::Post, "/session/$sessionId/execute"),
            Command::W3C_EXECUTE_SCRIPT => (Method::Post, "/session/$sessionId/execute/sync"),
            Command::W3C_EXECUTE_SCRIPT_ASYNC => {
                (Method::Post, "/session/$sessionId/execute/async")
            }
            Command::GET_CURRENT_URL => (Method::Get, "/session/$sessionId/url"),
            Command::GET_TITLE => (Method::Get, "/session/$sessionId/title"),
            Command::GET_PAGE_SOURCE => (Method::Get, "/session/$sessionId/source"),
            Command::SCREENSHOT => (Method::Get, "/session/$sessionId/screenshot"),
            Command::ELEMENT_SCREENSHOT => {
                (Method::Get, "/session/$sessionId/element/$id/screenshot")
            }
            Command::FIND_ELEMENT => (Method::Post, "/session/$sessionId/element"),
            Command::FIND_ELEMENTS => (Method::Post, "/session/$sessionId/elements"),
            Command::W3C_GET_ACTIVE_ELEMENT => (Method::Get, "/session/$sessionId/element/active"),
            Command::GET_ACTIVE_ELEMENT => (Method::Post, "/session/$sessionId/element/active"),
            Command::FIND_CHILD_ELEMENT => {
                (Method::Post, "/session/$sessionId/element/$id/element")
            }
            Command::FIND_CHILD_ELEMENTS => {
                (Method::Post, "/session/$sessionId/element/$id/elements")
            }
            Command::CLICK_ELEMENT => (Method::Post, "/session/$sessionId/element/$id/click"),
            Command::CLEAR_ELEMENT => (Method::Post, "/session/$sessionId/element/$id/clear"),
            Command::SUBMIT_ELEMENT => (Method::Post, "/session/$sessionId/element/$id/submit"),
            Command::GET_ELEMENT_TEXT => (Method::Get, "/session/$sessionId/element/$id/text"),
            Command::SEND_KEYS_TO_ELEMENT => {
                (Method::Post, "/session/$sessionId/element/$id/value")
            }
            Command::SEND_KEYS_TO_ACTIVE_ELEMENT => (Method::Post, "/session/$sessionId/keys"),
            Command::UPLOAD_FILE => (Method::Post, "/session/$sessionId/file"),
            Command::GET_ELEMENT_VALUE => (Method::Get, "/session/$sessionId/element/$id/value"),
            Command::GET_ELEMENT_TAG_NAME => (Method::Get, "/session/$sessionId/element/$id/name"),
            Command::IS_ELEMENT_SELECTED => {
                (Method::Get, "/session/$sessionId/element/$id/selected")
            }
            Command::SET_ELEMENT_SELECTED => {
                (Method::Post, "/session/$sessionId/element/$id/selected")
            }
            Command::IS_ELEMENT_ENABLED => (Method::Get, "/session/$sessionId/element/$id/enabled"),
            Command::IS_ELEMENT_DISPLAYED => {
                (Method::Get, "/session/$sessionId/element/$id/displayed")
            }
            Command::GET_ELEMENT_LOCATION => {
                (Method::Get, "/session/$sessionId/element/$id/location")
            }
            Command::GET_ELEMENT_LOCATION_ONCE_SCROLLED_INTO_VIEW => (
                Method::Get,
                "/session/$sessionId/element/$id/location_in_view",
            ),
            Command::GET_ELEMENT_SIZE => (Method::Get, "/session/$sessionId/element/$id/size"),
            Command::GET_ELEMENT_RECT => (Method::Get, "/session/$sessionId/element/$id/rect"),
            Command::GET_ELEMENT_ATTRIBUTE => (
                Method::Get,
                "/session/$sessionId/element/$id/attribute/$name",
            ),
            Command::GET_ELEMENT_PROPERTY => (
                Method::Get,
                "/session/$sessionId/element/$id/property/$name",
            ),
            Command::ELEMENT_EQUALS => {
                (Method::Get, "/session/$sessionId/element/$id/equals/$other")
            }
            Command::GET_ALL_COOKIES => (Method::Get, "/session/$sessionId/cookie"),
            Command::ADD_COOKIE => (Method::Post, "/session/$sessionId/cookie"),
            Command::DELETE_ALL_COOKIES => (Method::Delete, "/session/$sessionId/cookie"),
            Command::DELETE_COOKIE => (Method::Delete, "/session/$sessionId/cookie/$name"),
            Command::SWITCH_TO_FRAME => (Method::Post, "/session/$sessionId/frame"),
            Command::SWITCH_TO_PARENT_FRAME => (Method::Post, "/session/$sessionId/frame/parent"),
            Command::SWITCH_TO_WINDOW => (Method::Post, "/session/$sessionId/window"),
            Command::CLOSE => (Method::Delete, "/session/$sessionId/window"),
            Command::GET_ELEMENT_VALUE_OF_CSS_PROPERTY => (
                Method::Get,
                "/session/$sessionId/element/$id/css/$propertyName",
            ),
            Command::IMPLICIT_WAIT => (Method::Post, "/session/$sessionId/timeouts/implicit_wait"),
            Command::EXECUTE_ASYNC_SCRIPT => (Method::Post, "/session/$sessionId/execute_async"),
            Command::SET_SCRIPT_TIMEOUT => {
                (Method::Post, "/session/$sessionId/timeouts/async_script")
            }
            Command::SET_TIMEOUTS => (Method::Post, "/session/$sessionId/timeouts"),
            Command::DISMISS_ALERT => (Method::Post, "/session/$sessionId/dismiss_alert"),
            Command::W3C_DISMISS_ALERT => (Method::Post, "/session/$sessionId/alert/dismiss"),
            Command::ACCEPT_ALERT => (Method::Post, "/session/$sessionId/accept_alert"),
            Command::W3C_ACCEPT_ALERT => (Method::Post, "/session/$sessionId/alert/accept"),
            Command::SET_ALERT_VALUE => (Method::Post, "/session/$sessionId/alert_text"),
            Command::W3C_SET_ALERT_VALUE => (Method::Post, "/session/$sessionId/alert/text"),
            Command::GET_ALERT_TEXT => (Method::Get, "/session/$sessionId/alert_text"),
            Command::W3C_GET_ALERT_TEXT => (Method::Get, "/session/$sessionId/alert/text"),
            Command::SET_ALERT_CREDENTIALS => {
                (Method::Post, "/session/$sessionId/alert/credentials")
            }
            Command::CLICK => (Method::Post, "/session/$sessionId/click"),
            Command::W3C_ACTIONS => (Method::Post, "/session/$sessionId/actions"),
            Command::W3C_CLEAR_ACTIONS => (Method::Delete, "/session/$sessionId/actions"),
            Command::DOUBLE_CLICK => (Method::Post, "/session/$sessionId/doubleclick"),
            Command::MOUSE_DOWN => (Method::Post, "/session/$sessionId/buttondown"),
            Command::MOUSE_UP => (Method::Post, "/session/$sessionId/buttonup"),
            Command::MOVE_TO => (Method::Post, "/session/$sessionId/moveto"),
            Command::GET_WINDOW_SIZE => {
                (Method::Get, "/session/$sessionId/window/$windowHandle/size")
            }
            Command::SET_WINDOW_SIZE => (
                Method::Post,
                "/session/$sessionId/window/$windowHandle/size",
            ),
            Command::GET_WINDOW_POSITION => (
                Method::Get,
                "/session/$sessionId/window/$windowHandle/position",
            ),
            Command::SET_WINDOW_POSITION => (
                Method::Post,
                "/session/$sessionId/window/$windowHandle/position",
            ),
            Command::SET_WINDOW_RECT => (Method::Post, "/session/$sessionId/window/rect"),
            Command::GET_WINDOW_RECT => (Method::Get, "/session/$sessionId/window/rect"),
            Command::MAXIMIZE_WINDOW => (
                Method::Post,
                "/session/$sessionId/window/$windowHandle/maximize",
            ),
            Command::W3C_MAXIMIZE_WINDOW => (Method::Post, "/session/$sessionId/window/maximize"),
            Command::SET_SCREEN_ORIENTATION => (Method::Post, "/session/$sessionId/orientation"),
            Command::GET_SCREEN_ORIENTATION => (Method::Get, "/session/$sessionId/orientation"),
            Command::SINGLE_TAP => (Method::Post, "/session/$sessionId/touch/click"),
            Command::TOUCH_DOWN => (Method::Post, "/session/$sessionId/touch/down"),
            Command::TOUCH_UP => (Method::Post, "/session/$sessionId/touch/up"),
            Command::TOUCH_MOVE => (Method::Post, "/session/$sessionId/touch/move"),
            Command::TOUCH_SCROLL => (Method::Post, "/session/$sessionId/touch/scroll"),
            Command::DOUBLE_TAP => (Method::Post, "/session/$sessionId/touch/doubleclick"),
            Command::LONG_PRESS => (Method::Post, "/session/$sessionId/touch/longclick"),
            Command::FLICK => (Method::Post, "/session/$sessionId/touch/flick"),
            Command::EXECUTE_SQL => (Method::Post, "/session/$sessionId/execute_sql"),
            Command::GET_LOCATION => (Method::Get, "/session/$sessionId/location"),
            Command::SET_LOCATION => (Method::Post, "/session/$sessionId/location"),
            Command::GET_APP_CACHE => (Method::Get, "/session/$sessionId/application_cache"),
            Command::GET_APP_CACHE_STATUS => {
                (Method::Get, "/session/$sessionId/application_cache/status")
            }
            Command::CLEAR_APP_CACHE => (
                Method::Delete,
                "/session/$sessionId/application_cache/clear",
            ),
            Command::GET_NETWORK_CONNECTION => {
                (Method::Get, "/session/$sessionId/network_connection")
            }
            Command::SET_NETWORK_CONNECTION => {
                (Method::Post, "/session/$sessionId/network_connection")
            }
            Command::GET_LOCAL_STORAGE_ITEM => {
                (Method::Get, "/session/$sessionId/local_storage/key/$key")
            }
            Command::REMOVE_LOCAL_STORAGE_ITEM => {
                (Method::Delete, "/session/$sessionId/local_storage/key/$key")
            }
            Command::GET_LOCAL_STORAGE_KEYS => (Method::Get, "/session/$sessionId/local_storage"),
            Command::SET_LOCAL_STORAGE_ITEM => (Method::Post, "/session/$sessionId/local_storage"),
            Command::CLEAR_LOCAL_STORAGE => (Method::Delete, "/session/$sessionId/local_storage"),
            Command::GET_LOCAL_STORAGE_SIZE => {
                (Method::Get, "/session/$sessionId/local_storage/size")
            }
            Command::GET_SESSION_STORAGE_ITEM => {
                (Method::Get, "/session/$sessionId/session_storage/key/$key")
            }
            Command::REMOVE_SESSION_STORAGE_ITEM => (
                Method::Delete,
                "/session/$sessionId/session_storage/key/$key",
            ),
            Command::GET_SESSION_STORAGE_KEYS => {
                (Method::Get, "/session/$sessionId/session_storage")
            }
            Command::SET_SESSION_STORAGE_ITEM => {
                (Method::Post, "/session/$sessionId/session_storage")
            }
            Command::CLEAR_SESSION_STORAGE => {
                (Method::Delete, "/session/$sessionId/session_storage")
            }
            Command::GET_SESSION_STORAGE_SIZE => {
                (Method::Get, "/session/$sessionId/session_storage/size")
            }
            Command::GET_LOG => (Method::Post, "/session/$sessionId/log"),
            Command::GET_AVAILABLE_LOG_TYPES => (Method::Get, "/session/$sessionId/log/types"),
            Command::CURRENT_CONTEXT_HANDLE => (Method::Get, "/session/$sessionId/context"),
            Command::CONTEXT_HANDLES => (Method::Get, "/session/$sessionId/contexts"),
            Command::SWITCH_TO_CONTEXT => (Method::Post, "/session/$sessionId/context"),
            Command::FULLSCREEN_WINDOW => (Method::Post, "/session/$sessionId/window/fullscreen"),
            Command::MINIMIZE_WINDOW => (Method::Post, "/session/$sessionId/window/minimize"),
            _ => panic!(format!("Illegal Driver Command: {:?}", driver_command)),
        }
    }
    fn replace_path(path: &str, params: &Value) -> String {
        let pats = [
            Regex::new(r"\$(\w+)").expect("Invalid regular expression"),
            Regex::new(r"\$\{(\w+)\}").expect("Invalid regular expression"),
        ];
        pats.iter().fold(path.to_owned(), |acc: String, p: &Regex| {
            p.replace_all(&acc, |caps: &Captures| {
                let key = caps[1].to_owned();
                params[key].as_str().unwrap_or("").to_owned()
            }).to_string()
        })
    }
    fn request(&self, method: Method, url: &str, body: &str) -> Result<Value> {
        let client = reqwest::Client::new();
        let mut req = match method {
            Method::Get => client.get(url),
            Method::Post => client.post(url),
            Method::Delete => client.delete(url),
            _ => panic!(format!("Unknown HTTP Method: {}", method)),
        };
        let mut res = req.body(body.to_owned()).send()?;
        let status = res.status();

        if status.is_success() {
            Ok(res.json()?)
        } else {
            panic!("Server response results in error")
        }
    }
}
