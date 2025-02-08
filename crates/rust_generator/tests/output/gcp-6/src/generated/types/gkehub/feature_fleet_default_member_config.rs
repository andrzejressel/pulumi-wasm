#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureFleetDefaultMemberConfig {
    /// Config Management spec
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "configmanagement")]
    pub r#configmanagement: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagement>>,
    /// Service Mesh spec
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mesh")]
    pub r#mesh: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigMesh>>,
    /// Policy Controller spec
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "policycontroller")]
    pub r#policycontroller: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontroller>>,
}
