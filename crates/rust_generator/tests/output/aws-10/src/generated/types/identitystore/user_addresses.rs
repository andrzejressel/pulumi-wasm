#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserAddresses {
    /// The country that this address is in.
    #[builder(into, default)]
    #[serde(rename = "country")]
    pub r#country: Box<Option<String>>,
    /// The name that is typically displayed when the address is shown for display.
    #[builder(into, default)]
    #[serde(rename = "formatted")]
    pub r#formatted: Box<Option<String>>,
    /// The address locality.
    #[builder(into, default)]
    #[serde(rename = "locality")]
    pub r#locality: Box<Option<String>>,
    /// The postal code of the address.
    #[builder(into, default)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Box<Option<String>>,
    /// When `true`, this is the primary address associated with the user.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
    /// The region of the address.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// The street of the address.
    #[builder(into, default)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: Box<Option<String>>,
    /// The type of address.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
