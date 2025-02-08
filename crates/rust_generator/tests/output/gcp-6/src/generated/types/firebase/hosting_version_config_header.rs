#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HostingVersionConfigHeader {
    /// The user-supplied glob to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "glob")]
    pub r#glob: Box<Option<String>>,
    /// The additional headers to add to the response. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<std::collections::HashMap<String, String>>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
}
