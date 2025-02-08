#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpecMulticlusteringress {
    /// Fully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: `projects/foo-proj/locations/global/memberships/bar`
    #[builder(into)]
    #[serde(rename = "configMembership")]
    pub r#config_membership: Box<String>,
}
