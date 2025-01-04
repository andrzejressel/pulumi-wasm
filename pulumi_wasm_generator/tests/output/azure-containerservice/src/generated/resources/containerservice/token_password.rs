/// Manages a Container Registry Token Password.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource-group
///       location: West Europe
///   exampleRegistry:
///     type: azure:containerservice:Registry
///     name: example
///     properties:
///       name: example-registry
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Premium
///       adminEnabled: false
///       georeplicationLocations:
///         - East US
///         - West Europe
///   exampleRegistryScopeMap:
///     type: azure:containerservice:RegistryScopeMap
///     name: example
///     properties:
///       name: example-scope-map
///       containerRegistryName: ${exampleRegistry.name}
///       resourceGroupName: ${example.name}
///       actions:
///         - repositories/repo1/content/read
///         - repositories/repo1/content/write
///   exampleRegistryToken:
///     type: azure:containerservice:RegistryToken
///     name: example
///     properties:
///       name: exampletoken
///       containerRegistryName: ${exampleRegistry.name}
///       resourceGroupName: ${example.name}
///       scopeMapId: ${exampleRegistryScopeMap.id}
///   exampleTokenPassword:
///     type: azure:containerservice:TokenPassword
///     name: example
///     properties:
///       containerRegistryTokenId: ${exampleRegistryToken.id}
///       password1:
///         expiry: 2023-03-22T17:57:36+08:00
/// ```
///
/// ## Import
///
/// Container Registry Token Passwords can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/tokenPassword:TokenPassword example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.ContainerRegistry/registries/registry1/tokens/token1/passwords/password
/// ```
///
pub mod token_password {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TokenPasswordArgs {
        /// The ID of the Container Registry Token that this Container Registry Token Password resides in. Changing this forces a new Container Registry Token Password to be created.
        #[builder(into)]
        pub container_registry_token_id: pulumi_wasm_rust::Output<String>,
        /// One `password` block as defined below.
        #[builder(into)]
        pub password1: pulumi_wasm_rust::Output<
            super::super::types::containerservice::TokenPasswordPassword1,
        >,
        /// One `password` block as defined below.
        #[builder(into, default)]
        pub password2: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::TokenPasswordPassword2>,
        >,
    }
    #[allow(dead_code)]
    pub struct TokenPasswordResult {
        /// The ID of the Container Registry Token that this Container Registry Token Password resides in. Changing this forces a new Container Registry Token Password to be created.
        pub container_registry_token_id: pulumi_wasm_rust::Output<String>,
        /// One `password` block as defined below.
        pub password1: pulumi_wasm_rust::Output<
            super::super::types::containerservice::TokenPasswordPassword1,
        >,
        /// One `password` block as defined below.
        pub password2: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::TokenPasswordPassword2>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TokenPasswordArgs) -> TokenPasswordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_registry_token_id_binding = args
            .container_registry_token_id
            .get_inner();
        let password1_binding = args.password1.get_inner();
        let password2_binding = args.password2.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/tokenPassword:TokenPassword".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryTokenId".into(),
                    value: &container_registry_token_id_binding,
                },
                register_interface::ObjectField {
                    name: "password1".into(),
                    value: &password1_binding,
                },
                register_interface::ObjectField {
                    name: "password2".into(),
                    value: &password2_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerRegistryTokenId".into(),
                },
                register_interface::ResultField {
                    name: "password1".into(),
                },
                register_interface::ResultField {
                    name: "password2".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TokenPasswordResult {
            container_registry_token_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryTokenId").unwrap(),
            ),
            password1: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password1").unwrap(),
            ),
            password2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password2").unwrap(),
            ),
        }
    }
}
