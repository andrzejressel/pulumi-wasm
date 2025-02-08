#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallApplicationRuleCollectionRuleProtocol {
    /// Specify a port for the connection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Specifies the type of connection. Possible values are `Http`, `Https` and `Mssql`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
