#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_load_balancer_pools {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLoadBalancerPoolsArgs {
        /// The account identifier to target for the datasource lookups.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::GetLoadBalancerPoolsFilter>,
        >,
        /// A list of Load Balancer Pools details.
        #[builder(into, default)]
        pub pools: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::GetLoadBalancerPoolsPool>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLoadBalancerPoolsResult {
        /// The account identifier to target for the datasource lookups.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::GetLoadBalancerPoolsFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of Load Balancer Pools details.
        pub pools: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetLoadBalancerPoolsPool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLoadBalancerPoolsArgs,
    ) -> GetLoadBalancerPoolsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let pools_binding = args.pools.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getLoadBalancerPools:getLoadBalancerPools".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "pools".into(),
                    value: &pools_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLoadBalancerPoolsResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            pools: pulumi_gestalt_rust::__private::into_domain(o.extract_field("pools")),
        }
    }
}
