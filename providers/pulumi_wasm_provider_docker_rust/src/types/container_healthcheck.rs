#[derive(serde::Serialize)]
pub struct ContainerHealthcheck {
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
