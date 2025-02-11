/// Provides a resource to manage a default route table of a VPC. This resource can manage the default route table of the default or a non-default VPC.
///
/// > **NOTE:** This is an advanced resource with special caveats. Please read this document in its entirety before using this resource. The `aws.ec2.DefaultRouteTable` resource behaves differently from normal resources. This provider does not _create_ this resource but instead attempts to "adopt" it into management. **Do not** use both `aws.ec2.DefaultRouteTable` to manage a default route table **and** `aws.ec2.MainRouteTableAssociation` with the same VPC due to possible route conflicts. See aws.ec2.MainRouteTableAssociation documentation for more details.
///
/// Every VPC has a default route table that can be managed but not destroyed. When the provider first adopts a default route table, it **immediately removes all defined routes**. It then proceeds to create any routes specified in the configuration. This step is required so that only the routes specified in the configuration exist in the default route table.
///
/// For more information, see the Amazon VPC User Guide on [Route Tables](https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Route_Tables.html). For information about managing normal route tables in this provider, see `aws.ec2.RouteTable`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:DefaultRouteTable
///     properties:
///       defaultRouteTableId: ${exampleAwsVpc.defaultRouteTableId}
///       routes:
///         - cidrBlock: 10.0.1.0/24
///           gatewayId: ${exampleAwsInternetGateway.id}
///         - ipv6CidrBlock: ::/0
///           egressOnlyGatewayId: ${exampleAwsEgressOnlyInternetGateway.id}
///       tags:
///         Name: example
/// ```
///
/// To subsequently remove all managed routes:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:DefaultRouteTable
///     properties:
///       defaultRouteTableId: ${exampleAwsVpc.defaultRouteTableId}
///       routes: []
///       tags:
///         Name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Default VPC route tables using the `vpc_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultRouteTable:DefaultRouteTable example vpc-33cc44dd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_route_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultRouteTableArgs {
        /// ID of the default route table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub default_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of virtual gateways for propagation.
        #[builder(into, default)]
        pub propagating_vgws: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of objects. Detailed below
        #[builder(into, default)]
        pub routes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::DefaultRouteTableRoute>>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultRouteTableResult {
        /// The ARN of the route table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the default route table.
        ///
        /// The following arguments are optional:
        pub default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the route table.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// List of virtual gateways for propagation.
        pub propagating_vgws: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of objects. Detailed below
        pub routes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::DefaultRouteTableRoute>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ID of the VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultRouteTableArgs,
    ) -> DefaultRouteTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_route_table_id_binding = args
            .default_route_table_id
            .get_output(context);
        let propagating_vgws_binding = args.propagating_vgws.get_output(context);
        let routes_binding = args.routes.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/defaultRouteTable:DefaultRouteTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRouteTableId".into(),
                    value: &default_route_table_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "propagatingVgws".into(),
                    value: &propagating_vgws_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routes".into(),
                    value: &routes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultRouteTableResult {
            arn: o.get_field("arn"),
            default_route_table_id: o.get_field("defaultRouteTableId"),
            owner_id: o.get_field("ownerId"),
            propagating_vgws: o.get_field("propagatingVgws"),
            routes: o.get_field("routes"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
