/// Manages an EC2 Transit Gateway Policy Table.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:PolicyTable
///     properties:
///       transitGatewayId: ${exampleAwsEc2TransitGateway.id}
///       tags:
///         Name: Example Policy Table
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_policy_table` using the EC2 Transit Gateway Policy Table identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/policyTable:PolicyTable example tgw-rtb-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTableArgs {
        /// Key-value tags for the EC2 Transit Gateway Policy Table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// EC2 Transit Gateway identifier.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTableResult {
        /// EC2 Transit Gateway Policy Table Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The state of the EC2 Transit Gateway Policy Table.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Policy Table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// EC2 Transit Gateway identifier.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTableArgs,
    ) -> PolicyTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/policyTable:PolicyTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: transit_gateway_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyTableResult {
            arn: o.get_field("arn"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_id: o.get_field("transitGatewayId"),
        }
    }
}
