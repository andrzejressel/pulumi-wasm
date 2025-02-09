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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lb {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbArgs {
        /// The health check path of the load balancer. Default value "/".
        #[builder(into, default)]
        pub health_check_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The instance port the load balancer will connect.
        #[builder(into)]
        pub instance_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Lightsail load balancer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LbResult {
        /// The ARN of the Lightsail load balancer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the load balancer was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The DNS name of the load balancer.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// The health check path of the load balancer. Default value "/".
        pub health_check_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The instance port the load balancer will connect.
        pub instance_port: pulumi_gestalt_rust::Output<i32>,
        pub ip_address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Lightsail load balancer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The protocol of the load balancer.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The public ports of the load balancer.
        pub public_ports: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbArgs,
    ) -> LbResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let health_check_path_binding = args.health_check_path.get_output(context);
        let instance_port_binding = args.instance_port.get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lb:Lb".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckPath".into(),
                    value: health_check_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instancePort".into(),
                    value: instance_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: ip_address_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LbResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            dns_name: o.get_field("dnsName"),
            health_check_path: o.get_field("healthCheckPath"),
            instance_port: o.get_field("instancePort"),
            ip_address_type: o.get_field("ipAddressType"),
            name: o.get_field("name"),
            protocol: o.get_field("protocol"),
            public_ports: o.get_field("publicPorts"),
            support_code: o.get_field("supportCode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
