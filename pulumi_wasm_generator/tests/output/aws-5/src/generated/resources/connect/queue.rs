/// Provides an Amazon Connect Queue resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       tags:
///         Name: Example Queue
/// ```
///
/// ### With Quick Connect IDs
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       quickConnectIds:
///         - 12345678-abcd-1234-abcd-123456789012
///       tags:
///         Name: Example Queue with Quick Connect IDs
/// ```
///
/// ### With Outbound Caller Config
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       outboundCallerConfig:
///         outboundCallerIdName: example
///         outboundCallerIdNumberId: 12345678-abcd-1234-abcd-123456789012
///         outboundFlowId: 87654321-defg-1234-defg-987654321234
///       tags:
///         Name: Example Queue with Outbound Caller Config
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Queues using the `instance_id` and `queue_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/queue:Queue example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
pub mod queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// Specifies the description of the Queue.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the identifier of the Hours of Operation.
        #[builder(into)]
        pub hours_of_operation_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        #[builder(into, default)]
        pub max_contacts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Queue.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        #[builder(into, default)]
        pub outbound_caller_config: pulumi_wasm_rust::Output<
            Option<super::super::types::connect::QueueOutboundCallerConfig>,
        >,
        /// Specifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
        #[builder(into, default)]
        pub quick_connect_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The Amazon Resource Name (ARN) of the Queue.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the Queue.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the identifier of the Hours of Operation.
        pub hours_of_operation_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        pub max_contacts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Queue.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        pub outbound_caller_config: pulumi_wasm_rust::Output<
            Option<super::super::types::connect::QueueOutboundCallerConfig>,
        >,
        /// The identifier for the Queue.
        pub queue_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
        pub quick_connect_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: QueueArgs) -> QueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let hours_of_operation_id_binding = args.hours_of_operation_id.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let max_contacts_binding = args.max_contacts.get_inner();
        let name_binding = args.name.get_inner();
        let outbound_caller_config_binding = args.outbound_caller_config.get_inner();
        let quick_connect_ids_binding = args.quick_connect_ids.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hoursOfOperationId".into(),
                    value: &hours_of_operation_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "maxContacts".into(),
                    value: &max_contacts_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outboundCallerConfig".into(),
                    value: &outbound_caller_config_binding,
                },
                register_interface::ObjectField {
                    name: "quickConnectIds".into(),
                    value: &quick_connect_ids_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "hoursOfOperationId".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "maxContacts".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundCallerConfig".into(),
                },
                register_interface::ResultField {
                    name: "queueId".into(),
                },
                register_interface::ResultField {
                    name: "quickConnectIds".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        QueueResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            hours_of_operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hoursOfOperationId").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            max_contacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxContacts").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_caller_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundCallerConfig").unwrap(),
            ),
            queue_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueId").unwrap(),
            ),
            quick_connect_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quickConnectIds").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
