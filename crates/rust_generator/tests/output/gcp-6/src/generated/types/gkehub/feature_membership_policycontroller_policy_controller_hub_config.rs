#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfig {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into, default)]
    #[serde(rename = "auditIntervalSeconds")]
    pub r#audit_interval_seconds: Box<Option<i32>>,
    /// The maximum number of audit violations to be stored in a constraint. If not set, the  default of 20 will be used.
    #[builder(into, default)]
    #[serde(rename = "constraintViolationLimit")]
    pub r#constraint_violation_limit: Box<Option<i32>>,
    /// Map of deployment configs to deployments ("admission", "audit", "mutation").
    #[builder(into, default)]
    #[serde(rename = "deploymentConfigs")]
    pub r#deployment_configs: Box<Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig>>>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into, default)]
    #[serde(rename = "exemptableNamespaces")]
    pub r#exemptable_namespaces: Box<Option<Vec<String>>>,
    /// Configures the mode of the Policy Controller installation. Must be one of `INSTALL_SPEC_NOT_INSTALLED`, `INSTALL_SPEC_ENABLED`, `INSTALL_SPEC_SUSPENDED` or `INSTALL_SPEC_DETACHED`.
    #[builder(into, default)]
    #[serde(rename = "installSpec")]
    pub r#install_spec: Box<Option<String>>,
    /// Logs all denies and dry run failures.
    #[builder(into, default)]
    #[serde(rename = "logDeniesEnabled")]
    pub r#log_denies_enabled: Box<Option<bool>>,
    /// Specifies the backends Policy Controller should export metrics to. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Box<Option<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigMonitoring>>,
    /// Enables mutation in policy controller. If true, mutation CRDs, webhook, and controller deployment will be deployed to the cluster.
    #[builder(into, default)]
    #[serde(rename = "mutationEnabled")]
    pub r#mutation_enabled: Box<Option<bool>>,
    /// Specifies the desired policy content on the cluster. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "policyContent")]
    pub r#policy_content: Box<Option<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContent>>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into, default)]
    #[serde(rename = "referentialRulesEnabled")]
    pub r#referential_rules_enabled: Box<Option<bool>>,
}
