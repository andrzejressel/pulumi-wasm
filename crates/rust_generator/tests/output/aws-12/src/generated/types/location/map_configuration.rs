#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MapConfiguration {
    /// Specifies the map style selected from an available data provider. Valid values can be found in the [Location Service CreateMap API Reference](https://docs.aws.amazon.com/location/latest/APIReference/API_CreateMap.html).
    #[builder(into)]
    #[serde(rename = "style")]
    pub r#style: Box<String>,
}
