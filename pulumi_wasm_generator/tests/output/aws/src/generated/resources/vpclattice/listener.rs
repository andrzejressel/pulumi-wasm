/// Resource for managing an AWS VPC Lattice Listener.
///
/// ## Example Usage
///
/// ### Fixed response action
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder().name("example").build_struct(),
///     );
///     let exampleListener = listener::create(
///         "exampleListener",
///         ListenerArgs::builder()
///             .default_action(
///                 ListenerDefaultAction::builder()
///                     .fixedResponse(
///                         ListenerDefaultActionFixedResponse::builder()
///                             .statusCode(404)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .protocol("HTTPS")
///             .service_identifier("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Forward action
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder().name("example").build_struct(),
///     );
///     let exampleListener = listener::create(
///         "exampleListener",
///         ListenerArgs::builder()
///             .default_action(
///                 ListenerDefaultAction::builder()
///                     .forwards(
///                         vec![
///                             ListenerDefaultActionForward::builder()
///                             .targetGroups(vec![ListenerDefaultActionForwardTargetGroup::builder()
///                             .targetGroupIdentifier("${exampleTargetGroup.id}")
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .protocol("HTTP")
///             .service_identifier("${example.id}")
///             .build_struct(),
///     );
///     let exampleTargetGroup = target_group::create(
///         "exampleTargetGroup",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(80)
///                     .protocol("HTTP")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example-target-group-1")
///             .type_("INSTANCE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Forward action with weighted target groups
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder().name("example").build_struct(),
///     );
///     let example1 = target_group::create(
///         "example1",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(80)
///                     .protocol("HTTP")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example-target-group-1")
///             .type_("INSTANCE")
///             .build_struct(),
///     );
///     let example2 = target_group::create(
///         "example2",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(8080)
///                     .protocol("HTTP")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example-target-group-2")
///             .type_("INSTANCE")
///             .build_struct(),
///     );
///     let exampleListener = listener::create(
///         "exampleListener",
///         ListenerArgs::builder()
///             .default_action(
///                 ListenerDefaultAction::builder()
///                     .forwards(
///                         vec![
///                             ListenerDefaultActionForward::builder()
///                             .targetGroups(vec![ListenerDefaultActionForwardTargetGroup::builder()
///                             .targetGroupIdentifier("${example1.id}").weight(80)
///                             .build_struct(),
///                             ListenerDefaultActionForwardTargetGroup::builder()
///                             .targetGroupIdentifier("${example2.id}").weight(20)
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .protocol("HTTP")
///             .service_identifier("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Listener using the `listener_id` of the listener and the `id` of the VPC Lattice service combined with a `/` character. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/listener:Listener example svc-1a2b3c4d/listener-987654321
/// ```
pub mod listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerArgs {
        /// Default action block for the default listener rule. Default action blocks are defined below.
        #[builder(into)]
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerDefaultAction,
        >,
        /// Name of the listener. A listener name must be unique within a service. Valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Listener port. You can specify a value from 1 to 65535. If `port` is not specified and `protocol` is HTTP, the value will default to 80. If `port` is not specified and `protocol` is HTTPS, the value will default to 443.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Protocol for the listener. Supported values are `HTTP`, `HTTPS` or `TLS_PASSTHROUGH`
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the VPC Lattice service. You must include either the `service_arn` or `service_identifier` arguments.
        #[builder(into, default)]
        pub service_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the VPC Lattice service. You must include either the `service_arn` or `service_identifier` arguments.
        /// > **NOTE:** You must specify one of the following arguments: `service_arn` or `service_identifier`.
        #[builder(into, default)]
        pub service_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerResult {
        /// ARN of the listener.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time that the listener was created, specified in ISO-8601 format.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Default action block for the default listener rule. Default action blocks are defined below.
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerDefaultAction,
        >,
        pub last_updated_at: pulumi_wasm_rust::Output<String>,
        /// Standalone ID of the listener, e.g. `listener-0a1b2c3d4e5f6g`.
        pub listener_id: pulumi_wasm_rust::Output<String>,
        /// Name of the listener. A listener name must be unique within a service. Valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Listener port. You can specify a value from 1 to 65535. If `port` is not specified and `protocol` is HTTP, the value will default to 80. If `port` is not specified and `protocol` is HTTPS, the value will default to 443.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Protocol for the listener. Supported values are `HTTP`, `HTTPS` or `TLS_PASSTHROUGH`
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the VPC Lattice service. You must include either the `service_arn` or `service_identifier` arguments.
        pub service_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the VPC Lattice service. You must include either the `service_arn` or `service_identifier` arguments.
        /// > **NOTE:** You must specify one of the following arguments: `service_arn` or `service_identifier`.
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListenerArgs) -> ListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_action_binding = args.default_action.get_inner();
        let name_binding = args.name.get_inner();
        let port_binding = args.port.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let service_arn_binding = args.service_arn.get_inner();
        let service_identifier_binding = args.service_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/listener:Listener".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "serviceArn".into(),
                    value: &service_arn_binding,
                },
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
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
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "defaultAction".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedAt".into(),
                },
                register_interface::ResultField {
                    name: "listenerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "serviceArn".into(),
                },
                register_interface::ResultField {
                    name: "serviceIdentifier".into(),
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
        ListenerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            default_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAction").unwrap(),
            ),
            last_updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedAt").unwrap(),
            ),
            listener_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            service_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceArn").unwrap(),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceIdentifier").unwrap(),
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