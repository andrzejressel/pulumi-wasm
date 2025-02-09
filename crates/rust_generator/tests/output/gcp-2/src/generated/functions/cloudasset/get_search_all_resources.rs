#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_search_all_resources {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSearchAllResourcesArgs {
        /// A list of asset types that this request searches for. If empty, it will search all the [supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types).
        #[builder(into, default)]
        pub asset_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query) for more information. If not specified or empty, it will search all the resources within the specified `scope` and `asset_types`.
        #[builder(into, default)]
        pub query: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A scope can be a project, a folder, or an organization. The search is limited to the resources within the scope. The allowed value must be: organization number (such as "organizations/123"), folder number (such as "folders/1234"), project number (such as "projects/12345") or project id (such as "projects/abc")
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSearchAllResourcesResult {
        pub asset_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub query: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of search results based on provided inputs. Structure is defined below.
        pub results: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudasset::GetSearchAllResourcesResult>,
        >,
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSearchAllResourcesArgs,
    ) -> GetSearchAllResourcesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asset_types_binding = args.asset_types.get_output(context);
        let query_binding = args.query.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudasset/getSearchAllResources:getSearchAllResources".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetTypes".into(),
                    value: asset_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "query".into(),
                    value: query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSearchAllResourcesResult {
            asset_types: o.get_field("assetTypes"),
            id: o.get_field("id"),
            query: o.get_field("query"),
            results: o.get_field("results"),
            scope: o.get_field("scope"),
        }
    }
}
