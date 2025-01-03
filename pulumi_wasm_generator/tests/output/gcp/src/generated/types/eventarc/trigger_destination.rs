#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerDestination {
    /// The Cloud Function resource name. Only Cloud Functions V2 is supported. Format projects/{project}/locations/{location}/functions/{function} This is a read-only field. [WARNING] Creating Cloud Functions V2 triggers is only supported via the Cloud Functions product. An error will be returned if the user sets this value.
    #[builder(into, default)]
    #[serde(rename = "cloudFunction")]
    pub r#cloud_function: Box<Option<String>>,
    /// Cloud Run fully-managed service that receives the events. The service should be running in the same project of the trigger.
    #[builder(into, default)]
    #[serde(rename = "cloudRunService")]
    pub r#cloud_run_service: Box<Option<super::super::types::eventarc::TriggerDestinationCloudRunService>>,
    /// A GKE service capable of receiving events. The service should be running in the same project as the trigger.
    #[builder(into, default)]
    #[serde(rename = "gke")]
    pub r#gke: Box<Option<super::super::types::eventarc::TriggerDestinationGke>>,
    /// An HTTP endpoint destination described by an URI.
    #[builder(into, default)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Box<Option<super::super::types::eventarc::TriggerDestinationHttpEndpoint>>,
    /// Optional. Network config is used to configure how Eventarc resolves and connect to a destination. This should only be used with HttpEndpoint destination type.
    #[builder(into, default)]
    #[serde(rename = "networkConfig")]
    pub r#network_config: Box<Option<super::super::types::eventarc::TriggerDestinationNetworkConfig>>,
    /// The resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: `projects/{project}/locations/{location}/workflows/{workflow}`
    #[builder(into, default)]
    #[serde(rename = "workflow")]
    pub r#workflow: Box<Option<String>>,
}
