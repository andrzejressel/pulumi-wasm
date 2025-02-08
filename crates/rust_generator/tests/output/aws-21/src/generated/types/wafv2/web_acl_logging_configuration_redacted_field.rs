#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclLoggingConfigurationRedactedField {
    /// HTTP method to be redacted. It must be specified as an empty configuration block `{}`. The method indicates the type of operation that the request is asking the origin to perform.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationRedactedFieldMethod>>,
    /// Whether to redact the query string. It must be specified as an empty configuration block `{}`. The query string is the part of a URL that appears after a `?` character, if any.
    #[builder(into, default)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationRedactedFieldQueryString>>,
    /// "single_header" refers to the redaction of a single header. For more information, please see the details below under Single Header.
    #[builder(into, default)]
    #[serde(rename = "singleHeader")]
    pub r#single_header: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationRedactedFieldSingleHeader>>,
    /// Configuration block that redacts the request URI path. It should be specified as an empty configuration block `{}`. The URI path is the part of a web request that identifies a resource, such as `/images/daily-ad.jpg`.
    #[builder(into, default)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationRedactedFieldUriPath>>,
}
