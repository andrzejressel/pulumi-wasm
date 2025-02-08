#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetEmailIdentityDkimSigningAttribute {
    /// [Easy DKIM] The key length of the DKIM key pair in use.
    #[builder(into)]
    #[serde(rename = "currentSigningKeyLength")]
    pub r#current_signing_key_length: Box<String>,
    #[builder(into)]
    #[serde(rename = "domainSigningPrivateKey")]
    pub r#domain_signing_private_key: Box<String>,
    #[builder(into)]
    #[serde(rename = "domainSigningSelector")]
    pub r#domain_signing_selector: Box<String>,
    /// [Easy DKIM] The last time a key pair was generated for this identity.
    #[builder(into)]
    #[serde(rename = "lastKeyGenerationTimestamp")]
    pub r#last_key_generation_timestamp: Box<String>,
    /// [Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.
    #[builder(into)]
    #[serde(rename = "nextSigningKeyLength")]
    pub r#next_signing_key_length: Box<String>,
    /// A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
    #[builder(into)]
    #[serde(rename = "signingAttributesOrigin")]
    pub r#signing_attributes_origin: Box<String>,
    /// Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
    #[builder(into)]
    #[serde(rename = "tokens")]
    pub r#tokens: Box<Vec<String>>,
}
