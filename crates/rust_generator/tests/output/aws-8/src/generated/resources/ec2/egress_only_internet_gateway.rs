/// [IPv6 only] Creates an egress-only Internet gateway for your VPC.
/// An egress-only Internet gateway is used to enable outbound communication
/// over IPv6 from instances in your VPC to the Internet, and prevents hosts
/// outside of your VPC from initiating an IPv6 connection with your instance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.1.0.0/16
///       assignGeneratedIpv6CidrBlock: true
///   exampleEgressOnlyInternetGateway:
///     type: aws:ec2:EgressOnlyInternetGateway
///     name: example
///     properties:
///       vpcId: ${example.id}
///       tags:
///         Name: main
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Egress-only Internet gateways using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/egressOnlyInternetGateway:EgressOnlyInternetGateway example eigw-015e0e244e24dfe8a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod egress_only_internet_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EgressOnlyInternetGatewayArgs {
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC ID to create in.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EgressOnlyInternetGatewayResult {
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID to create in.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EgressOnlyInternetGatewayArgs,
    ) -> EgressOnlyInternetGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/egressOnlyInternetGateway:EgressOnlyInternetGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EgressOnlyInternetGatewayResult {
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
