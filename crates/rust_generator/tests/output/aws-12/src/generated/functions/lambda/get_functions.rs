#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetFunctionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getFunctions:getFunctions".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetFunctionsResult {
            function_arns: o.get_field("functionArns"),
            function_names: o.get_field("functionNames"),
            id: o.get_field("id"),
        }
    }
}
