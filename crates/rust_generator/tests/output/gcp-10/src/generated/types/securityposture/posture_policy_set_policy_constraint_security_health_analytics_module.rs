#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule {
    /// The state of enablement for the module at its level of the resource hierarchy.
    /// Possible values are: `ENABLEMENT_STATE_UNSPECIFIED`, `ENABLED`, `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "moduleEnablementState")]
    pub r#module_enablement_state: Box<Option<String>>,
    /// The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED.
    #[builder(into)]
    #[serde(rename = "moduleName")]
    pub r#module_name: Box<String>,
}
