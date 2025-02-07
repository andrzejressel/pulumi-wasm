#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatasetAccessDataset {
    /// The dataset this entry applies to
    #[builder(into)]
    #[serde(rename = "datasets")]
    pub r#datasets: Box<Vec<super::super::types::bigquery::GetDatasetAccessDatasetDataset>>,
    /// Which resources in the dataset this entry applies to. Currently, only views are supported,
    /// but additional target types may be added in the future. Possible values: VIEWS
    #[builder(into)]
    #[serde(rename = "targetTypes")]
    pub r#target_types: Box<Vec<String>>,
}
