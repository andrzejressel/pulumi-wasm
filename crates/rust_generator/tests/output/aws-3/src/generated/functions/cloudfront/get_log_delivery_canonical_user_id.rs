#[allow(clippy::doc_lazy_continuation)]
pub mod get_log_delivery_canonical_user_id {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogDeliveryCanonicalUserIdArgs {
        /// Region you'd like the zone for. By default, fetches the current region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogDeliveryCanonicalUserIdResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLogDeliveryCanonicalUserIdArgs,
    ) -> GetLogDeliveryCanonicalUserIdResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getLogDeliveryCanonicalUserId:getLogDeliveryCanonicalUserId"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLogDeliveryCanonicalUserIdResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
        }
    }
}
