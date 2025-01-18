/// Attaches a Lightsail Instance to a Lightsail Load Balancer.
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
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///   testLbAttachment:
///     type: aws:lightsail:LbAttachment
///     name: test
///     properties:
///       lbName: ${test.name}
///       instanceName: ${testInstance.name}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///         filters:
///           - name: opt-in-status
///             values:
///               - opt-in-not-required
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_attachment` using the name attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbAttachment:LbAttachment test example-load-balancer,example-instance
/// ```
pub mod lb_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbAttachmentArgs {
        /// The name of the instance to attach to the load balancer.
        #[builder(into)]
        pub instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail load balancer.
        #[builder(into)]
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LbAttachmentResult {
        /// The name of the instance to attach to the load balancer.
        pub instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail load balancer.
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LbAttachmentArgs) -> LbAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_name_binding = args.instance_name.get_inner();
        let lb_name_binding = args.lb_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lbAttachment:LbAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "instanceName".into(),
                },
                register_interface::ResultField {
                    name: "lbName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LbAttachmentResult {
            instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceName").unwrap(),
            ),
            lb_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lbName").unwrap(),
            ),
        }
    }
}
