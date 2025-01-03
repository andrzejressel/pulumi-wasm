#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingVersionConfig {
    /// An array of objects, where each object specifies a URL pattern that, if matched to the request URL path,
    /// triggers Hosting to apply the specified custom response headers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::firebase::HostingVersionConfigHeader>>>,
    /// An array of objects (called redirect rules), where each rule specifies a URL pattern that, if matched to the request URL path,
    /// triggers Hosting to respond with a redirect to the specified destination path.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Option<Vec<super::super::types::firebase::HostingVersionConfigRedirect>>>,
    /// An array of objects (called rewrite rules), where each rule specifies a URL pattern that, if matched to the
    /// request URL path, triggers Hosting to respond as if the service were given the specified destination URL.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rewrites")]
    pub r#rewrites: Box<Option<Vec<super::super::types::firebase::HostingVersionConfigRewrite>>>,
}
