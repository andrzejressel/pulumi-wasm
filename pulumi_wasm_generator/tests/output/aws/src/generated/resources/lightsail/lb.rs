/// Creates a Lightsail load balancer resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Lb
///     properties:
///       name: test-load-balancer
///       healthCheckPath: /
///       instancePort: '80'
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb` using the name attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lb:Lb test example-load-balancer
/// ```
pub mod lb {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbArgs {
        /// The health check path of the load balancer. Default value "/".
        #[builder(into, default)]
        pub health_check_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The instance port the load balancer will connect.
        #[builder(into)]
        pub instance_port: pulumi_wasm_rust::Output<i32>,
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Lightsail load balancer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LbResult {
        /// The ARN of the Lightsail load balancer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the load balancer was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The DNS name of the load balancer.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// The health check path of the load balancer. Default value "/".
        pub health_check_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The instance port the load balancer will connect.
        pub instance_port: pulumi_wasm_rust::Output<i32>,
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Lightsail load balancer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The protocol of the load balancer.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The public ports of the load balancer.
        pub public_ports: pulumi_wasm_rust::Output<Vec<i32>>,
        /// The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: LbArgs) -> LbResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let health_check_path_binding = args.health_check_path.get_inner();
        let instance_port_binding = args.instance_port.get_inner();
        let ip_address_type_binding = args.ip_address_type.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lb:Lb".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "healthCheckPath".into(),
                    value: &health_check_path_binding,
                },
                register_interface::ObjectField {
                    name: "instancePort".into(),
                    value: &instance_port_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
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
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckPath".into(),
                },
                register_interface::ResultField {
                    name: "instancePort".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "publicPorts".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
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
        LbResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            health_check_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckPath").unwrap(),
            ),
            instance_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instancePort").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            public_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicPorts").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
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