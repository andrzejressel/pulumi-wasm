pub mod get_project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectArgs {
        /// A string filter as defined in the [REST API](https://cloud.google.com/resource-manager/reference/rest/v1/projects/list#query-parameters).
        #[builder(into)]
        pub filter: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProjectResult {
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of projects matching the provided filter. Structure is defined below.
        pub projects: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::projects::GetProjectProject>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetProjectArgs,
    ) -> GetProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:projects/getProject:getProject".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProjectResult {
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            projects: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("projects"),
            ),
        }
    }
}
