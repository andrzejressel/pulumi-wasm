#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitMatchRequest {
    /// HTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    /// HTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "schemes")]
    pub r#schemes: Box<Option<Vec<String>>>,
    /// The URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Box<Option<String>>,
}
