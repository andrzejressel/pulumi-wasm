#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingHostRule {
    /// A human-readable description of the hostRule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The list of host patterns to match.
    /// Host patterns must be valid hostnames. Ports are not allowed. Wildcard hosts are supported in the suffix or prefix form. * matches any string of ([a-z0-9-.]*). It does not match the empty string.
    /// When multiple hosts are specified, hosts are matched in the following priority:
    /// 1. Exact domain names: ``www.foo.com``.
    /// 2. Suffix domain wildcards: ``*.foo.com`` or ``*-bar.foo.com``.
    /// 3. Prefix domain wildcards: ``foo.*`` or ``foo-*``.
    /// 4. Special wildcard ``*`` matching any domain.
    /// Notes:
    /// The wildcard will not match the empty string. e.g. ``*-bar.foo.com`` will match ``baz-bar.foo.com`` but not ``-bar.foo.com``. The longest wildcards match first. Only a single host in the entire service can match on ``*``. A domain must be unique across all configured hosts within a service.
    /// Hosts are matched against the HTTP Host header, or for HTTP/2 and HTTP/3, the ":authority" header, from the incoming request.
    /// You may specify up to 10 hosts.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Vec<String>>,
    /// The name of the pathMatcher associated with this hostRule.
    #[builder(into)]
    #[serde(rename = "pathMatcher")]
    pub r#path_matcher: Box<String>,
}
