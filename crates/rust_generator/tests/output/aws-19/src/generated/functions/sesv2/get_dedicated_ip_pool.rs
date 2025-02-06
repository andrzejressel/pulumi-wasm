pub mod get_dedicated_ip_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDedicatedIpPoolArgs {
        /// Name of the dedicated IP pool.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags attached to the pool.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDedicatedIpPoolResult {
        /// ARN of the Dedicated IP Pool.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of objects describing the pool's dedicated IP's. See `dedicated_ips`.
        pub dedicated_ips: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetDedicatedIpPoolDedicatedIp>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// (Optional) IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`.
        pub scaling_mode: pulumi_gestalt_rust::Output<String>,
        /// A map of tags attached to the pool.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDedicatedIpPoolArgs,
    ) -> GetDedicatedIpPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let pool_name_binding = args.pool_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getDedicatedIpPool:getDedicatedIpPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDedicatedIpPoolResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            dedicated_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dedicatedIps"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            pool_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("poolName"),
            ),
            scaling_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingMode"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
