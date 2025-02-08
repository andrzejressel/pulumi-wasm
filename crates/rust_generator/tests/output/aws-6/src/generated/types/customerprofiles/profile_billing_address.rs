#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProfileBillingAddress {
    /// The first line of a customer address.
    #[builder(into, default)]
    #[serde(rename = "address1")]
    pub r#address_1: Box<Option<String>>,
    /// The second line of a customer address.
    #[builder(into, default)]
    #[serde(rename = "address2")]
    pub r#address_2: Box<Option<String>>,
    /// The third line of a customer address.
    #[builder(into, default)]
    #[serde(rename = "address3")]
    pub r#address_3: Box<Option<String>>,
    /// The fourth line of a customer address.
    #[builder(into, default)]
    #[serde(rename = "address4")]
    pub r#address_4: Box<Option<String>>,
    /// The city in which a customer lives.
    #[builder(into, default)]
    #[serde(rename = "city")]
    pub r#city: Box<Option<String>>,
    /// The country in which a customer lives.
    #[builder(into, default)]
    #[serde(rename = "country")]
    pub r#country: Box<Option<String>>,
    /// The county in which a customer lives.
    #[builder(into, default)]
    #[serde(rename = "county")]
    pub r#county: Box<Option<String>>,
    /// The postal code of a customer address.
    #[builder(into, default)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Box<Option<String>>,
    /// The province in which a customer lives.
    #[builder(into, default)]
    #[serde(rename = "province")]
    pub r#province: Box<Option<String>>,
    /// The state in which a customer lives.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
