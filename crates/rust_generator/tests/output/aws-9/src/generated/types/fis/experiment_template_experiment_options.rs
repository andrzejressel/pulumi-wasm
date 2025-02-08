#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExperimentTemplateExperimentOptions {
    /// Specifies the account targeting setting for experiment options. Supports `single-account` and `multi-account`.
    #[builder(into, default)]
    #[serde(rename = "accountTargeting")]
    pub r#account_targeting: Box<Option<String>>,
    /// Specifies the empty target resolution mode for experiment options. Supports `fail` and `skip`.
    #[builder(into, default)]
    #[serde(rename = "emptyTargetResolutionMode")]
    pub r#empty_target_resolution_mode: Box<Option<String>>,
}
