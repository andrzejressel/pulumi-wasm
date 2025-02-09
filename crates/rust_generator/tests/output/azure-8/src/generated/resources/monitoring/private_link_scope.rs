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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PrivateLinkScopeArgs,
    ) -> PrivateLinkScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ingestion_access_mode_binding_1 = args
            .ingestion_access_mode
            .get_output(context);
        let ingestion_access_mode_binding = ingestion_access_mode_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let query_access_mode_binding_1 = args.query_access_mode.get_output(context);
        let query_access_mode_binding = query_access_mode_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/privateLinkScope:PrivateLinkScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ingestionAccessMode".into(),
                    value: &ingestion_access_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryAccessMode".into(),
                    value: &query_access_mode_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrivateLinkScopeResult {
            ingestion_access_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingestionAccessMode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_access_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryAccessMode"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
