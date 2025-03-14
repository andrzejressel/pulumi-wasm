#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLocationData {
    /// The city or locality where the resource is located.
    #[builder(into)]
    #[serde(rename = "city")]
    pub r#city: Box<String>,
    /// The country or region where the resource is located.
    #[builder(into)]
    #[serde(rename = "countryOrRegion")]
    pub r#country_or_region: Box<String>,
    /// The district, state, or province where the resource is located.
    #[builder(into)]
    #[serde(rename = "district")]
    pub r#district: Box<String>,
    /// The name of this Azure Arc machine.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
