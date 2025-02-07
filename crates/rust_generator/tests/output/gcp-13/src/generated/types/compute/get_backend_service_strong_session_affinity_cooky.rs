#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendServiceStrongSessionAffinityCooky {
    /// The name of the Backend Service.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Path to set for the cookie.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Lifetime of the cookie.
    #[builder(into)]
    #[serde(rename = "ttls")]
    pub r#ttls: Box<Vec<super::super::types::compute::GetBackendServiceStrongSessionAffinityCookyTtl>>,
}
