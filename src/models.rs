use serde::Serialize;
use std::sync::Mutex;

// AppState will hold shared application state
pub struct AppState {
    pub counter: Mutex<i32>,
}

// Data structure for index page
#[derive(Serialize)]
pub struct IndexTemplateData {
    pub name: String,
    pub counter: i32,
}

// Data structure for about page
#[derive(Serialize)]
pub struct AboutTemplateData {
    pub time: String,
}
