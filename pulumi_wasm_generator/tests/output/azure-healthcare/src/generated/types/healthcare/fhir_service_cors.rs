#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FhirServiceCors {
    /// A set of headers to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Vec<String>>,
    /// The methods to be allowed via CORS. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PATCH` and `PUT`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Vec<String>>,
    /// A set of origins to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Vec<String>>,
    /// If credentials are allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "credentialsAllowed")]
    pub r#credentials_allowed: Box<Option<bool>>,
    /// The max age to be allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Box<Option<i32>>,
}