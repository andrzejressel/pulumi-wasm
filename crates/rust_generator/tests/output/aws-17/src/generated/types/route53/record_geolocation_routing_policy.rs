#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordGeolocationRoutingPolicy {
    /// A two-letter continent code. See http://docs.aws.amazon.com/Route53/latest/APIReference/API_GetGeoLocation.html for code details. Either `continent` or `country` must be specified.
    #[builder(into, default)]
    #[serde(rename = "continent")]
    pub r#continent: Box<Option<String>>,
    /// A two-character country code or `*` to indicate a default resource record set.
    #[builder(into, default)]
    #[serde(rename = "country")]
    pub r#country: Box<Option<String>>,
    /// A subdivision code for a country.
    #[builder(into, default)]
    #[serde(rename = "subdivision")]
    pub r#subdivision: Box<Option<String>>,
}
