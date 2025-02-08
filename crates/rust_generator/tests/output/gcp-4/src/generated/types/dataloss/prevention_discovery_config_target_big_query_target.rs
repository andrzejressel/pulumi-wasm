#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTarget {
    /// How often and when to update profiles. New tables that match both the fiter and conditions are scanned as quickly as possible depending on system capacity.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cadence")]
    pub r#cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadence>>,
    /// In addition to matching the filter, these conditions must be true before a profile is generated
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetConditions>>,
    /// Tables that match this filter will not have profiles created.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetDisabled>>,
    /// Required. The tables the discovery cadence applies to. The first target with a matching filter will be the one to apply to a table
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilter>>,
}
