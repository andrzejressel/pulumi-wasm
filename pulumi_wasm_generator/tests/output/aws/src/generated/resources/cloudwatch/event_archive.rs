/// Provides an EventBridge event archive resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventArchiveArgs {
        /// The description of the new event archive.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        #[builder(into, default)]
        pub event_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        #[builder(into)]
        pub event_source_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EventArchiveResult {
        /// The Amazon Resource Name (ARN) of the event archive.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description of the new event archive.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        pub event_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        pub event_source_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventArchiveArgs) -> EventArchiveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let event_pattern_binding = args.event_pattern.get_inner();
        let event_source_arn_binding = args.event_source_arn.get_inner();
        let name_binding = args.name.get_inner();
        let retention_days_binding = args.retention_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventArchive:EventArchive".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventPattern".into(),
                },
                register_interface::ResultField {
                    name: "eventSourceArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "retentionDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventArchiveResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            event_pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventPattern").unwrap(),
            ),
            event_source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSourceArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
        }
    }
}