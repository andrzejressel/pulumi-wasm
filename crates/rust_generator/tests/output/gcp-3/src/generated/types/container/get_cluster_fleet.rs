#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterFleet {
    /// Full resource name of the registered fleet membership of the cluster.
    #[builder(into)]
    #[serde(rename = "membership")]
    pub r#membership: Box<String>,
    /// Short name of the fleet membership, for example "member-1".
    #[builder(into)]
    #[serde(rename = "membershipId")]
    pub r#membership_id: Box<String>,
    /// Location of the fleet membership, for example "us-central1".
    #[builder(into)]
    #[serde(rename = "membershipLocation")]
    pub r#membership_location: Box<String>,
    /// Whether the cluster has been registered via the fleet API.
    #[builder(into)]
    #[serde(rename = "preRegistered")]
    pub r#pre_registered: Box<bool>,
    /// The project in which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
}
