#[allow(clippy::doc_lazy_continuation)]
pub mod get_ipam_preview_next_cidr {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpamPreviewNextCidrArgs {
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Netmask length of the CIDR you would like to preview from the IPAM pool.
        #[builder(into, default)]
        pub netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetIpamPreviewNextCidrResult {
        /// Previewed CIDR from the pool.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        pub disallowed_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        pub netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetIpamPreviewNextCidrArgs,
    ) -> GetIpamPreviewNextCidrResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disallowed_cidrs_binding = args
            .disallowed_cidrs
            .get_output(context)
            .get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context).get_inner();
        let netmask_length_binding = args.netmask_length.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getIpamPreviewNextCidr:getIpamPreviewNextCidr".into(),
            version: super::super::super::get_version(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIpamPreviewNextCidrResult {
            cidr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cidr")),
            disallowed_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disallowedCidrs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
            netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netmaskLength"),
            ),
        }
    }
}
