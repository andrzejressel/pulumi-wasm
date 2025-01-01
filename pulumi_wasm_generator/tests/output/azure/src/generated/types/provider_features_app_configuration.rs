#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeaturesAppConfiguration {
    #[builder(into, default)]
    #[serde(rename = "purgeSoftDeleteOnDestroy")]
    pub r#purge_soft_delete_on_destroy: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "recoverSoftDeleted")]
    pub r#recover_soft_deleted: Box<Option<bool>>,
}
