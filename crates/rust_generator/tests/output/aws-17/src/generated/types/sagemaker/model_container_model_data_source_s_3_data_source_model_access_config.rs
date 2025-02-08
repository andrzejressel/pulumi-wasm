#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ModelContainerModelDataSourceS3DataSourceModelAccessConfig {
    /// Specifies agreement to the model end-user license agreement (EULA). The AcceptEula value must be explicitly defined as `true` in order to accept the EULA that this model requires. You are responsible for reviewing and complying with any applicable license terms and making sure they are acceptable for your use case before downloading or using a model.
    #[builder(into)]
    #[serde(rename = "acceptEula")]
    pub r#accept_eula: Box<bool>,
}
