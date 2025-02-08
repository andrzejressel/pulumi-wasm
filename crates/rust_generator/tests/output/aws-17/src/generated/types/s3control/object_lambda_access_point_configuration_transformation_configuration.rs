#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ObjectLambdaAccessPointConfigurationTransformationConfiguration {
    /// The actions of an Object Lambda Access Point configuration. Valid values: `GetObject`.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// The content transformation of an Object Lambda Access Point configuration. See Content Transformation below for more details.
    #[builder(into)]
    #[serde(rename = "contentTransformation")]
    pub r#content_transformation: Box<super::super::types::s3control::ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformation>,
}
