#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackendServiceLocalityLbPolicyCustomPolicy {
    /// An optional, arbitrary JSON object with configuration data, understood
    /// by a locally installed custom policy implementation.
    #[builder(into, default)]
    #[serde(rename = "data")]
    pub r#data: Box<Option<String>>,
    /// Identifies the custom policy.
    /// The value should match the type the custom implementation is registered
    /// with on the gRPC clients. It should follow protocol buffer
    /// message naming conventions and include the full path (e.g.
    /// myorg.CustomLbPolicy). The maximum length is 256 characters.
    /// Note that specifying the same custom policy more than once for a
    /// backend is not a valid configuration and will be rejected.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
