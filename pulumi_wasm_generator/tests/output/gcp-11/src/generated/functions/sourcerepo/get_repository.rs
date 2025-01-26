pub mod get_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// Resource name of the repository. The repo name may contain slashes. eg, `name/with/slash`
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        pub create_ignore_already_exists: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pubsub_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sourcerepo::GetRepositoryPubsubConfig>,
        >,
        pub size: pulumi_wasm_rust::Output<i32>,
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sourcerepo/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRepositoryResult {
            create_ignore_already_exists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createIgnoreAlreadyExists"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pubsub_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pubsubConfigs"),
            ),
            size: pulumi_wasm_rust::__private::into_domain(o.extract_field("size")),
            url: pulumi_wasm_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
