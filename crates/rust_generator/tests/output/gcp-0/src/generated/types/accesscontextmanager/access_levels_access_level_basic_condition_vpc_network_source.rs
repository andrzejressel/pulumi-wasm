#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessLevelsAccessLevelBasicConditionVpcNetworkSource {
    /// Sub networks within a VPC network.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcSubnetwork")]
    pub r#vpc_subnetwork: Box<Option<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicConditionVpcNetworkSourceVpcSubnetwork>>,
}
