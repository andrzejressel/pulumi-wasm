#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetDedicatedIpPoolArgs,
    ) -> GetDedicatedIpPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pool_name_binding = args.pool_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sesv2/getDedicatedIpPool:getDedicatedIpPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDedicatedIpPoolResult {
            arn: o.get_field("arn"),
            dedicated_ips: o.get_field("dedicatedIps"),
            id: o.get_field("id"),
            pool_name: o.get_field("poolName"),
            scaling_mode: o.get_field("scalingMode"),
            tags: o.get_field("tags"),
        }
    }
}
