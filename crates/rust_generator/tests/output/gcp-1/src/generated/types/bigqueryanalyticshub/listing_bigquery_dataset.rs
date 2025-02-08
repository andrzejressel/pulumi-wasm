#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListingBigqueryDataset {
    /// Resource name of the dataset source for this listing. e.g. projects/myproject/datasets/123
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    /// Resource in this dataset that is selectively shared. This field is required for data clean room exchanges.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "selectedResources")]
    pub r#selected_resources: Box<Option<Vec<super::super::types::bigqueryanalyticshub::ListingBigqueryDatasetSelectedResource>>>,
}
