#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorityConfigX509ConfigNameConstraints {
    /// Indicates whether or not the name constraints are marked critical.
    #[builder(into)]
    #[serde(rename = "critical")]
    pub r#critical: Box<bool>,
    /// Contains excluded DNS names. Any DNS name that can be
    /// constructed by simply adding zero or more labels to
    /// the left-hand side of the name satisfies the name constraint.
    /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
    /// would satisfy `example.com` while `example1.com` does not.
    #[builder(into, default)]
    #[serde(rename = "excludedDnsNames")]
    pub r#excluded_dns_names: Box<Option<Vec<String>>>,
    /// Contains the excluded email addresses. The value can be a particular
    /// email address, a hostname to indicate all email addresses on that host or
    /// a domain with a leading period (e.g. `.example.com`) to indicate
    /// all email addresses in that domain.
    #[builder(into, default)]
    #[serde(rename = "excludedEmailAddresses")]
    pub r#excluded_email_addresses: Box<Option<Vec<String>>>,
    /// Contains the excluded IP ranges. For IPv4 addresses, the ranges
    /// are expressed using CIDR notation as specified in RFC 4632.
    /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
    /// addresses.
    #[builder(into, default)]
    #[serde(rename = "excludedIpRanges")]
    pub r#excluded_ip_ranges: Box<Option<Vec<String>>>,
    /// Contains the excluded URIs that apply to the host part of the name.
    /// The value can be a hostname or a domain with a
    /// leading period (like `.example.com`)
    #[builder(into, default)]
    #[serde(rename = "excludedUris")]
    pub r#excluded_uris: Box<Option<Vec<String>>>,
    /// Contains permitted DNS names. Any DNS name that can be
    /// constructed by simply adding zero or more labels to
    /// the left-hand side of the name satisfies the name constraint.
    /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
    /// would satisfy `example.com` while `example1.com` does not.
    #[builder(into, default)]
    #[serde(rename = "permittedDnsNames")]
    pub r#permitted_dns_names: Box<Option<Vec<String>>>,
    /// Contains the permitted email addresses. The value can be a particular
    /// email address, a hostname to indicate all email addresses on that host or
    /// a domain with a leading period (e.g. `.example.com`) to indicate
    /// all email addresses in that domain.
    #[builder(into, default)]
    #[serde(rename = "permittedEmailAddresses")]
    pub r#permitted_email_addresses: Box<Option<Vec<String>>>,
    /// Contains the permitted IP ranges. For IPv4 addresses, the ranges
    /// are expressed using CIDR notation as specified in RFC 4632.
    /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
    /// addresses.
    #[builder(into, default)]
    #[serde(rename = "permittedIpRanges")]
    pub r#permitted_ip_ranges: Box<Option<Vec<String>>>,
    /// Contains the permitted URIs that apply to the host part of the name.
    /// The value can be a hostname or a domain with a
    /// leading period (like `.example.com`)
    #[builder(into, default)]
    #[serde(rename = "permittedUris")]
    pub r#permitted_uris: Box<Option<Vec<String>>>,
}
