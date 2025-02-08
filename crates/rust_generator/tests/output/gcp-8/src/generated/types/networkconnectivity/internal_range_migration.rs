#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InternalRangeMigration {
    /// Resource path as an URI of the source resource, for example a subnet.
    /// The project for the source resource should match the project for the
    /// InternalRange.
    /// An example /projects/{project}/regions/{region}/subnetworks/{subnet}
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// Resource path of the target resource. The target project can be
    /// different, as in the cases when migrating to peer networks. The resource
    /// may not exist yet.
    /// For example /projects/{project}/regions/{region}/subnetworks/{subnet}
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}
