#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionVpcAccessConnector {
    /// Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
