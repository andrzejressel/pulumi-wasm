#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureMembershipConfigmanagementPolicyController {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into, default)]
    #[serde(rename = "auditIntervalSeconds")]
    pub r#audit_interval_seconds: Box<Option<String>>,
    /// Enables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into, default)]
    #[serde(rename = "exemptableNamespaces")]
    pub r#exemptable_namespaces: Box<Option<Vec<String>>>,
    /// Logs all denies and dry run failures.
    #[builder(into, default)]
    #[serde(rename = "logDeniesEnabled")]
    pub r#log_denies_enabled: Box<Option<bool>>,
    /// Specifies the backends Policy Controller should export metrics to. For example, to specify metrics should be exported to Cloud Monitoring and Prometheus, specify backends: ["cloudmonitoring", "prometheus"]. Default: ["cloudmonitoring", "prometheus"]
    #[builder(into, default)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Box<Option<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyControllerMonitoring>>,
    /// Enables mutation in policy controller. If true, mutation CRDs, webhook, and controller deployment will be deployed to the cluster.
    #[builder(into, default)]
    #[serde(rename = "mutationEnabled")]
    pub r#mutation_enabled: Box<Option<bool>>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into, default)]
    #[serde(rename = "referentialRulesEnabled")]
    pub r#referential_rules_enabled: Box<Option<bool>>,
    /// Installs the default template library along with Policy Controller.
    #[builder(into, default)]
    #[serde(rename = "templateLibraryInstalled")]
    pub r#template_library_installed: Box<Option<bool>>,
}
