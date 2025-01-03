#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionHandlerStaticFiles {
    /// Whether files should also be uploaded as code data. By default, files declared in static file handlers are
    /// uploaded as static data and are only served to end users; they cannot be read by the application. If enabled,
    /// uploads are charged against both your code and static data storage resource quotas.
    #[builder(into, default)]
    #[serde(rename = "applicationReadable")]
    pub r#application_readable: Box<Option<bool>>,
    /// Time a static file served by this handler should be cached by web proxies and browsers.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example "3.5s".
    /// Default is '0s'
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    /// HTTP headers to use for all responses from these URLs.
    /// An object containing a list of "key:value" value pairs.".
    #[builder(into, default)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// MIME type used to serve all files served by this handler.
    /// Defaults to file-specific MIME types, which are derived from each file's filename extension.
    #[builder(into, default)]
    #[serde(rename = "mimeType")]
    pub r#mime_type: Box<Option<String>>,
    /// Path to the static files matched by the URL pattern, from the application root directory.
    /// The path can refer to text matched in groupings in the URL pattern.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Whether this handler should match the request if the file referenced by the handler does not exist.
    #[builder(into, default)]
    #[serde(rename = "requireMatchingFile")]
    pub r#require_matching_file: Box<Option<bool>>,
    /// Regular expression that matches the file paths for all files that should be referenced by this handler.
    #[builder(into, default)]
    #[serde(rename = "uploadPathRegex")]
    pub r#upload_path_regex: Box<Option<String>>,
}
