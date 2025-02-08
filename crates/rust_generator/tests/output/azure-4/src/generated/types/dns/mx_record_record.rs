#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MxRecordRecord {
    /// The mail server responsible for the domain covered by the MX record.
    #[builder(into)]
    #[serde(rename = "exchange")]
    pub r#exchange: Box<String>,
    /// String representing the "preference” value of the MX records. Records with lower preference value take priority.
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: Box<String>,
}
