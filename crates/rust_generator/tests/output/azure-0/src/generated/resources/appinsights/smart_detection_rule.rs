/// Manages an Application Insights Smart Detection Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod smart_detection_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmartDetectionRuleArgs {
        /// Specifies a list of additional recipients that will be sent emails on this Application Insights Smart Detection Rule.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        #[builder(into, default)]
        pub additional_email_recipients: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the Application Insights component on which the Smart Detection Rule operates. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the Application Insights Smart Detection Rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Application Insights Smart Detection Rule. Valid values include `Slow page load time`, `Slow server response time`, `Potential memory leak detected`, `Potential security issue detected`, `Long dependency duration`, `Degradation in server response time`, `Degradation in dependency duration`, `Degradation in trace severity ratio`, `Abnormal rise in exception volume`, `Abnormal rise in daily data volume`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Do emails get sent to subscription owners? Defaults to `true`.
        #[builder(into, default)]
        pub send_emails_to_subscription_owners: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct SmartDetectionRuleResult {
        /// Specifies a list of additional recipients that will be sent emails on this Application Insights Smart Detection Rule.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        pub additional_email_recipients: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the Application Insights component on which the Smart Detection Rule operates. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// Is the Application Insights Smart Detection Rule enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Application Insights Smart Detection Rule. Valid values include `Slow page load time`, `Slow server response time`, `Potential memory leak detected`, `Potential security issue detected`, `Long dependency duration`, `Degradation in server response time`, `Degradation in dependency duration`, `Degradation in trace severity ratio`, `Abnormal rise in exception volume`, `Abnormal rise in daily data volume`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Do emails get sent to subscription owners? Defaults to `true`.
        pub send_emails_to_subscription_owners: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmartDetectionRuleArgs,
    ) -> SmartDetectionRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_email_recipients_binding = args
            .additional_email_recipients
            .get_output(context);
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let send_emails_to_subscription_owners_binding = args
            .send_emails_to_subscription_owners
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/smartDetectionRule:SmartDetectionRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalEmailRecipients".into(),
                    value: additional_email_recipients_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsId".into(),
                    value: application_insights_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendEmailsToSubscriptionOwners".into(),
                    value: send_emails_to_subscription_owners_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SmartDetectionRuleResult {
            additional_email_recipients: o.get_field("additionalEmailRecipients"),
            application_insights_id: o.get_field("applicationInsightsId"),
            enabled: o.get_field("enabled"),
            name: o.get_field("name"),
            send_emails_to_subscription_owners: o
                .get_field("sendEmailsToSubscriptionOwners"),
        }
    }
}
