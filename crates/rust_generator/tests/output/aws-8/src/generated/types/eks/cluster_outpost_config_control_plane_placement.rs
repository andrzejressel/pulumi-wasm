#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterOutpostConfigControlPlanePlacement {
    /// The name of the placement group for the Kubernetes control plane instances. This setting can't be changed after cluster creation.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<String>,
}
