#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetDefaultClusterConfigBinaryAuthorizationConfigPolicyBinding {
    /// The relative resource name of the binauthz platform policy to audit. GKE
    /// platform policies have the following format:
    /// `projects/{project_number}/platforms/gke/policies/{policy_id}`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
