#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSrvRecordRecord {
    /// Port the service is listening on.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Priority of the SRV record.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// FQDN of the service.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Weight of the SRV record.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
