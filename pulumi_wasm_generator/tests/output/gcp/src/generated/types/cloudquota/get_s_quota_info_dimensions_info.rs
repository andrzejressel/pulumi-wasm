#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSQuotaInfoDimensionsInfo {
    /// The applicable regions or zones of this dimensions info. The field will be set to `['global']` for quotas that are not per region or per zone. Otherwise, it will be set to the list of locations this dimension info is applicable to.
    #[builder(into)]
    #[serde(rename = "applicableLocations")]
    pub r#applicable_locations: Box<Vec<String>>,
    /// The quota details for a map of dimensions.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Box<Vec<super::super::types::cloudquota::GetSQuotaInfoDimensionsInfoDetail>>,
    /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<std::collections::HashMap<String, String>>,
}
