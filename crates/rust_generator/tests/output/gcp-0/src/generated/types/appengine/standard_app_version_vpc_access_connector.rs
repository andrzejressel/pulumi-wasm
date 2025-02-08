#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StandardAppVersionVpcAccessConnector {
    /// The egress setting for the connector, controlling what traffic is diverted through it.
    #[builder(into, default)]
    #[serde(rename = "egressSetting")]
    pub r#egress_setting: Box<Option<String>>,
    /// Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
