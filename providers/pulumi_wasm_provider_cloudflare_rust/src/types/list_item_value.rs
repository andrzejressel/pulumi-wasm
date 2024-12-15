#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListItemValue {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Option<Vec<crate::types::ListItemValueHostname>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Option<Vec<crate::types::ListItemValueRedirect>>>,
}
