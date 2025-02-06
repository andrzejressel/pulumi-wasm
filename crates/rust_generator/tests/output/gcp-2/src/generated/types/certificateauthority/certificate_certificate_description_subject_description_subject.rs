#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubject {
    /// The common name of the distinguished name.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// The country code of the subject.
    #[builder(into, default)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<Option<String>>,
    /// The locality or city of the subject.
    #[builder(into, default)]
    #[serde(rename = "locality")]
    pub r#locality: Box<Option<String>>,
    /// The organization of the subject.
    #[builder(into, default)]
    #[serde(rename = "organization")]
    pub r#organization: Box<Option<String>>,
    /// The organizational unit of the subject.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Box<Option<String>>,
    /// The postal code of the subject.
    #[builder(into, default)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Box<Option<String>>,
    /// The province, territory, or regional state of the subject.
    #[builder(into, default)]
    #[serde(rename = "province")]
    pub r#province: Box<Option<String>>,
    /// The street address of the subject.
    #[builder(into, default)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: Box<Option<String>>,
}
