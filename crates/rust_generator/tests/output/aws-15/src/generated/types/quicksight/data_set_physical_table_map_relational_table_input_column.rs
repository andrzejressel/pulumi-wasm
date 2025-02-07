#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetPhysicalTableMapRelationalTableInputColumn {
    /// Name of this column in the underlying data source.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Data type of the column.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
