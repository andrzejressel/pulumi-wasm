/// Provisions a CIDR from an IPAM address pool.
///
/// > **NOTE:** Provisioning Public IPv4 or Public IPv6 require [steps outside the scope of this resource](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html#prepare-for-byoip). The resource accepts `message` and `signature` as part of the `cidr_authorization_context` attribute but those must be generated ahead of time. Public IPv6 CIDRs that are provisioned into a Pool with `publicly_advertisable = true` and all public IPv4 CIDRs also require creating a Route Origin Authorization (ROA) object in your Regional Internet Registry (RIR).
///
/// > **NOTE:** In order to deprovision CIDRs all Allocations must be released. Allocations created by a VPC take up to 30 minutes to be released. However, for IPAM to properly manage the removal of allocation records created by VPCs and other resources, you must [grant it permissions](https://docs.aws.amazon.com/vpc/latest/ipam/choose-single-user-or-orgs-ipam.html) in
/// either a single account or organizationally. If you are unable to deprovision a cidr after waiting over 30 minutes, you may be missing the Service Linked Role.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${example.privateDefaultScopeId}
///       locale: ${current.name}
///   exampleVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: example
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/16
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// Provision Public IPv6 Pool CIDRs:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   ipv6TestPublic:
///     type: aws:ec2:VpcIpamPool
///     name: ipv6_test_public
///     properties:
///       addressFamily: ipv6
///       ipamScopeId: ${example.publicDefaultScopeId}
///       locale: us-east-1
///       description: public ipv6
///       publiclyAdvertisable: false
///       publicIpSource: amazon
///       awsService: ec2
///   ipv6TestPublicVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: ipv6_test_public
///     properties:
///       ipamPoolId: ${ipv6TestPublic.id}
///       netmaskLength: 52
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the `<cidr>_<ipam-pool-id>`. For example:
///
/// __NOTE:__ Do not use the IPAM Pool Cidr ID as this was introduced after the resource already existed.
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamPoolCidr:VpcIpamPoolCidr example 172.20.0.0/24_ipam-pool-0e634f5a1517cccdc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_pool_cidr {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrArgs {
        /// The CIDR you want to assign to the pool. Conflicts with `netmask_length`.
        #[builder(into, default)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP. This is not stored in the state file. See cidr_authorization_context for more information.
        #[builder(into, default)]
        pub cidr_authorization_context: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcIpamPoolCidrCidrAuthorizationContext>,
        >,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If provided, the cidr provisioned into the specified pool will be the next available cidr given this declared netmask length. Conflicts with `cidr`.
        #[builder(into, default)]
        pub netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrResult {
        /// The CIDR you want to assign to the pool. Conflicts with `netmask_length`.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP. This is not stored in the state file. See cidr_authorization_context for more information.
        pub cidr_authorization_context: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::VpcIpamPoolCidrCidrAuthorizationContext>,
        >,
        /// The unique ID generated by AWS for the pool cidr. Typically this is the resource `id` but this attribute was added to the API calls after the fact and is therefore not used as the resource id.
        pub ipam_pool_cidr_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// If provided, the cidr provisioned into the specified pool will be the next available cidr given this declared netmask length. Conflicts with `cidr`.
        pub netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamPoolCidrArgs,
    ) -> VpcIpamPoolCidrResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_binding = args.cidr.get_output(context);
        let cidr_authorization_context_binding = args
            .cidr_authorization_context
            .get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let netmask_length_binding = args.netmask_length.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPoolCidr:VpcIpamPoolCidr".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrAuthorizationContext".into(),
                    value: &cidr_authorization_context_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netmaskLength".into(),
                    value: &netmask_length_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamPoolCidrResult {
            cidr: o.get_field("cidr"),
            cidr_authorization_context: o.get_field("cidrAuthorizationContext"),
            ipam_pool_cidr_id: o.get_field("ipamPoolCidrId"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            netmask_length: o.get_field("netmaskLength"),
        }
    }
}
