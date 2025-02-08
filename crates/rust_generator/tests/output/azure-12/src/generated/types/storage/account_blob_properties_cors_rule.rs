#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountBlobPropertiesCorsRule {
    /// A list of headers that are allowed to be a part of the cross-origin request.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Vec<String>>,
    /// A list of HTTP methods that are allowed to be executed by the origin. Valid options are
    /// `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PUT` or `PATCH`.
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
    /// The number of seconds the client should cache a preflight response.
    #[builder(into)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Box<i32>,
}
