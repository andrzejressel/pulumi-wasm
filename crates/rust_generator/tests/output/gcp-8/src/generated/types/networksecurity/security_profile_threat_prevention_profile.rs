#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityProfileThreatPreventionProfile {
    /// The configuration for overriding threats actions by severity match.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "severityOverrides")]
    pub r#severity_overrides: Box<Option<Vec<super::super::types::networksecurity::SecurityProfileThreatPreventionProfileSeverityOverride>>>,
    /// The configuration for overriding threats actions by threat id match.
    /// If a threat is matched both by configuration provided in severity overrides
    /// and threat overrides, the threat overrides action is applied.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "threatOverrides")]
    pub r#threat_overrides: Box<Option<Vec<super::super::types::networksecurity::SecurityProfileThreatPreventionProfileThreatOverride>>>,
}
