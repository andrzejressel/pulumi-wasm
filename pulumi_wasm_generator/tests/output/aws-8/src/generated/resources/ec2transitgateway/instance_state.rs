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
///       function: aws:ec2:getAmi
///       arguments:
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceStateArgs {
        /// Whether to request a forced stop when `state` is `stopped`. Otherwise (_i.e._, `state` is `running`), ignored. When an instance is forced to stop, it does not flush file system caches or file system metadata, and you must subsequently perform file system check and repair. Not recommended for Windows instances. Defaults to `false`.
        #[builder(into, default)]
        pub force: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ID of the instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// State of the instance. Valid values are `stopped`, `running`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub state: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceStateArgs,
    ) -> InstanceStateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let force_binding = args.force.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/instanceState:InstanceState".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceStateResult {
            force: pulumi_wasm_rust::__private::into_domain(o.extract_field("force")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
