#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HostingCustomDomainCert {
    /// The state of the certificate. Only the `CERT_ACTIVE` and
    /// `CERT_EXPIRING_SOON` states provide SSL coverage for a domain name. If the
    /// state is `PROPAGATING` and Hosting had an active cert for the domain name
    /// before, that formerly-active cert provides SSL coverage for the domain name
    /// until the current cert propagates.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The record's type, which determines what data the record contains.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// A set of ACME challenges you can add to your DNS records or existing,
    /// non-Hosting hosting provider to allow Hosting to create an SSL certificate
    /// for your domain name before you point traffic toward hosting. You can use
    /// thse challenges as part of a zero downtime transition from your old
    /// provider to Hosting.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "verification")]
    pub r#verification: Box<Option<super::super::types::firebase::HostingCustomDomainCertVerification>>,
}
