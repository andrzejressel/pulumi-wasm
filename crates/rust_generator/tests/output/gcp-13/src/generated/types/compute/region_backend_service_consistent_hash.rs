#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionBackendServiceConsistentHash {
    /// Hash is based on HTTP Cookie. This field describes a HTTP cookie
    /// that will be used as the hash key for the consistent hash load
    /// balancer. If the cookie is not present, it will be generated.
    /// This field is applicable if the sessionAffinity is set to HTTP_COOKIE.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "httpCookie")]
    pub r#http_cookie: Box<Option<super::super::types::compute::RegionBackendServiceConsistentHashHttpCookie>>,
    /// The hash based on the value of the specified header field.
    /// This field is applicable if the sessionAffinity is set to HEADER_FIELD.
    #[builder(into, default)]
    #[serde(rename = "httpHeaderName")]
    pub r#http_header_name: Box<Option<String>>,
    /// The minimum number of virtual nodes to use for the hash ring.
    /// Larger ring sizes result in more granular load
    /// distributions. If the number of hosts in the load balancing pool
    /// is larger than the ring size, each host will be assigned a single
    /// virtual node.
    /// Defaults to 1024.
    #[builder(into, default)]
    #[serde(rename = "minimumRingSize")]
    pub r#minimum_ring_size: Box<Option<i32>>,
}
