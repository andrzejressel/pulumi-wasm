#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IoTHubFileUpload {
    /// The type used to authenticate against the storage account. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
    #[builder(into, default)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<Option<String>>,
    /// The connection string for the Azure Storage account to which files are uploaded.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<String>,
    /// The name of the root container where the files should be uploaded to. The container need not exist but should be creatable using the connection_string specified.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// The period of time for which a file upload notification message is available to consume before it expires, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 48 hours. Defaults to `PT1H`.
    #[builder(into, default)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<Option<String>>,
    /// The ID of the User Managed Identity used to authenticate against the storage account.
    /// 
    /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the IoT Hub. If `identity_id` is omitted when `authentication_type` is `identityBased`, then the System-Assigned Managed Identity of the IoT Hub will be used.
    /// 
    /// > **NOTE:** An IoT Hub can only be updated to use the System-Assigned Managed Identity for `file_upload` since it is not possible to grant access to the endpoint until after creation.
    #[builder(into, default)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<Option<String>>,
    /// The lock duration for the file upload notifications queue, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 5 and 300 seconds. Defaults to `PT1M`.
    #[builder(into, default)]
    #[serde(rename = "lockDuration")]
    pub r#lock_duration: Box<Option<String>>,
    /// The number of times the IoT Hub attempts to deliver a file upload notification message. Defaults to `10`.
    #[builder(into, default)]
    #[serde(rename = "maxDeliveryCount")]
    pub r#max_delivery_count: Box<Option<i32>>,
    /// Used to specify whether file notifications are sent to IoT Hub on upload. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "notifications")]
    pub r#notifications: Box<Option<bool>>,
    /// The period of time for which the SAS URI generated by IoT Hub for file upload is valid, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 24 hours. Defaults to `PT1H`.
    #[builder(into, default)]
    #[serde(rename = "sasTtl")]
    pub r#sas_ttl: Box<Option<String>>,
}