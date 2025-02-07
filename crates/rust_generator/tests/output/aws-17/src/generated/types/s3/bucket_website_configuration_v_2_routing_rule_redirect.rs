#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketWebsiteConfigurationV2RoutingRuleRedirect {
    /// Host name to use in the redirect request.
    #[builder(into, default)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<Option<String>>,
    /// HTTP redirect code to use on the response.
    #[builder(into, default)]
    #[serde(rename = "httpRedirectCode")]
    pub r#http_redirect_code: Box<Option<String>>,
    /// Protocol to use when redirecting requests. The default is the protocol that is used in the original request. Valid values: `http`, `https`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Object key prefix to use in the redirect request. For example, to redirect requests for all pages with prefix `docs/` (objects in the `docs/` folder) to `documents/`, you can set a `condition` block with `key_prefix_equals` set to `docs/` and in the `redirect` set `replace_key_prefix_with` to `/documents`.
    #[builder(into, default)]
    #[serde(rename = "replaceKeyPrefixWith")]
    pub r#replace_key_prefix_with: Box<Option<String>>,
    /// Specific object key to use in the redirect request. For example, redirect request to `error.html`.
    #[builder(into, default)]
    #[serde(rename = "replaceKeyWith")]
    pub r#replace_key_with: Box<Option<String>>,
}
