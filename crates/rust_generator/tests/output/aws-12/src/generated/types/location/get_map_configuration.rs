#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMapConfiguration {
    /// The map style selected from an available data provider.
    #[builder(into)]
    #[serde(rename = "style")]
    pub r#style: Box<String>,
}
