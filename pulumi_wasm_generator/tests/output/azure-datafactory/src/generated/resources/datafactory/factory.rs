/// Manages an Azure Data Factory (Version 2).
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/factory:Factory example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example
/// ```
///
pub mod factory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FactoryArgs {
        /// Specifies the Azure Key Vault Key ID to be used as the Customer Managed Key (CMK) for double encryption. Required with user assigned identity.
        #[builder(into, default)]
        pub customer_managed_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the user assigned identity associated with the Customer Managed Key. Must be supplied if `customer_managed_key_id` is set.
        #[builder(into, default)]
        pub customer_managed_key_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `github_configuration` block as defined below.
        #[builder(into, default)]
        pub github_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryGithubConfiguration>,
        >,
        /// A list of `global_parameter` blocks as defined above.
        #[builder(into, default)]
        pub global_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::FactoryGlobalParameter>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Is Managed Virtual Network enabled?
        #[builder(into, default)]
        pub managed_virtual_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Data Factory. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the Data Factory visible to the public network? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the ID of the purview account resource associated with the Data Factory.
        #[builder(into, default)]
        pub purview_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Data Factory. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `vsts_configuration` block as defined below.
        #[builder(into, default)]
        pub vsts_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryVstsConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct FactoryResult {
        /// Specifies the Azure Key Vault Key ID to be used as the Customer Managed Key (CMK) for double encryption. Required with user assigned identity.
        pub customer_managed_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the user assigned identity associated with the Customer Managed Key. Must be supplied if `customer_managed_key_id` is set.
        pub customer_managed_key_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `github_configuration` block as defined below.
        pub github_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryGithubConfiguration>,
        >,
        /// A list of `global_parameter` blocks as defined above.
        pub global_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::FactoryGlobalParameter>>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Is Managed Virtual Network enabled?
        pub managed_virtual_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Data Factory. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is the Data Factory visible to the public network? Defaults to `true`.
        pub public_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the ID of the purview account resource associated with the Data Factory.
        pub purview_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Data Factory. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `vsts_configuration` block as defined below.
        pub vsts_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::FactoryVstsConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FactoryArgs) -> FactoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let customer_managed_key_id_binding = args.customer_managed_key_id.get_inner();
        let customer_managed_key_identity_id_binding = args
            .customer_managed_key_identity_id
            .get_inner();
        let github_configuration_binding = args.github_configuration.get_inner();
        let global_parameters_binding = args.global_parameters.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let managed_virtual_network_enabled_binding = args
            .managed_virtual_network_enabled
            .get_inner();
        let name_binding = args.name.get_inner();
        let public_network_enabled_binding = args.public_network_enabled.get_inner();
        let purview_id_binding = args.purview_id.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let vsts_configuration_binding = args.vsts_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/factory:Factory".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customerManagedKeyId".into(),
                    value: &customer_managed_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKeyIdentityId".into(),
                    value: &customer_managed_key_identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "githubConfiguration".into(),
                    value: &github_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "globalParameters".into(),
                    value: &global_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedVirtualNetworkEnabled".into(),
                    value: &managed_virtual_network_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkEnabled".into(),
                    value: &public_network_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "purviewId".into(),
                    value: &purview_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vstsConfiguration".into(),
                    value: &vsts_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customerManagedKeyId".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyIdentityId".into(),
                },
                register_interface::ResultField {
                    name: "githubConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "globalParameters".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedVirtualNetworkEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkEnabled".into(),
                },
                register_interface::ResultField {
                    name: "purviewId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vstsConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FactoryResult {
            customer_managed_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyId").unwrap(),
            ),
            customer_managed_key_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyIdentityId").unwrap(),
            ),
            github_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("githubConfiguration").unwrap(),
            ),
            global_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalParameters").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_virtual_network_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedVirtualNetworkEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkEnabled").unwrap(),
            ),
            purview_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purviewId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vsts_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vstsConfiguration").unwrap(),
            ),
        }
    }
}
