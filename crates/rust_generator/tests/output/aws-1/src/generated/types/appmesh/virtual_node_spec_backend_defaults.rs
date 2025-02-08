#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualNodeSpecBackendDefaults {
    /// Default client policy for virtual service backends. See above for details.
    #[builder(into, default)]
    #[serde(rename = "clientPolicy")]
    pub r#client_policy: Box<Option<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicy>>,
}
