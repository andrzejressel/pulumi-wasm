#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetHostedZoneIdArgs,
    ) -> GetHostedZoneIdResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let load_balancer_type_binding = args
            .load_balancer_type
            .get_output(context)
            .get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lb/getHostedZoneId:getHostedZoneId".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadBalancerType".into(),
                    value: &load_balancer_type_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetHostedZoneIdResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            load_balancer_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancerType"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
        }
    }
}
