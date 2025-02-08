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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventPermissionArgs {
        /// The action that you are enabling the other account to perform. Defaults to `events:PutEvents`.
        #[builder(into, default)]
        pub action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block to limit the event bus permissions you are granting to only accounts that fulfill the condition. Specified below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventPermissionCondition>,
        >,
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        #[builder(into, default)]
        pub event_bus_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify `*` to permit any account to put events to your default event bus, optionally limited by `condition`.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier string for the external account that you are granting permissions to.
        #[builder(into)]
        pub statement_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EventPermissionResult {
        /// The action that you are enabling the other account to perform. Defaults to `events:PutEvents`.
        pub action: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block to limit the event bus permissions you are granting to only accounts that fulfill the condition. Specified below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventPermissionCondition>,
        >,
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        pub event_bus_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify `*` to permit any account to put events to your default event bus, optionally limited by `condition`.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// An identifier string for the external account that you are granting permissions to.
        pub statement_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EventPermissionArgs,
    ) -> EventPermissionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let condition_binding = args.condition.get_output(context).get_inner();
        let event_bus_name_binding = args.event_bus_name.get_output(context).get_inner();
        let principal_binding = args.principal.get_output(context).get_inner();
        let statement_id_binding = args.statement_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventPermission:EventPermission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventPermissionResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            event_bus_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventBusName"),
            ),
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            statement_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statementId"),
            ),
        }
    }
}
