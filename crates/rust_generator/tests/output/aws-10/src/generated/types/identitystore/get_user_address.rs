#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserAddress {
    /// The country that this address is in.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: Box<String>,
    /// The address locality.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: Box<String>,
    /// The postal code of the address.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Box<String>,
    /// When `true`, this is the primary phone number associated with the user.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// The region of the address.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// The street of the address.
    #[builder(into)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: Box<String>,
    /// The type of phone number.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
