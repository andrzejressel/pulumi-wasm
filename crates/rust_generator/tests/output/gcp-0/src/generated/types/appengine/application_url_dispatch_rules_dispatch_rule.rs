#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationUrlDispatchRulesDispatchRule {
    /// Domain name to match against. The wildcard "*" is supported if specified before a period: "*.".
    /// Defaults to matching all domains: "*".
    #[builder(into, default)]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// Pathname within the host. Must start with a "/". A single "*" can be included at the end of the path.
    /// The sum of the lengths of the domain and path may not exceed 100 characters.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Pathname within the host. Must start with a "/". A single "*" can be included at the end of the path.
    /// The sum of the lengths of the domain and path may not exceed 100 characters.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
