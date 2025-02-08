#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EmailIdentityDkimSigningAttributes {
    /// [Easy DKIM] The key length of the DKIM key pair in use.
    #[builder(into, default)]
    #[serde(rename = "currentSigningKeyLength")]
    pub r#current_signing_key_length: Box<Option<String>>,
    /// [Bring Your Own DKIM] A private key that's used to generate a DKIM signature. The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.
    /// 
    /// > **NOTE:** You have to delete the first and last lines ('-----BEGIN PRIVATE KEY-----' and '-----END PRIVATE KEY-----', respectively) of the generated private key. Additionally, you have to remove the line breaks in the generated private key. The resulting value is a string of characters with no spaces or line breaks.
    #[builder(into, default)]
    #[serde(rename = "domainSigningPrivateKey")]
    pub r#domain_signing_private_key: Box<Option<String>>,
    /// [Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.
    #[builder(into, default)]
    #[serde(rename = "domainSigningSelector")]
    pub r#domain_signing_selector: Box<Option<String>>,
    /// [Easy DKIM] The last time a key pair was generated for this identity.
    #[builder(into, default)]
    #[serde(rename = "lastKeyGenerationTimestamp")]
    pub r#last_key_generation_timestamp: Box<Option<String>>,
    /// [Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day. Valid values: `RSA_1024_BIT`, `RSA_2048_BIT`.
    #[builder(into, default)]
    #[serde(rename = "nextSigningKeyLength")]
    pub r#next_signing_key_length: Box<Option<String>>,
    /// A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
    #[builder(into, default)]
    #[serde(rename = "signingAttributesOrigin")]
    pub r#signing_attributes_origin: Box<Option<String>>,
    /// Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
    #[builder(into, default)]
    #[serde(rename = "tokens")]
    pub r#tokens: Box<Option<Vec<String>>>,
}
