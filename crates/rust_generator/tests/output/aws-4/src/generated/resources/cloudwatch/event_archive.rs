/// Provides an EventBridge event archive resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let order = event_bus::create(
///         "order",
///         EventBusArgs::builder().name("orders").build_struct(),
///     );
///     let orderEventArchive = event_archive::create(
///         "orderEventArchive",
///         EventArchiveArgs::builder()
///             .event_source_arn("${order.arn}")
///             .name("order-archive")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example all optional arguments
///
/// ```yaml
/// resources:
///   order:
///     type: aws:cloudwatch:EventBus
///     properties:
///       name: orders
///   orderEventArchive:
///     type: aws:cloudwatch:EventArchive
///     name: order
///     properties:
///       name: order-archive
///       description: Archived events from order service
///       eventSourceArn: ${order.arn}
///       retentionDays: 7
///       eventPattern:
///         fn::toJSON:
///           source:
///             - company.team.order
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an EventBridge archive using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventArchive:EventArchive imported_event_archive order-archive
/// ```
pub mod event_archive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventArchiveArgs {
        /// The description of the new event archive.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        #[builder(into, default)]
        pub event_pattern: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        #[builder(into)]
        pub event_source_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EventArchiveResult {
        /// The Amazon Resource Name (ARN) of the event archive.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the new event archive.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        pub event_pattern: pulumi_gestalt_rust::Output<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        pub event_source_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EventArchiveArgs,
    ) -> EventArchiveResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let event_pattern_binding = args.event_pattern.get_output(context).get_inner();
        let event_source_arn_binding = args
            .event_source_arn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let retention_days_binding = args.retention_days.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventArchive:EventArchive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventPattern".into(),
                    value: &event_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "eventSourceArn".into(),
                    value: &event_source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventArchiveResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            event_pattern: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventPattern"),
            ),
            event_source_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventSourceArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionDays"),
            ),
        }
    }
}
