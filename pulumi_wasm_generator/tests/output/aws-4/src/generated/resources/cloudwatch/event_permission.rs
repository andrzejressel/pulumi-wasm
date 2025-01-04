/// Provides a resource to create an EventBridge permission to support cross-account events in the current account default event bus.
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
///     let devAccountAccess = event_permission::create(
///         "devAccountAccess",
///         EventPermissionArgs::builder()
///             .principal("123456789012")
///             .statement_id("DevAccountAccess")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Access
///
/// ```yaml
/// resources:
///   organizationAccess:
///     type: aws:cloudwatch:EventPermission
///     name: OrganizationAccess
///     properties:
///       principal: '*'
///       statementId: OrganizationAccess
///       condition:
///         key: aws:PrincipalOrgID
///         type: StringEquals
///         value: ${example.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge permissions using the `event_bus_name/statement_id` (if you omit `event_bus_name`, the `default` event bus will be used). For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventPermission:EventPermission DevAccountAccess example-event-bus/DevAccountAccess
/// ```
pub mod event_permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventPermissionArgs {
        /// The action that you are enabling the other account to perform. Defaults to `events:PutEvents`.
        #[builder(into, default)]
        pub action: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to limit the event bus permissions you are granting to only accounts that fulfill the condition. Specified below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventPermissionCondition>,
        >,
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        #[builder(into, default)]
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify `*` to permit any account to put events to your default event bus, optionally limited by `condition`.
        #[builder(into)]
        pub principal: pulumi_wasm_rust::Output<String>,
        /// An identifier string for the external account that you are granting permissions to.
        #[builder(into)]
        pub statement_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EventPermissionResult {
        /// The action that you are enabling the other account to perform. Defaults to `events:PutEvents`.
        pub action: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to limit the event bus permissions you are granting to only accounts that fulfill the condition. Specified below.
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventPermissionCondition>,
        >,
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify `*` to permit any account to put events to your default event bus, optionally limited by `condition`.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// An identifier string for the external account that you are granting permissions to.
        pub statement_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventPermissionArgs) -> EventPermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let condition_binding = args.condition.get_inner();
        let event_bus_name_binding = args.event_bus_name.get_inner();
        let principal_binding = args.principal.get_inner();
        let statement_id_binding = args.statement_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventPermission:EventPermission".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "statementId".into(),
                    value: &statement_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "eventBusName".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "statementId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventPermissionResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            event_bus_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventBusName").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            statement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statementId").unwrap(),
            ),
        }
    }
}
