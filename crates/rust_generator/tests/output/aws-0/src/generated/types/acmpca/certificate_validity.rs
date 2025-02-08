#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateValidity {
    /// Determines how `value` is interpreted. Valid values: `DAYS`, `MONTHS`, `YEARS`, `ABSOLUTE`, `END_DATE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// If `type` is `DAYS`, `MONTHS`, or `YEARS`, the relative time until the certificate expires. If `type` is `ABSOLUTE`, the date in seconds since the Unix epoch. If `type` is `END_DATE`, the  date in RFC 3339 format.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
