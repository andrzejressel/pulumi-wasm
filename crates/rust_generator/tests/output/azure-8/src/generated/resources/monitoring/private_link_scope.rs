/// Manages an Azure Monitor Private Link Scope.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let examplePrivateLinkScope = private_link_scope::create(
///         "examplePrivateLinkScope",
///         PrivateLinkScopeArgs::builder()
///             .ingestion_access_mode("PrivateOnly")
///             .name("example-ampls")
///             .query_access_mode("Open")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Monitor Private Link Scopes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/privateLinkScope:PrivateLinkScope example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/privateLinkScopes/pls1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_link_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkScopeArgs {
        /// The default ingestion access mode for the associated private endpoints in scope. Possible values are `Open` and `PrivateOnly`. Defaults to `Open`.
        #[builder(into, default)]
        pub ingestion_access_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default query access mode for hte associated private endpoints in scope. Possible values are `Open` and `PrivateOnly`. Defaults to `Open`.
        #[builder(into, default)]
        pub query_access_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scope should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Monitor Private Link Scope.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkScopeResult {
        /// The default ingestion access mode for the associated private endpoints in scope. Possible values are `Open` and `PrivateOnly`. Defaults to `Open`.
        pub ingestion_access_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The default query access mode for hte associated private endpoints in scope. Possible values are `Open` and `PrivateOnly`. Defaults to `Open`.
        pub query_access_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scope should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Monitor Private Link Scope.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkScopeArgs,
    ) -> PrivateLinkScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ingestion_access_mode_binding = args
            .ingestion_access_mode
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let query_access_mode_binding = args.query_access_mode.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/privateLinkScope:PrivateLinkScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingestionAccessMode".into(),
                    value: ingestion_access_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryAccessMode".into(),
                    value: query_access_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrivateLinkScopeResult {
            ingestion_access_mode: o.get_field("ingestionAccessMode"),
            name: o.get_field("name"),
            query_access_mode: o.get_field("queryAccessMode"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
