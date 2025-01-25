/// Manages an Azure Arc Private Link Scope.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("west europe")
///             .name("rg-example")
///             .build_struct(),
///     );
///     let examplePrivateLinkScope = private_link_scope::create(
///         "examplePrivateLinkScope",
///         PrivateLinkScopeArgs::builder()
///             .location("${example.location}")
///             .name("plsexample")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Arc Private Link Scope can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arc/privateLinkScope:PrivateLinkScope example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HybridCompute/privateLinkScopes/privateLinkScope1
/// ```
///
pub mod private_link_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkScopeArgs {
        /// The Azure Region where the Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for the Azure Arc Private Link Scope. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates whether machines associated with the private link scope can also use public Azure Arc service endpoints. Defaults to `false`. Possible values are `true` and `false`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Azure Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Arc Private Link Scope.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkScopeResult {
        /// The Azure Region where the Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for the Azure Arc Private Link Scope. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether machines associated with the private link scope can also use public Azure Arc service endpoints. Defaults to `false`. Possible values are `true` and `false`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Azure Arc Private Link Scope should exist. Changing this forces a new Azure Arc Private Link Scope to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Arc Private Link Scope.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PrivateLinkScopeArgs,
    ) -> PrivateLinkScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arc/privateLinkScope:PrivateLinkScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrivateLinkScopeResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
