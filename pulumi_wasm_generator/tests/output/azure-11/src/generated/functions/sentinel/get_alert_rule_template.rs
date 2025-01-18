pub mod get_alert_rule_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleTemplateArgs {
        /// The display name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        ///
        /// > **NOTE** As `display_name` is not unique, errors may occur when there are multiple Sentinel Alert Rule Template with same `display_name`.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleTemplateResult {
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `nrt_template` block as defined below. This only applies to Sentinel NRT Alert Rule Template.
        pub nrt_templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sentinel::GetAlertRuleTemplateNrtTemplate>,
        >,
        /// A `scheduled_template` block as defined below. This only applies to Sentinel Scheduled Alert Rule Template.
        pub scheduled_templates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleTemplateScheduledTemplate,
            >,
        >,
        /// A `security_incident_template` block as defined below. This only applies to Sentinel MS Security Incident Alert Rule Template.
        pub security_incident_templates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleTemplateSecurityIncidentTemplate,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAlertRuleTemplateArgs) -> GetAlertRuleTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:sentinel/getAlertRuleTemplate:getAlertRuleTemplate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nrtTemplates".into(),
                },
                register_interface::ResultField {
                    name: "scheduledTemplates".into(),
                },
                register_interface::ResultField {
                    name: "securityIncidentTemplates".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAlertRuleTemplateResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nrt_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nrtTemplates").unwrap(),
            ),
            scheduled_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledTemplates").unwrap(),
            ),
            security_incident_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityIncidentTemplates").unwrap(),
            ),
        }
    }
}
