#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PartitionStorageDescriptorSkewedInfo {
    /// A list of names of columns that contain skewed values.
    #[builder(into, default)]
    #[serde(rename = "skewedColumnNames")]
    pub r#skewed_column_names: Box<Option<Vec<String>>>,
    /// A list of values that appear so frequently as to be considered skewed.
    #[builder(into, default)]
    #[serde(rename = "skewedColumnValueLocationMaps")]
    pub r#skewed_column_value_location_maps: Box<Option<std::collections::HashMap<String, String>>>,
    /// A map of skewed values to the columns that contain them.
    #[builder(into, default)]
    #[serde(rename = "skewedColumnValues")]
    pub r#skewed_column_values: Box<Option<Vec<String>>>,
}
