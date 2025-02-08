#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCatalogTableStorageDescriptorSkewedInfo {
    /// List of names of columns that contain skewed values.
    #[builder(into)]
    #[serde(rename = "skewedColumnNames")]
    pub r#skewed_column_names: Box<Vec<String>>,
    /// List of values that appear so frequently as to be considered skewed.
    #[builder(into)]
    #[serde(rename = "skewedColumnValueLocationMaps")]
    pub r#skewed_column_value_location_maps: Box<std::collections::HashMap<String, String>>,
    /// Map of skewed values to the columns that contain them.
    #[builder(into)]
    #[serde(rename = "skewedColumnValues")]
    pub r#skewed_column_values: Box<Vec<String>>,
}
