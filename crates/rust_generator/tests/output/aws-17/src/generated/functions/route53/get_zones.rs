#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zones {
    #[allow(dead_code)]
    pub struct GetZonesResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of all the Route53 Hosted Zone IDs found.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::PulumiContext) -> GetZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getZones:getZones".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZonesResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ids")),
        }
    }
}
