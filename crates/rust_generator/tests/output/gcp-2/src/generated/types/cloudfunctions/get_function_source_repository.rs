#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFunctionSourceRepository {
    /// The URL pointing to the hosted repository where the function was defined at the time of deployment.
    #[builder(into)]
    #[serde(rename = "deployedUrl")]
    pub r#deployed_url: Box<String>,
    /// The URL pointing to the hosted repository where the function is defined.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
