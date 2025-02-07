#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkVpcNetwork {
    /// (Output)
    /// The relative resource name of the service VPC network this VMware Engine network is attached to.
    /// For example: projects/123123/global/networks/my-network
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// VMware Engine network type.
    /// Possible values are: `LEGACY`, `STANDARD`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
