#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountDataStore {
    /// The ID of the Storage Account that should be linked to this Azure Maps Account.
    #[builder(into, default)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<Option<String>>,
    /// The name given to the linked Storage Account.
    #[builder(into)]
    #[serde(rename = "uniqueName")]
    pub r#unique_name: Box<String>,
}
