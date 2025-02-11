#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetIpamPreviewNextCidrArgs,
    ) -> GetIpamPreviewNextCidrResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let netmask_length_binding = args.netmask_length.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getIpamPreviewNextCidr:getIpamPreviewNextCidr".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disallowedCidrs".into(),
                    value: &disallowed_cidrs_binding.drop_type(),
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
        let o = context.invoke_resource(request);
        GetIpamPreviewNextCidrResult {
            cidr: o.get_field("cidr"),
            disallowed_cidrs: o.get_field("disallowedCidrs"),
            id: o.get_field("id"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            netmask_length: o.get_field("netmaskLength"),
        }
    }
}
