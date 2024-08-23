#[derive(serde::Serialize)]
pub struct ApiTokenConditionRequestIp {
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
