#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsPool {
    /// List of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://support.cloudflare.com/hc/en-us/articles/115000540888-Load-Balancing-Geographic-Regions).
    #[builder(into)]
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Box<Vec<String>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    #[builder(into)]
    #[serde(rename = "createdOn")]
    pub r#created_on: Box<String>,
    /// Brief description of the Load Balancer Pool intention.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Whether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// ID for this load balancer pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Latitude this pool is physically located at; used for proximity steering.
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: Box<f64>,
    /// Setting for controlling load shedding for this pool.
    #[builder(into)]
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Box<Vec<super::types::GetLoadBalancerPoolsPoolLoadShedding>>,
    /// Longitude this pool is physically located at; used for proximity steering.
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: Box<f64>,
    /// Minimum number of origins that must be healthy for this pool to serve traffic.
    #[builder(into)]
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: Box<i32>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    #[builder(into)]
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: Box<String>,
    /// ID of the Monitor to use for health checking origins within this pool.
    #[builder(into)]
    #[serde(rename = "monitor")]
    pub r#monitor: Box<String>,
    /// Short name (tag) for the pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Email address to send health status notifications to. Multiple emails are set as a comma delimited list.
    #[builder(into)]
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: Box<String>,
    /// The list of origins within this pool.
    #[builder(into)]
    #[serde(rename = "origins")]
    pub r#origins: Box<Vec<super::types::GetLoadBalancerPoolsPoolOrigin>>,
}
