#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppIngressCustomDomain {
    /// The Binding type. Possible values include `Disabled` and `SniEnabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "certificateBindingType")]
    pub r#certificate_binding_type: Box<String>,
    /// The ID of the Container App Environment Certificate.
    #[builder(into)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<String>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
