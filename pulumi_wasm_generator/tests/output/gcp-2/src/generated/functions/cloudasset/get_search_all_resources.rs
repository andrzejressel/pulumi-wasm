pub mod get_search_all_resources {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSearchAllResourcesArgs {
        /// A list of asset types that this request searches for. If empty, it will search all the [supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types).
        #[builder(into, default)]
        pub asset_types: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query) for more information. If not specified or empty, it will search all the resources within the specified `scope` and `asset_types`.
        #[builder(into, default)]
        pub query: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A scope can be a project, a folder, or an organization. The search is limited to the resources within the scope. The allowed value must be: organization number (such as "organizations/123"), folder number (such as "folders/1234"), project number (such as "projects/12345") or project id (such as "projects/abc")
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSearchAllResourcesResult {
        pub asset_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub query: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of search results based on provided inputs. Structure is defined below.
        pub results: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudasset::GetSearchAllResourcesResult>,
        >,
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSearchAllResourcesArgs,
    ) -> GetSearchAllResourcesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let asset_types_binding = args.asset_types.get_output(context).get_inner();
        let query_binding = args.query.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudasset/getSearchAllResources:getSearchAllResources".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assetTypes".into(),
                    value: &asset_types_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSearchAllResourcesResult {
            asset_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assetTypes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            query: pulumi_wasm_rust::__private::into_domain(o.extract_field("query")),
            results: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("results"),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
