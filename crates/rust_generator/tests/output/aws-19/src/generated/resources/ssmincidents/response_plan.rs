/// Provides a resource to manage response plans in AWS Systems Manager Incident Manager.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssmincidents:ResponsePlan
///     properties:
///       name: name
///       incidentTemplate:
///         title: title
///         impact: '3'
///       tags:
///         key: value
///     options:
///       dependsOn:
///         - ${exampleAwsSsmincidentsReplicationSet}
/// ```
///
/// ### Usage With All Fields
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssmincidents:ResponsePlan
///     properties:
///       name: name
///       incidentTemplate:
///         title: title
///         impact: '3'
///         dedupeString: dedupe
///         incidentTags:
///           key: value
///         notificationTargets:
///           - snsTopicArn: ${example1.arn}
///           - snsTopicArn: ${example2.arn}
///         summary: summary
///       displayName: display name
///       chatChannels:
///         - ${topic.arn}
///       engagements:
///         - arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1
///       action:
///         ssmAutomations:
///           - documentName: ${document1.name}
///             roleArn: ${role1.arn}
///             documentVersion: version1
///             targetAccount: RESPONSE_PLAN_OWNER_ACCOUNT
///             parameters:
///               - name: key
///                 values:
///                   - value1
///                   - value2
///               - name: foo
///                 values:
///                   - bar
///             dynamicParameters:
///               someKey: INVOLVED_RESOURCES
///               anotherKey: INCIDENT_RECORD_ARN
///       integration:
///         pagerduties:
///           - name: pagerdutyIntergration
///             serviceId: example
///             secretId: example
///       tags:
///         key: value
///     options:
///       dependsOn:
///         - ${exampleAwsSsmincidentsReplicationSet}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Incident Manager response plan using the response plan ARN. You can find the response plan ARN in the AWS Management Console. For example:
///
/// ```sh
/// $ pulumi import aws:ssmincidents/responsePlan:ResponsePlan responsePlanName ARNValue
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod response_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponsePlanArgs {
        #[builder(into, default)]
        pub action: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ssmincidents::ResponsePlanAction>,
        >,
        #[builder(into, default)]
        pub chat_channels: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub engagements: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into)]
        pub incident_template: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ssmincidents::ResponsePlanIncidentTemplate,
        >,
        #[builder(into, default)]
        pub integration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ssmincidents::ResponsePlanIntegration>,
        >,
        /// The name of the response plan.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResponsePlanResult {
        pub action: pulumi_gestalt_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanAction>,
        >,
        /// The ARN of the response plan.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub chat_channels: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub engagements: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub incident_template: pulumi_gestalt_rust::Output<
            super::super::types::ssmincidents::ResponsePlanIncidentTemplate,
        >,
        pub integration: pulumi_gestalt_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanIntegration>,
        >,
        /// The name of the response plan.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResponsePlanArgs,
    ) -> ResponsePlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let chat_channels_binding = args.chat_channels.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let engagements_binding = args.engagements.get_output(context);
        let incident_template_binding = args.incident_template.get_output(context);
        let integration_binding = args.integration.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssmincidents/responsePlan:ResponsePlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "chatChannels".into(),
                    value: chat_channels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engagements".into(),
                    value: engagements_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "incidentTemplate".into(),
                    value: incident_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integration".into(),
                    value: integration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResponsePlanResult {
            action: o.get_field("action"),
            arn: o.get_field("arn"),
            chat_channels: o.get_field("chatChannels"),
            display_name: o.get_field("displayName"),
            engagements: o.get_field("engagements"),
            incident_template: o.get_field("incidentTemplate"),
            integration: o.get_field("integration"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
