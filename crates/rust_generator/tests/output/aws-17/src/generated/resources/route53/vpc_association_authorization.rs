/// Authorizes a VPC in a different account to be associated with a local Route53 Hosted Zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alternate = vpc::create(
///         "alternate",
///         VpcArgs::builder()
///             .cidr_block("10.7.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let example = vpc::create(
///         "example",
///         VpcArgs::builder()
///             .cidr_block("10.6.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let exampleVpcAssociationAuthorization = vpc_association_authorization::create(
///         "exampleVpcAssociationAuthorization",
///         VpcAssociationAuthorizationArgs::builder()
///             .vpc_id("${alternate.id}")
///             .zone_id("${exampleZone.id}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("example.com")
///             .vpcs(vec![ZoneVpc::builder().vpcId("${example.id}").build_struct(),])
///             .build_struct(),
///     );
///     let exampleZoneAssociation = zone_association::create(
///         "exampleZoneAssociation",
///         ZoneAssociationArgs::builder()
///             .vpc_id("${exampleVpcAssociationAuthorization.vpcId}")
///             .zone_id("${exampleVpcAssociationAuthorization.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 VPC Association Authorizations using the Hosted Zone ID and VPC ID, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:route53/vpcAssociationAuthorization:VpcAssociationAuthorization example Z123456ABCDEFG:vpc-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_association_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAssociationAuthorizationArgs {
        /// The VPC to authorize for association with the private hosted zone.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VPC's region. Defaults to the region of the AWS provider.
        #[builder(into, default)]
        pub vpc_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the private hosted zone that you want to authorize associating a VPC with.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcAssociationAuthorizationResult {
        /// The VPC to authorize for association with the private hosted zone.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC's region. Defaults to the region of the AWS provider.
        pub vpc_region: pulumi_gestalt_rust::Output<String>,
        /// The ID of the private hosted zone that you want to authorize associating a VPC with.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcAssociationAuthorizationArgs,
    ) -> VpcAssociationAuthorizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let vpc_id_binding = args.vpc_id.get_output(context);
        let vpc_region_binding = args.vpc_region.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/vpcAssociationAuthorization:VpcAssociationAuthorization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcRegion".into(),
                    value: &vpc_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcAssociationAuthorizationResult {
            vpc_id: o.get_field("vpcId"),
            vpc_region: o.get_field("vpcRegion"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
