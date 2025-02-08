#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BareMetalAdminClusterFleet {
    /// (Output)
    /// The name of the managed Hub Membership resource associated to this cluster.
    /// Membership names are formatted as
    /// `projects/<project-number>/locations/<location>/memberships/<cluster-id>`.
    #[builder(into, default)]
    #[serde(rename = "membership")]
    pub r#membership: Box<Option<String>>,
}
