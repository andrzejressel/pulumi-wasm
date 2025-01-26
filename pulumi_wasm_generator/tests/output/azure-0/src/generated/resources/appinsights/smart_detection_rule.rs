/// Manages an Application Insights Smart Detection Rule.
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
///             .name("tf-test")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("tf-test-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSmartDetectionRule = smart_detection_rule::create(
///         "exampleSmartDetectionRule",
///         SmartDetectionRuleArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .enabled(false)
///             .name("Slow server response time")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights Smart Detection Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/smartDetectionRule:SmartDetectionRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Insights/components/mycomponent1/proactiveDetectionConfigs/myrule1
/// ```
///
pub mod smart_detection_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmartDetectionRuleArgs {
        /// Specifies a list of additional recipients that will be sent emails on this Application Insights Smart Detection Rule.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        #[builder(into, default)]
        pub additional_email_recipients: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the Application Insights component on which the Smart Detection Rule operates. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Is the Application Insights Smart Detection Rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Application Insights Smart Detection Rule. Valid values include `Slow page load time`, `Slow server response time`, `Potential memory leak detected`, `Potential security issue detected`, `Long dependency duration`, `Degradation in server response time`, `Degradation in dependency duration`, `Degradation in trace severity ratio`, `Abnormal rise in exception volume`, `Abnormal rise in daily data volume`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Do emails get sent to subscription owners? Defaults to `true`.
        #[builder(into, default)]
        pub send_emails_to_subscription_owners: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct SmartDetectionRuleResult {
        /// Specifies a list of additional recipients that will be sent emails on this Application Insights Smart Detection Rule.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        pub additional_email_recipients: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Application Insights component on which the Smart Detection Rule operates. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_wasm_rust::Output<String>,
        /// Is the Application Insights Smart Detection Rule enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Application Insights Smart Detection Rule. Valid values include `Slow page load time`, `Slow server response time`, `Potential memory leak detected`, `Potential security issue detected`, `Long dependency duration`, `Degradation in server response time`, `Degradation in dependency duration`, `Degradation in trace severity ratio`, `Abnormal rise in exception volume`, `Abnormal rise in daily data volume`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Do emails get sent to subscription owners? Defaults to `true`.
        pub send_emails_to_subscription_owners: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SmartDetectionRuleArgs,
    ) -> SmartDetectionRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_email_recipients_binding = args
            .additional_email_recipients
            .get_output(context)
            .get_inner();
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let send_emails_to_subscription_owners_binding = args
            .send_emails_to_subscription_owners
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/smartDetectionRule:SmartDetectionRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalEmailRecipients".into(),
                    value: &additional_email_recipients_binding,
                },
                register_interface::ObjectField {
                    name: "applicationInsightsId".into(),
                    value: &application_insights_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sendEmailsToSubscriptionOwners".into(),
                    value: &send_emails_to_subscription_owners_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalEmailRecipients".into(),
                },
                register_interface::ResultField {
                    name: "applicationInsightsId".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sendEmailsToSubscriptionOwners".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SmartDetectionRuleResult {
            additional_email_recipients: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalEmailRecipients").unwrap(),
            ),
            application_insights_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationInsightsId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            send_emails_to_subscription_owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendEmailsToSubscriptionOwners").unwrap(),
            ),
        }
    }
}
