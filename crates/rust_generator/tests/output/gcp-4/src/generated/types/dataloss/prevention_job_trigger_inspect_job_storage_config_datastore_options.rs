#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobStorageConfigDatastoreOptions {
    /// A representation of a Datastore kind.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigDatastoreOptionsKind>,
    /// Datastore partition ID. A partition ID identifies a grouping of entities. The grouping
    /// is always by project and namespace, however the namespace ID may be empty.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "partitionId")]
    pub r#partition_id: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigDatastoreOptionsPartitionId>,
}
