#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPool {
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Box<Vec<String>>,
    #[serde(rename = "createdOn")]
    pub r#created_on: Box<String>,
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "latitude")]
    pub r#latitude: Box<f64>,
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Box<Vec<crate::types::GetLoadBalancerPoolsPoolLoadShedding>>,
    #[serde(rename = "longitude")]
    pub r#longitude: Box<f64>,
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: Box<i32>,
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: Box<String>,
    #[serde(rename = "monitor")]
    pub r#monitor: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: Box<String>,
    #[serde(rename = "origins")]
    pub r#origins: Box<Vec<crate::types::GetLoadBalancerPoolsPoolOrigin>>,
}
