#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_iam_pool_cidrs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIamPoolCidrsArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolCidrsFilter>>,
        >,
        /// ID of the IPAM pool you would like the list of provisioned CIDRs.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpcIamPoolCidrsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolCidrsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The CIDRs provisioned into the IPAM pool, described below.
        pub ipam_pool_cidrs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcIamPoolCidrsIpamPoolCidr>,
        >,
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcIamPoolCidrsArgs,
    ) -> GetVpcIamPoolCidrsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let ipam_pool_id_binding_1 = args.ipam_pool_id.get_output(context);
        let ipam_pool_id_binding = ipam_pool_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcIamPoolCidrs:getVpcIamPoolCidrs".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcIamPoolCidrsResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipam_pool_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolCidrs"),
            ),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
        }
    }
}
