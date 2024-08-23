#[derive(serde::Serialize)]
pub struct ListItemValue {
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Option<Vec<crate::types::ListItemValueHostname>>>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Option<Vec<crate::types::ListItemValueRedirect>>>,
}
