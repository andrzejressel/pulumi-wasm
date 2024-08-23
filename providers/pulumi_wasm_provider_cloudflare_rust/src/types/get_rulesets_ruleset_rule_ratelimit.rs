#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleRatelimit {
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}
