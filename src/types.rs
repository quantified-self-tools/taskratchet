use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "forum user")]
    forum_user: String,
    active: bool,
    email: String,
    #[serde(rename = "default deadline")]
    default_deadline: String,
    #[serde(rename = "last morning email")]
    last_morning_email: String,
    #[serde(rename = "token hash")]
    token_hash: String,
    stripe: String,
    name: String,
    location: String,
    id: u64,
    timezone: String,
    #[serde(rename = "task count")]
    task_count: u64,
    #[serde(rename = "lifetime stakes")]
    lifetime_stakes: u64,
    #[serde(rename = "average stakes")]
    average_stakes: AverageStakes,
    created: String,
    #[serde(rename = "days active")]
    days_active: u64,
    #[serde(rename = "tasks per day")]
    tasks_per_day: u64,
    #[serde(rename = "tasks per month")]
    tasks_per_month: u64,
    #[serde(rename = "failed task count")]
    failed_task_count: u64,
    #[serde(rename = "complete count")]
    complete_count: u64,
    #[serde(rename = "paid stakes")]
    paid_stakes: u64,
    row_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStakes {
    #[serde(rename = "specialValue")]
    special_value: String,
}
