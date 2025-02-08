#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceServiceConnectConfigurationServiceClientAlias {
    /// Name that you use in the applications of client tasks to connect to this service.
    #[builder(into, default)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Box<Option<String>>,
    /// Listening port number for the Service Connect proxy. This port is available inside of all of the tasks within the same namespace.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
