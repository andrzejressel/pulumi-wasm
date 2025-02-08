#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LbTrafficExtensionExtensionChain {
    /// A set of extensions to execute for the matching request.
    /// At least one extension is required. Up to 3 extensions can be defined for each extension chain for
    /// LbTrafficExtension resource. LbRouteExtension chains are limited to 1 extension per extension chain.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "extensions")]
    pub r#extensions: Box<Vec<super::super::types::networkservices::LbTrafficExtensionExtensionChainExtension>>,
    /// Conditions under which this chain is invoked for a request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matchCondition")]
    pub r#match_condition: Box<super::super::types::networkservices::LbTrafficExtensionExtensionChainMatchCondition>,
    /// The name for this extension chain. The name is logged as part of the HTTP request logs.
    /// The name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,
    /// and can have a maximum length of 63 characters. Additionally, the first character must be a letter
    /// and the last a letter or a number.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
