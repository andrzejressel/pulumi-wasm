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
pub mod response_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponsePlanArgs {
        #[builder(into, default)]
        pub action: pulumi_wasm_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanAction>,
        >,
        #[builder(into, default)]
        pub chat_channels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub engagements: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into)]
        pub incident_template: pulumi_wasm_rust::Output<
            super::super::types::ssmincidents::ResponsePlanIncidentTemplate,
        >,
        #[builder(into, default)]
        pub integration: pulumi_wasm_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanIntegration>,
        >,
        /// The name of the response plan.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResponsePlanResult {
        pub action: pulumi_wasm_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanAction>,
        >,
        /// The ARN of the response plan.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub chat_channels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub engagements: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub incident_template: pulumi_wasm_rust::Output<
            super::super::types::ssmincidents::ResponsePlanIncidentTemplate,
        >,
        pub integration: pulumi_wasm_rust::Output<
            Option<super::super::types::ssmincidents::ResponsePlanIntegration>,
        >,
        /// The name of the response plan.
        pub name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResponsePlanArgs) -> ResponsePlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let chat_channels_binding = args.chat_channels.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let engagements_binding = args.engagements.get_inner();
        let incident_template_binding = args.incident_template.get_inner();
        let integration_binding = args.integration.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssmincidents/responsePlan:ResponsePlan".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "chatChannels".into(),
                    value: &chat_channels_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "engagements".into(),
                    value: &engagements_binding,
                },
                register_interface::ObjectField {
                    name: "incidentTemplate".into(),
                    value: &incident_template_binding,
                },
                register_interface::ObjectField {
                    name: "integration".into(),
                    value: &integration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "chatChannels".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "engagements".into(),
                },
                register_interface::ResultField {
                    name: "incidentTemplate".into(),
                },
                register_interface::ResultField {
                    name: "integration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResponsePlanResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            chat_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chatChannels").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            engagements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engagements").unwrap(),
            ),
            incident_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("incidentTemplate").unwrap(),
            ),
            integration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
