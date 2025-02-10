#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hosted_zone_id {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostedZoneIdArgs {
        /// Type of load balancer to create. Possible values are `application` or `network`. The default value is `application`.
        #[builder(into, default)]
        pub load_balancer_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the region whose AWS ELB HostedZoneId is desired.
        /// Defaults to the region from the AWS provider configuration.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHostedZoneIdResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub load_balancer_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHostedZoneIdArgs,
    ) -> GetHostedZoneIdResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let load_balancer_type_binding = args.load_balancer_type.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lb/getHostedZoneId:getHostedZoneId".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerType".into(),
                    value: load_balancer_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetHostedZoneIdResult {
            id: o.get_field("id"),
            load_balancer_type: o.get_field("loadBalancerType"),
            region: o.get_field("region"),
        }
    }
}
