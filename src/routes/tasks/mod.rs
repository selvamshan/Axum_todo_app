use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod custom_extractor;
pub mod get_tasks;
pub mod update_tasks;



#[derive(Deserialize)]
pub struct RequestTask {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>
}


#[derive(Serialize)]
pub struct ResponseTask {
    pub id : i32,
    pub title: String,
    pub desctription: Option<String>,
    pub priority: Option<String>,
    completed_at: Option<String>,
    user_id: Option<i32>
}

#[derive(Serialize)]
pub struct ResponseDataTask {
    data: ResponseTask
}

#[derive(Serialize)]
pub struct ResponseDataTasks {
    data: Vec<ResponseTask>
}