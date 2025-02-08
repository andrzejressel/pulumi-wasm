#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationSubject {
    /// Fully qualified domain name (FQDN) associated with the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// Two digit code that specifies the country in which the certificate subject located. Must be less than or equal to 2 characters in length.
    #[builder(into, default)]
    #[serde(rename = "country")]
    pub r#country: Box<Option<String>>,
    /// Disambiguating information for the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "distinguishedNameQualifier")]
    pub r#distinguished_name_qualifier: Box<Option<String>>,
    /// Typically a qualifier appended to the name of an individual. Examples include Jr. for junior, Sr. for senior, and III for third. Must be less than or equal to 3 characters in length.
    #[builder(into, default)]
    #[serde(rename = "generationQualifier")]
    pub r#generation_qualifier: Box<Option<String>>,
    /// First name. Must be less than or equal to 16 characters in length.
    #[builder(into, default)]
    #[serde(rename = "givenName")]
    pub r#given_name: Box<Option<String>>,
    /// Concatenation that typically contains the first letter of the `given_name`, the first letter of the middle name if one exists, and the first letter of the `surname`. Must be less than or equal to 5 characters in length.
    #[builder(into, default)]
    #[serde(rename = "initials")]
    pub r#initials: Box<Option<String>>,
    /// Locality (such as a city or town) in which the certificate subject is located. Must be less than or equal to 128 characters in length.
    #[builder(into, default)]
    #[serde(rename = "locality")]
    pub r#locality: Box<Option<String>>,
    /// Legal name of the organization with which the certificate subject is affiliated. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "organization")]
    pub r#organization: Box<Option<String>>,
    /// Subdivision or unit of the organization (such as sales or finance) with which the certificate subject is affiliated. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Box<Option<String>>,
    /// Typically a shortened version of a longer `given_name`. For example, Jonathan is often shortened to John. Elizabeth is often shortened to Beth, Liz, or Eliza. Must be less than or equal to 128 characters in length.
    #[builder(into, default)]
    #[serde(rename = "pseudonym")]
    pub r#pseudonym: Box<Option<String>>,
    /// State in which the subject of the certificate is located. Must be less than or equal to 128 characters in length.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Family name. In the US and the UK for example, the surname of an individual is ordered last. In Asian cultures the surname is typically ordered first. Must be less than or equal to 40 characters in length.
    #[builder(into, default)]
    #[serde(rename = "surname")]
    pub r#surname: Box<Option<String>>,
    /// Title such as Mr. or Ms. which is pre-pended to the name to refer formally to the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
