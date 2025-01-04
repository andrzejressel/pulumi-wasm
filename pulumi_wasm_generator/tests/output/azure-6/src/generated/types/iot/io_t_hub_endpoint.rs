#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IoTHubEndpoint {
    /// The type used to authenticate against the endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
    #[builder(into, default)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<Option<String>>,
    /// Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds. This attribute is applicable for endpoint type `AzureIotHub.StorageContainer`.
    #[builder(into, default)]
    #[serde(rename = "batchFrequencyInSeconds")]
    pub r#batch_frequency_in_seconds: Box<Option<i32>>,
    /// The connection string for the endpoint. This attribute is mandatory and can only be specified when `authentication_type` is `keyBased`.
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The name of storage container in the storage account. This attribute is mandatory for endpoint type `AzureIotHub.StorageContainer`.
    #[builder(into, default)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<Option<String>>,
    /// Encoding that is used to serialize messages to blobs. Supported values are `Avro`, `AvroDeflate` and `JSON`. Default value is `Avro`. This attribute is applicable for endpoint type `AzureIotHub.StorageContainer`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// URI of the Service Bus or Event Hubs Namespace endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased` for endpoint type `AzureIotHub.ServiceBusQueue`, `AzureIotHub.ServiceBusTopic` or `AzureIotHub.EventHub`.
    #[builder(into, default)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: Box<Option<String>>,
    /// Name of the Service Bus Queue/Topic or Event Hub. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased` for endpoint type `AzureIotHub.ServiceBusQueue`, `AzureIotHub.ServiceBusTopic` or `AzureIotHub.EventHub`.
    #[builder(into, default)]
    #[serde(rename = "entityPath")]
    pub r#entity_path: Box<Option<String>>,
    /// File name format for the blob. All parameters are mandatory but can be reordered. This attribute is applicable for endpoint type `AzureIotHub.StorageContainer`. Defaults to `{iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}`.
    #[builder(into, default)]
    #[serde(rename = "fileNameFormat")]
    pub r#file_name_format: Box<Option<String>>,
    /// The ID of the User Managed Identity used to authenticate against the endpoint.
    /// 
    /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the IoT Hub. If `identity_id` is omitted when `authentication_type` is `identityBased`, then the System-Assigned Managed Identity of the IoT Hub will be used.
    /// 
    /// > **NOTE:** An IoT Hub can only be updated to use the System-Assigned Managed Identity for `endpoint` since it is not possible to grant access to the endpoint until after creation. The extracted resources `azurerm_iothub_endpoint_*` can be used to configure Endpoints with the IoT Hub's System-Assigned Managed Identity without the need for an update.
    #[builder(into, default)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<Option<String>>,
    /// Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB). This attribute is applicable for endpoint type `AzureIotHub.StorageContainer`.
    #[builder(into, default)]
    #[serde(rename = "maxChunkSizeInBytes")]
    pub r#max_chunk_size_in_bytes: Box<Option<i32>>,
    /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource group in which the endpoint will be created.
    #[builder(into, default)]
    #[serde(rename = "resourceGroupName")]
    pub r#resource_group_name: Box<Option<String>>,
    /// The type of the endpoint. Possible values are `AzureIotHub.StorageContainer`, `AzureIotHub.ServiceBusQueue`, `AzureIotHub.ServiceBusTopic` or `AzureIotHub.EventHub`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
