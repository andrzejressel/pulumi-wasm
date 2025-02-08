#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatasetAccessAuthorizedDataset {
    /// The dataset this entry applies to
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<super::super::types::bigquery::DatasetAccessAuthorizedDatasetDataset>,
    /// Which resources in the dataset this entry applies to. Currently, only views are supported,
    /// but additional target types may be added in the future. Possible values: VIEWS
    #[builder(into)]
    #[serde(rename = "targetTypes")]
    pub r#target_types: Box<Vec<String>>,
}
