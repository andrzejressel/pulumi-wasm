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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_state {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceStateArgs {
        /// Whether to request a forced stop when `state` is `stopped`. Otherwise (_i.e._, `state` is `running`), ignored. When an instance is forced to stop, it does not flush file system caches or file system metadata, and you must subsequently perform file system check and repair. Not recommended for Windows instances. Defaults to `false`.
        #[builder(into, default)]
        pub force: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of the instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// State of the instance. Valid values are `stopped`, `running`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceStateResult {
        /// Whether to request a forced stop when `state` is `stopped`. Otherwise (_i.e._, `state` is `running`), ignored. When an instance is forced to stop, it does not flush file system caches or file system metadata, and you must subsequently perform file system check and repair. Not recommended for Windows instances. Defaults to `false`.
        pub force: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// State of the instance. Valid values are `stopped`, `running`.
        ///
        /// The following arguments are optional:
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceStateArgs,
    ) -> InstanceStateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let force_binding = args.force.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/instanceState:InstanceState".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "force".into(),
                    value: force_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceStateResult {
            force: o.get_field("force"),
            instance_id: o.get_field("instanceId"),
            state: o.get_field("state"),
        }
    }
}
