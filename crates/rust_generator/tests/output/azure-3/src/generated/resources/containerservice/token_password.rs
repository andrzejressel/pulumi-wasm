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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod token_password {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TokenPasswordArgs {
        /// The ID of the Container Registry Token that this Container Registry Token Password resides in. Changing this forces a new Container Registry Token Password to be created.
        #[builder(into)]
        pub container_registry_token_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One `password` block as defined below.
        #[builder(into)]
        pub password1: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containerservice::TokenPasswordPassword1,
        >,
        /// One `password` block as defined below.
        #[builder(into, default)]
        pub password2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::TokenPasswordPassword2>,
        >,
    }
    #[allow(dead_code)]
    pub struct TokenPasswordResult {
        /// The ID of the Container Registry Token that this Container Registry Token Password resides in. Changing this forces a new Container Registry Token Password to be created.
        pub container_registry_token_id: pulumi_gestalt_rust::Output<String>,
        /// One `password` block as defined below.
        pub password1: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::TokenPasswordPassword1,
        >,
        /// One `password` block as defined below.
        pub password2: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::TokenPasswordPassword2>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TokenPasswordArgs,
    ) -> TokenPasswordResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_token_id_binding = args
            .container_registry_token_id
            .get_output(context);
        let password1_binding = args.password1.get_output(context);
        let password2_binding = args.password2.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/tokenPassword:TokenPassword".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryTokenId".into(),
                    value: container_registry_token_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password1".into(),
                    value: password1_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password2".into(),
                    value: password2_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TokenPasswordResult {
            container_registry_token_id: o.get_field("containerRegistryTokenId"),
            password1: o.get_field("password1"),
            password2: o.get_field("password2"),
        }
    }
}
