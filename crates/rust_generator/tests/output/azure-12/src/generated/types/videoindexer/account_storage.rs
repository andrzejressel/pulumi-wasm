#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountStorage {
    /// The ID of the storage account to be associated with the Video Indexer Account. Changing this forces a new Video Indexer Account to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<String>,
    /// The reference to the user assigned identity to use to access the Storage Account.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
