#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataQualityJobDefinitionNetworkConfig {
    /// Whether to encrypt all communications between the instances used for the monitoring jobs. Choose `true` to encrypt communications. Encryption provides greater security for distributed jobs, but the processing might take longer.
    #[builder(into, default)]
    #[serde(rename = "enableInterContainerTrafficEncryption")]
    pub r#enable_inter_container_traffic_encryption: Box<Option<bool>>,
    /// Whether to allow inbound and outbound network calls to and from the containers used for the monitoring job.
    #[builder(into, default)]
    #[serde(rename = "enableNetworkIsolation")]
    pub r#enable_network_isolation: Box<Option<bool>>,
    /// Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionNetworkConfigVpcConfig>>,
}
