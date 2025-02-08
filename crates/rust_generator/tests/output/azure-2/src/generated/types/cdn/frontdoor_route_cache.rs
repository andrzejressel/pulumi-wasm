#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRouteCache {
    /// Is content compression enabled? Possible values are `true` or `false`. Defaults to `false`.
    /// 
    /// > **NOTE:** Content won't be compressed when the requested content is smaller than `1 KB` or larger than `8 MB`(inclusive).
    #[builder(into, default)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Box<Option<bool>>,
    /// A list of one or more `Content types` (formerly known as `MIME types`) to compress. Possible values include `application/eot`, `application/font`, `application/font-sfnt`, `application/javascript`, `application/json`, `application/opentype`, `application/otf`, `application/pkcs7-mime`, `application/truetype`, `application/ttf`, `application/vnd.ms-fontobject`, `application/xhtml+xml`, `application/xml`, `application/xml+rss`, `application/x-font-opentype`, `application/x-font-truetype`, `application/x-font-ttf`, `application/x-httpd-cgi`, `application/x-mpegurl`, `application/x-opentype`, `application/x-otf`, `application/x-perl`, `application/x-ttf`, `application/x-javascript`, `font/eot`, `font/ttf`, `font/otf`, `font/opentype`, `image/svg+xml`, `text/css`, `text/csv`, `text/html`, `text/javascript`, `text/js`, `text/plain`, `text/richtext`, `text/tab-separated-values`, `text/xml`, `text/x-script`, `text/x-component` or `text/x-java-source`.
    #[builder(into, default)]
    #[serde(rename = "contentTypesToCompresses")]
    pub r#content_types_to_compresses: Box<Option<Vec<String>>>,
    /// Defines how the Front Door Route will cache requests that include query strings. Possible values include `IgnoreQueryString`, `IgnoreSpecifiedQueryStrings`, `IncludeSpecifiedQueryStrings` or `UseQueryString`. Defaults to `IgnoreQueryString`.
    /// 
    /// > **NOTE:** The value of the `query_string_caching_behavior` determines if the `query_strings` field will be used as an include list or an ignore list.
    #[builder(into, default)]
    #[serde(rename = "queryStringCachingBehavior")]
    pub r#query_string_caching_behavior: Box<Option<String>>,
    /// Query strings to include or ignore.
    #[builder(into, default)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Option<Vec<String>>>,
}
