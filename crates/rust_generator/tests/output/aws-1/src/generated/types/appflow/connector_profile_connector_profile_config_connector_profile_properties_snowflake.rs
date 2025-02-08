#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake {
    /// The name of the account.
    #[builder(into, default)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "privateLinkServiceName")]
    pub r#private_link_service_name: Box<Option<String>>,
    /// AWS Region of the Snowflake account.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// Name of the Amazon S3 stage that was created while setting up an Amazon S3 stage in the Snowflake account. This is written in the following format: `<Database>.<Schema>.<Stage Name>`.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: Box<String>,
    /// The name of the Snowflake warehouse.
    #[builder(into)]
    #[serde(rename = "warehouse")]
    pub r#warehouse: Box<String>,
}
