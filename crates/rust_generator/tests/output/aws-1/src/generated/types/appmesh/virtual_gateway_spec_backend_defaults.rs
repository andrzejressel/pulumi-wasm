#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecBackendDefaults {
    /// Default client policy for virtual gateway backends.
    #[builder(into, default)]
    #[serde(rename = "clientPolicy")]
    pub r#client_policy: Box<Option<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicy>>,
}
