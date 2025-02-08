#[allow(clippy::doc_lazy_continuation)]
pub mod get_resources_search_all {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcesSearchAllArgs {
        /// A list of asset types that this request searches for. If empty, it will search all the [supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types).
        #[builder(into, default)]
        pub asset_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query) for more information. If not specified or empty, it will search all the resources within the specified `scope` and `asset_types`.
        #[builder(into, default)]
        pub query: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A scope can be a project, a folder, or an organization. The allowed value must be: organization number (such as "organizations/123"), folder number (such as "folders/1234"), project number (such as "projects/12345") or project id (such as "projects/abc")
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourcesSearchAllResult {
        pub asset_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub query: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of search results based on provided inputs. Structure is defined below.
        pub results: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudasset::GetResourcesSearchAllResult>,
        >,
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResourcesSearchAllArgs,
    ) -> GetResourcesSearchAllResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let asset_types_binding = args.asset_types.get_output(context).get_inner();
        let query_binding = args.query.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudasset/getResourcesSearchAll:getResourcesSearchAll".into(),
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
        GetResourcesSearchAllResult {
            asset_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assetTypes"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            query: pulumi_gestalt_rust::__private::into_domain(o.extract_field("query")),
            results: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("results"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
