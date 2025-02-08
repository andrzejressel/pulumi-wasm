#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrailEventSelector {
    /// Configuration block for data events. See details below.
    #[builder(into, default)]
    #[serde(rename = "dataResources")]
    pub r#data_resources: Box<Option<Vec<super::super::types::cloudtrail::TrailEventSelectorDataResource>>>,
    /// A set of event sources to exclude. Valid values include: `kms.amazonaws.com` and `rdsdata.amazonaws.com`. `include_management_events` must be set to`true` to allow this.
    #[builder(into, default)]
    #[serde(rename = "excludeManagementEventSources")]
    pub r#exclude_management_event_sources: Box<Option<Vec<String>>>,
    /// Whether to include management events for your trail. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "includeManagementEvents")]
    pub r#include_management_events: Box<Option<bool>>,
    /// Type of events to log. Valid values are `ReadOnly`, `WriteOnly`, `All`. Default value is `All`.
    #[builder(into, default)]
    #[serde(rename = "readWriteType")]
    pub r#read_write_type: Box<Option<String>>,
}
