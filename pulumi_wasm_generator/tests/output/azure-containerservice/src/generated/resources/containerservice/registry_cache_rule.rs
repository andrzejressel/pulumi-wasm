/// Manages an Azure Container Registry Cache Rule.
///
/// > **Note:** All arguments including the access key will be stored in the raw state as plain-text.
/// [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let acr = registry::create(
///         "acr",
///         RegistryArgs::builder()
///             .location("${example.location}")
///             .name("containerRegistry1")
///             .resource_group_name("${example.name}")
///             .sku("Basic")
///             .build_struct(),
///     );
///     let cacheRule = registry_cache_rule::create(
///         "cacheRule",
///         RegistryCacheRuleArgs::builder()
///             .container_registry_id("${acr.id}")
///             .credential_set_id("${acr.id}/credentialSets/example")
///             .name("cacherule")
///             .source_repo("docker.io/hello-world")
///             .target_repo("target")
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Container Registry Cache Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryCacheRule:RegistryCacheRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myResourceGroup/providers/Microsoft.ContainerRegistry/registries/myRegistry/cacheRules/myCacheRule
/// ```
///
pub mod registry_cache_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryCacheRuleArgs {
        /// The ID of the Container Registry where the Cache Rule should apply. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// The ARM resource ID of the Credential Store which is associated with the Cache Rule.
        #[builder(into, default)]
        pub credential_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Container Registry Cache Rule. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the source repository path. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_repo: pulumi_wasm_rust::Output<String>,
        /// The name of the new repository path to store artifacts. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_repo: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryCacheRuleResult {
        /// The ID of the Container Registry where the Cache Rule should apply. Changing this forces a new resource to be created.
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// The ARM resource ID of the Credential Store which is associated with the Cache Rule.
        pub credential_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Container Registry Cache Rule. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the source repository path. Changing this forces a new resource to be created.
        pub source_repo: pulumi_wasm_rust::Output<String>,
        /// The name of the new repository path to store artifacts. Changing this forces a new resource to be created.
        pub target_repo: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegistryCacheRuleArgs) -> RegistryCacheRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_registry_id_binding = args.container_registry_id.get_inner();
        let credential_set_id_binding = args.credential_set_id.get_inner();
        let name_binding = args.name.get_inner();
        let source_repo_binding = args.source_repo.get_inner();
        let target_repo_binding = args.target_repo.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registryCacheRule:RegistryCacheRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "credentialSetId".into(),
                    value: &credential_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceRepo".into(),
                    value: &source_repo_binding,
                },
                register_interface::ObjectField {
                    name: "targetRepo".into(),
                    value: &target_repo_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerRegistryId".into(),
                },
                register_interface::ResultField {
                    name: "credentialSetId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sourceRepo".into(),
                },
                register_interface::ResultField {
                    name: "targetRepo".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegistryCacheRuleResult {
            container_registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryId").unwrap(),
            ),
            credential_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialSetId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            source_repo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRepo").unwrap(),
            ),
            target_repo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRepo").unwrap(),
            ),
        }
    }
}