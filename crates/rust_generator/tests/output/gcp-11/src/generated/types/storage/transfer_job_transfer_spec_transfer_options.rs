#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TransferJobTransferSpecTransferOptions {
    /// Whether objects should be deleted from the source after they are transferred to the sink. Note that this option and `delete_objects_unique_in_sink` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "deleteObjectsFromSourceAfterTransfer")]
    pub r#delete_objects_from_source_after_transfer: Box<Option<bool>>,
    /// Whether objects that exist only in the sink should be deleted. Note that this option and
    /// `delete_objects_from_source_after_transfer` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "deleteObjectsUniqueInSink")]
    pub r#delete_objects_unique_in_sink: Box<Option<bool>>,
    /// Whether overwriting objects that already exist in the sink is allowed.
    #[builder(into, default)]
    #[serde(rename = "overwriteObjectsAlreadyExistingInSink")]
    pub r#overwrite_objects_already_existing_in_sink: Box<Option<bool>>,
    /// When to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by `overwrite_objects_already_existing_in_sink`. Possible values: ALWAYS, DIFFERENT, NEVER.
    #[builder(into, default)]
    #[serde(rename = "overwriteWhen")]
    pub r#overwrite_when: Box<Option<String>>,
}
