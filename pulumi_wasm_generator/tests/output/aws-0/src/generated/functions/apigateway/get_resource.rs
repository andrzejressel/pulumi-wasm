pub mod get_resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceArgs {
        /// Full path of the resource.  If no path is found, an error will be returned.
        #[builder(into)]
        pub path: pulumi_wasm_rust::InputOrOutput<String>,
        /// REST API id that owns the resource. If no REST API is found, an error will be returned.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set to the ID of the parent Resource.
        pub parent_id: pulumi_wasm_rust::Output<String>,
        pub path: pulumi_wasm_rust::Output<String>,
        /// Set to the path relative to the parent Resource.
        pub path_part: pulumi_wasm_rust::Output<String>,
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResourceArgs,
    ) -> GetResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let path_binding = args.path.get_output(context).get_inner();
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getResource:getResource".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parentId".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "pathPart".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResourceResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            path_part: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathPart").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
        }
    }
}
