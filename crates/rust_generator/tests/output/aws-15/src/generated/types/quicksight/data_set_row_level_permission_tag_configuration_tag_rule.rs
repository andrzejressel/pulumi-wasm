#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetRowLevelPermissionTagConfigurationTagRule {
    /// Column name that a tag key is assigned to.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// A string that you want to use to filter by all the values in a column in the dataset and don’t want to list the values one by one.
    #[builder(into, default)]
    #[serde(rename = "matchAllValue")]
    pub r#match_all_value: Box<Option<String>>,
    /// Unique key for a tag.
    #[builder(into)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: Box<String>,
    /// A string that you want to use to delimit the values when you pass the values at run time.
    #[builder(into, default)]
    #[serde(rename = "tagMultiValueDelimiter")]
    pub r#tag_multi_value_delimiter: Box<Option<String>>,
}
