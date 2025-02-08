#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyNetwork {
    /// The id or fully qualified URL of the VPC network to forward queries to.
    /// This should be formatted like `projects/{project}/global/networks/{network}` or
    /// `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`
    #[builder(into)]
    #[serde(rename = "networkUrl")]
    pub r#network_url: Box<String>,
}
