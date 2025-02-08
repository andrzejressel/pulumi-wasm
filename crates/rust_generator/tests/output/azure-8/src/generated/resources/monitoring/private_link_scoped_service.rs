/// Manages an Azure Monitor Private Link Scoped Service.
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
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePrivateLinkScope = private_link_scope::create(
///         "examplePrivateLinkScope",
///         PrivateLinkScopeArgs::builder()
///             .name("example-ampls")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePrivateLinkScopedService = private_link_scoped_service::create(
///         "examplePrivateLinkScopedService",
///         PrivateLinkScopedServiceArgs::builder()
///             .linked_resource_id("${exampleInsights.id}")
///             .name("example-amplsservice")
///             .resource_group_name("${example.name}")
///             .scope_name("${examplePrivateLinkScope.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Monitor Private Link Scoped Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/privateLinkScopedService:PrivateLinkScopedService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/privateLinkScopes/pls1/scopedResources/sr1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_link_scoped_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkScopedServiceArgs {
        /// The ID of the linked resource. It must be the Log Analytics workspace or the Application Insights component or the Data Collection endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub linked_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Azure Monitor Private Link Scoped Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scoped Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkScopedServiceResult {
        /// The ID of the linked resource. It must be the Log Analytics workspace or the Application Insights component or the Data Collection endpoint. Changing this forces a new resource to be created.
        pub linked_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Monitor Private Link Scoped Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Monitor Private Link Scoped Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Monitor Private Link Scope. Changing this forces a new resource to be created.
        pub scope_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PrivateLinkScopedServiceArgs,
    ) -> PrivateLinkScopedServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let linked_resource_id_binding = args
            .linked_resource_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scope_name_binding = args.scope_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/privateLinkScopedService:PrivateLinkScopedService"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "linkedResourceId".into(),
                    value: &linked_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeName".into(),
                    value: &scope_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrivateLinkScopedServiceResult {
            linked_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedResourceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scope_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeName"),
            ),
        }
    }
}
