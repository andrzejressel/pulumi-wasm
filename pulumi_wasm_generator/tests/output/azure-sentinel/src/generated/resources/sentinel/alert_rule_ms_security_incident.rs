/// Manages a Sentinel MS Security Incident Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAlertRuleMsSecurityIncident = alert_rule_ms_security_incident::create(
///         "exampleAlertRuleMsSecurityIncident",
///         AlertRuleMsSecurityIncidentArgs::builder()
///             .display_name("example rule")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example-ms-security-incident-alert-rule")
///             .product_filter("Microsoft Cloud App Security")
///             .severity_filters(vec!["High",])
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel MS Security Incident Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleMsSecurityIncident:AlertRuleMsSecurityIncident example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
pub mod alert_rule_ms_security_incident {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleMsSecurityIncidentArgs {
        /// The GUID of the alert rule template which is used to create this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Sentinel MS Security Incident Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel MS Security Incident Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Only create incidents when the alert display name doesn't contain text from this list.
        #[builder(into, default)]
        pub display_name_exclude_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Only create incidents when the alert display name contain text from this list, leave empty to apply no filter.
        #[builder(into, default)]
        pub display_name_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Should this Sentinel MS Security Incident Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel MS Security Incident Alert Rule belongs to. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel MS Security Incident Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Microsoft Security Service from where the alert will be generated. Possible values are `Azure Active Directory Identity Protection`, `Azure Advanced Threat Protection`, `Azure Security Center`, `Azure Security Center for IoT`, `Microsoft Cloud App Security`, `Microsoft Defender Advanced Threat Protection` and `Office 365 Advanced Threat Protection`.
        #[builder(into)]
        pub product_filter: pulumi_wasm_rust::Output<String>,
        /// Only create incidents from alerts when alert severity level is contained in this list. Possible values are `High`, `Medium`, `Low` and `Informational`.
        ///
        /// > **NOTE** At least one of the severity filters need to be set.
        #[builder(into)]
        pub severity_filters: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleMsSecurityIncidentResult {
        /// The GUID of the alert rule template which is used to create this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Sentinel MS Security Incident Alert Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel MS Security Incident Alert Rule.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Only create incidents when the alert display name doesn't contain text from this list.
        pub display_name_exclude_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Only create incidents when the alert display name contain text from this list, leave empty to apply no filter.
        pub display_name_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Should this Sentinel MS Security Incident Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel MS Security Incident Alert Rule belongs to. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel MS Security Incident Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Microsoft Security Service from where the alert will be generated. Possible values are `Azure Active Directory Identity Protection`, `Azure Advanced Threat Protection`, `Azure Security Center`, `Azure Security Center for IoT`, `Microsoft Cloud App Security`, `Microsoft Defender Advanced Threat Protection` and `Office 365 Advanced Threat Protection`.
        pub product_filter: pulumi_wasm_rust::Output<String>,
        /// Only create incidents from alerts when alert severity level is contained in this list. Possible values are `High`, `Medium`, `Low` and `Informational`.
        ///
        /// > **NOTE** At least one of the severity filters need to be set.
        pub severity_filters: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AlertRuleMsSecurityIncidentArgs,
    ) -> AlertRuleMsSecurityIncidentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_rule_template_guid_binding = args.alert_rule_template_guid.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let display_name_exclude_filters_binding = args
            .display_name_exclude_filters
            .get_inner();
        let display_name_filters_binding = args.display_name_filters.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let product_filter_binding = args.product_filter.get_inner();
        let severity_filters_binding = args.severity_filters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleMsSecurityIncident:AlertRuleMsSecurityIncident"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertRuleTemplateGuid".into(),
                    value: &alert_rule_template_guid_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "displayNameExcludeFilters".into(),
                    value: &display_name_exclude_filters_binding,
                },
                register_interface::ObjectField {
                    name: "displayNameFilters".into(),
                    value: &display_name_filters_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "productFilter".into(),
                    value: &product_filter_binding,
                },
                register_interface::ObjectField {
                    name: "severityFilters".into(),
                    value: &severity_filters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alertRuleTemplateGuid".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "displayNameExcludeFilters".into(),
                },
                register_interface::ResultField {
                    name: "displayNameFilters".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "productFilter".into(),
                },
                register_interface::ResultField {
                    name: "severityFilters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleMsSecurityIncidentResult {
            alert_rule_template_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertRuleTemplateGuid").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            display_name_exclude_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayNameExcludeFilters").unwrap(),
            ),
            display_name_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayNameFilters").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            product_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productFilter").unwrap(),
            ),
            severity_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severityFilters").unwrap(),
            ),
        }
    }
}