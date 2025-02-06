pub mod get_s {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSArgs {
        /// The ID of the project. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSResult {
        /// A list of all retrieved service accounts. Structure is defined below.
        pub accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::serviceaccount::GetSAccount>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSArgs,
    ) -> GetSResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:serviceaccount/getS:getS".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSResult {
            accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accounts"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
