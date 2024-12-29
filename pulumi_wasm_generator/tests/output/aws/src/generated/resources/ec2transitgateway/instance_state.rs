/// Provides an EC2 instance state resource. This allows managing an instance power state.
///
/// > **NOTE on Instance State Management:** AWS does not currently have an EC2 API operation to determine an instance has finished processing user data. As a result, this resource can interfere with user data processing. For example, this resource may stop an instance while the user data script is in mid run.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${ubuntu.id}
///       instanceType: t3.micro
///       tags:
///         Name: HelloWorld
///   testInstanceState:
///     type: aws:ec2transitgateway:InstanceState
///     name: test
///     properties:
///       instanceId: ${test.id}
///       state: stopped
/// variables:
///   ubuntu:
///     fn::invoke:
///       Function: aws:ec2:getAmi
///       Arguments:
///         mostRecent: true
///         filters:
///           - name: name
///             values:
///               - ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*
///           - name: virtualization-type
///             values:
///               - hvm
///         owners:
///           - '099720109477'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_instance_state` using the `instance_id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/instanceState:InstanceState test i-02cae6557dfcf2f96
/// ```
pub mod instance_state {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceStateArgs {
        /// Whether to request a forced stop when `state` is `stopped`. Otherwise (_i.e._, `state` is `running`), ignored. When an instance is forced to stop, it does not flush file system caches or file system metadata, and you must subsequently perform file system check and repair. Not recommended for Windows instances. Defaults to `false`.
        #[builder(into, default)]
        pub force: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// State of the instance. Valid values are `stopped`, `running`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub state: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceStateResult {
        /// Whether to request a forced stop when `state` is `stopped`. Otherwise (_i.e._, `state` is `running`), ignored. When an instance is forced to stop, it does not flush file system caches or file system metadata, and you must subsequently perform file system check and repair. Not recommended for Windows instances. Defaults to `false`.
        pub force: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// State of the instance. Valid values are `stopped`, `running`.
        ///
        /// The following arguments are optional:
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceStateArgs) -> InstanceStateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let force_binding = args.force.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let state_binding = args.state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/instanceState:InstanceState".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "force".into(),
                    value: &force_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "force".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceStateResult {
            force: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("force").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
