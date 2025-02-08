#[allow(clippy::doc_lazy_continuation)]
pub mod get_product {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// List of filters. Passed directly to the API (see GetProducts API reference). These filters must describe a single product, this resource will fail if more than one product is returned by the API.
        #[builder(into)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::pricing::GetProductFilter>,
        >,
        /// Code of the service. Available service codes can be fetched using the DescribeServices pricing API call.
        #[builder(into)]
        pub service_code: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pricing::GetProductFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set to the product returned from the API.
        pub result: pulumi_gestalt_rust::Output<String>,
        pub service_code: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProductArgs,
    ) -> GetProductResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let service_code_binding = args.service_code.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:pricing/getProduct:getProduct".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProductResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            result: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("result"),
            ),
            service_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceCode"),
            ),
        }
    }
}
