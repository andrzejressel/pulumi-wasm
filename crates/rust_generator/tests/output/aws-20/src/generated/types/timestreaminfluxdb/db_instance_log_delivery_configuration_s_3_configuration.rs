#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DbInstanceLogDeliveryConfigurationS3Configuration {
    /// Name of the S3 bucket to deliver logs to.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// Indicates whether log delivery to the S3 bucket is enabled.
    /// 
    /// **Note**: Only three arguments do updates in-place: `db_parameter_group_identifier`, `log_delivery_configuration`, and `tags`. Changes to any other argument after a DB instance has been deployed will cause destruction and re-creation of the DB instance. Additionally, when `db_parameter_group_identifier` is added to a DB instance or modified, the DB instance will be updated in-place but if `db_parameter_group_identifier` is removed from a DB instance, the DB instance will be destroyed and re-created.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
