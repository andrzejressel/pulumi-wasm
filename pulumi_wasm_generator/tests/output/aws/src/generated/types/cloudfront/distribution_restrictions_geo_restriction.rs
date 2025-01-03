#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionRestrictionsGeoRestriction {
    /// [ISO 3166-1-alpha-2 codes][4] for which you want CloudFront either to distribute your content (`whitelist`) or not distribute your content (`blacklist`). If the type is specified as `none` an empty array can be used.
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<String>>>,
    /// Method that you want to use to restrict distribution of your content by country: `none`, `whitelist`, or `blacklist`.
    #[builder(into)]
    #[serde(rename = "restrictionType")]
    pub r#restriction_type: Box<String>,
}
