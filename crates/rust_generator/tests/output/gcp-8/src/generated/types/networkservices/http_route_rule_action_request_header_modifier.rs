#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HttpRouteRuleActionRequestHeaderModifier {
    /// Add the headers with given map where key is the name of the header, value is the value of the header.
    #[builder(into, default)]
    #[serde(rename = "add")]
    pub r#add: Box<Option<std::collections::HashMap<String, String>>>,
    /// Remove headers (matching by header names) specified in the list.
    #[builder(into, default)]
    #[serde(rename = "removes")]
    pub r#removes: Box<Option<Vec<String>>>,
    /// Completely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header.
    #[builder(into, default)]
    #[serde(rename = "set")]
    pub r#set: Box<Option<std::collections::HashMap<String, String>>>,
}
