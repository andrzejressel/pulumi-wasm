#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ImageRecipeComponent {
    /// Amazon Resource Name (ARN) of the Image Builder Component to associate.
    #[builder(into)]
    #[serde(rename = "componentArn")]
    pub r#component_arn: Box<String>,
    /// Configuration block(s) for parameters to configure the component. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::imagebuilder::ImageRecipeComponentParameter>>>,
}
