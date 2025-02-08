#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppConnectionString {
    /// The name of the Connection String.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Type of database. Possible values include: `APIHub`, `Custom`, `DocDb`, `EventHub`, `MySQL`, `NotificationHub`, `PostgreSQL`, `RedisCache`, `ServiceBus`, `SQLAzure`, and `SQLServer`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The connection string value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
