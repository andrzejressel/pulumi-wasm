/// Provides a resource to create an EventBridge resource policy to support cross-account events.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// > **Note:** The EventBridge bus policy resource  (`aws.cloudwatch.EventBusPolicy`) is incompatible with the EventBridge permission resource (`aws.cloudwatch.EventPermission`) and will overwrite permissions.
///
/// ## Example Usage
///
/// ### Account Access
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["events:PutEvents",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["123456789012",]). type ("AWS").build_struct(),])
///                     .resources(vec!["arn:aws:events:eu-west-1:123456789012:event-bus/default",])
///                     .sid("DevAccountAccess").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testEventBusPolicy = event_bus_policy::create(
///         "testEventBusPolicy",
///         EventBusPolicyArgs::builder()
///             .event_bus_name("${testAwsCloudwatchEventBus.name}")
///             .policy("${test.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Access
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["events:DescribeRule", "events:ListRules",
///                     "events:ListTargetsByRule", "events:ListTagsForResource",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["${example.id}",])
///                     .variable("aws:PrincipalOrgID").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["arn:aws:events:eu-west-1:123456789012:rule/*",
///                     "arn:aws:events:eu-west-1:123456789012:event-bus/default",])
///                     .sid("OrganizationAccess").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testEventBusPolicy = event_bus_policy::create(
///         "testEventBusPolicy",
///         EventBusPolicyArgs::builder()
///             .event_bus_name("${testAwsCloudwatchEventBus.name}")
///             .policy("${test.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple Statements
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["events:PutEvents",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["123456789012",]). type ("AWS").build_struct(),])
///                     .resources(vec!["arn:aws:events:eu-west-1:123456789012:event-bus/default",])
///                     .sid("DevAccountAccess").build_struct(),
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["events:DescribeRule", "events:ListRules",
///                     "events:ListTargetsByRule", "events:ListTagsForResource",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["${example.id}",])
///                     .variable("aws:PrincipalOrgID").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["arn:aws:events:eu-west-1:123456789012:rule/*",
///                     "arn:aws:events:eu-west-1:123456789012:event-bus/default",])
///                     .sid("OrganizationAccess").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testEventBusPolicy = event_bus_policy::create(
///         "testEventBusPolicy",
///         EventBusPolicyArgs::builder()
///             .event_bus_name("${testAwsCloudwatchEventBus.name}")
///             .policy("${test.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an EventBridge policy using the `event_bus_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventBusPolicy:EventBusPolicy DevAccountAccess example-event-bus
/// ```
pub mod event_bus_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventBusPolicyArgs {
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        #[builder(into, default)]
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The text of the policy.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EventBusPolicyResult {
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The text of the policy.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventBusPolicyArgs) -> EventBusPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let event_bus_name_binding = args.event_bus_name.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventBusPolicy:EventBusPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventBusName".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventBusPolicyResult {
            event_bus_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventBusName").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}