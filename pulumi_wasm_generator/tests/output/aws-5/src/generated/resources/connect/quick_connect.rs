/// Provides an Amazon Connect Quick Connect resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:QuickConnect
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: quick connect phone number
///       quickConnectConfig:
///         quickConnectType: PHONE_NUMBER
///         phoneConfigs:
///           - phoneNumber: '+12345678912'
///       tags:
///         Name: Example Quick Connect
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Quick Connects using the `instance_id` and `quick_connect_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/quickConnect:QuickConnect example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
pub mod quick_connect {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QuickConnectArgs {
        /// Specifies the description of the Quick Connect.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Quick Connect.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
        #[builder(into)]
        pub quick_connect_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::connect::QuickConnectQuickConnectConfig,
        >,
        /// Tags to apply to the Quick Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QuickConnectResult {
        /// The Amazon Resource Name (ARN) of the Quick Connect.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the Quick Connect.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Quick Connect.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
        pub quick_connect_config: pulumi_wasm_rust::Output<
            super::super::types::connect::QuickConnectQuickConnectConfig,
        >,
        /// The identifier for the Quick Connect.
        pub quick_connect_id: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Quick Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: QuickConnectArgs,
    ) -> QuickConnectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let quick_connect_config_binding = args
            .quick_connect_config
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/quickConnect:QuickConnect".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "quickConnectConfig".into(),
                    value: &quick_connect_config_binding,
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
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "quickConnectConfig".into(),
                },
                register_interface::ResultField {
                    name: "quickConnectId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QuickConnectResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            quick_connect_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quickConnectConfig").unwrap(),
            ),
            quick_connect_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quickConnectId").unwrap(),
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
