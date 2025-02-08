#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceTypeOfferingsBrokerInstanceOption {
    /// List of available AZs. See Availability Zones. below
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Vec<super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone>>,
    /// Filter response by engine type.
    #[builder(into)]
    #[serde(rename = "engineType")]
    pub r#engine_type: Box<String>,
    /// Filter response by host instance type.
    #[builder(into)]
    #[serde(rename = "hostInstanceType")]
    pub r#host_instance_type: Box<String>,
    /// Filter response by storage type.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<String>,
    /// The list of supported deployment modes.
    #[builder(into)]
    #[serde(rename = "supportedDeploymentModes")]
    pub r#supported_deployment_modes: Box<Vec<String>>,
    /// The list of supported engine versions.
    #[builder(into)]
    #[serde(rename = "supportedEngineVersions")]
    pub r#supported_engine_versions: Box<Vec<String>>,
}
