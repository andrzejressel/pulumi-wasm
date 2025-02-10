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
        context: &pulumi_gestalt_rust::Context,
        args: GetLoadBalancerPoolsArgs,
    ) -> GetLoadBalancerPoolsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let pools_binding = args.pools.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getLoadBalancerPools:getLoadBalancerPools".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pools".into(),
                    value: pools_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLoadBalancerPoolsResult {
            account_id: o.get_field("accountId"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            pools: o.get_field("pools"),
        }
    }
}
