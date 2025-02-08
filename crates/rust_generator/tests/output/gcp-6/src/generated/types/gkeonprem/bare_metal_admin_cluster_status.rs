#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterStatus {
    /// (Output)
    /// ResourceConditions provide a standard mechanism for higher-level status reporting from admin cluster controller.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterStatusCondition>>>,
    /// (Output)
    /// Human-friendly representation of the error message from the admin cluster
    /// controller. The error message can be temporary as the admin cluster
    /// controller creates a cluster or node pool. If the error message persists
    /// for a longer period of time, it can be used to surface error message to
    /// indicate real problems requiring user intervention.
    #[builder(into, default)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Box<Option<String>>,
}
