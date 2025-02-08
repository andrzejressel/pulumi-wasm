/// Previews a CIDR from an IPAM address pool. Only works for private IPv4.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamPreviewNextCidr
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       netmaskLength: 28
///       disallowedCidrs:
///         - 172.2.0.0/32
///     options:
///       dependsOn:
///         - ${exampleVpcIpamPoolCidr}
///   exampleVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: example
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${exampleVpcIpam.privateDefaultScopeId}
///       locale: ${current.name}
///   exampleVpcIpam:
///     type: aws:ec2:VpcIpam
///     name: example
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_preview_next_cidr {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrArgs {
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        #[builder(into, default)]
        pub netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrResult {
        /// The previewed CIDR from the pool.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// Exclude a particular CIDR range from being returned by the pool.
        pub disallowed_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        pub netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcIpamPreviewNextCidrArgs,
    ) -> VpcIpamPreviewNextCidrResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disallowed_cidrs_binding = args
            .disallowed_cidrs
            .get_output(context)
            .get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context).get_inner();
        let netmask_length_binding = args.netmask_length.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPreviewNextCidr:VpcIpamPreviewNextCidr".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disallowedCidrs".into(),
                    value: &disallowed_cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "netmaskLength".into(),
                    value: &netmask_length_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamPreviewNextCidrResult {
            cidr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cidr")),
            disallowed_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disallowedCidrs"),
            ),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
            netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netmaskLength"),
            ),
        }
    }
}
