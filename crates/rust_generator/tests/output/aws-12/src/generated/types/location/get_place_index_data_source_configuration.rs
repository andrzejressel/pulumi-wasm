#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlaceIndexDataSourceConfiguration {
    #[builder(into)]
    #[serde(rename = "intendedUse")]
    pub r#intended_use: Box<String>,
}
