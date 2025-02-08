#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheOriginOriginOverrideActionHeaderAction {
    /// Describes a header to add.
    /// You may add a maximum of 25 request headers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Box<Option<Vec<super::super::types::networkservices::EdgeCacheOriginOriginOverrideActionHeaderActionRequestHeadersToAdd>>>,
}
