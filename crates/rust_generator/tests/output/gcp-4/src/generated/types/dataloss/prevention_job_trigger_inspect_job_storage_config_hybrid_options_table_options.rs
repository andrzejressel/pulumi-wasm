#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobStorageConfigHybridOptionsTableOptions {
    /// The columns that are the primary keys for table objects included in ContentItem. A copy of this
    /// cell's value will stored alongside alongside each finding so that the finding can be traced to
    /// the specific row it came from. No more than 3 may be provided.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "identifyingFields")]
    pub r#identifying_fields: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigHybridOptionsTableOptionsIdentifyingField>>>,
}
