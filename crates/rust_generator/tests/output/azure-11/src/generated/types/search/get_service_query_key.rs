#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceQueryKey {
    /// The value of this Query Key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The Name of the Search Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
