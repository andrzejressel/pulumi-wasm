#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppIngressCustomDomain {
    /// The Binding type.
    #[builder(into, default)]
    #[serde(rename = "certificateBindingType")]
    pub r#certificate_binding_type: Box<Option<String>>,
    /// The ID of the Container App Environment Certificate.
    #[builder(into, default)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    /// The name for this Container App. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
