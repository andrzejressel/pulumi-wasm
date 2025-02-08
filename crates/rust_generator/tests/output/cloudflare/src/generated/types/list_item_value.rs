#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListItemValue {
    #[builder(into, default)]
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Option<Vec<super::types::ListItemValueHostname>>>,
    #[builder(into, default)]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Option<Vec<super::types::ListItemValueRedirect>>>,
}
