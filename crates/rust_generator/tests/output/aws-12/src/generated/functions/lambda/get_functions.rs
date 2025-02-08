#[allow(clippy::doc_lazy_continuation)]
pub mod get_functions {
    #[allow(dead_code)]
    pub struct GetFunctionsResult {
        /// A list of Lambda Function ARNs.
        pub function_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of Lambda Function names.
        pub function_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::PulumiContext) -> GetFunctionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getFunctions:getFunctions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFunctionsResult {
            function_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionArns"),
            ),
            function_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionNames"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
