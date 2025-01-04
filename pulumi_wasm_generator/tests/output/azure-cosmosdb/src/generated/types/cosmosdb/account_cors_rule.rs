#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountCorsRule {
    /// A list of headers that are allowed to be a part of the cross-origin request.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Vec<String>>,
    /// A list of HTTP headers that are allowed to be executed by the origin. Valid options are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PUT` or `PATCH`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Vec<String>>,
    /// A list of origin domains that will be allowed by CORS.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Vec<String>>,
    /// A list of response headers that are exposed to CORS clients.
    #[builder(into)]
    #[serde(rename = "exposedHeaders")]
    pub r#exposed_headers: Box<Vec<String>>,
    /// The number of seconds the client should cache a preflight response. Possible values are between `1` and `2147483647`.
    #[builder(into, default)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Box<Option<i32>>,
}
