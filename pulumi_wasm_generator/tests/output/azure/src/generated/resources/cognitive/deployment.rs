/// Manages a Cognitive Services Account Deployment.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .kind("OpenAI")
///             .location("${example.location}")
///             .name("example-ca")
///             .resource_group_name("${example.name}")
///             .sku_name("S0")
///             .build_struct(),
///     );
///     let exampleDeployment = deployment::create(
///         "exampleDeployment",
///         DeploymentArgs::builder()
///             .cognitive_account_id("${exampleAccount.id}")
///             .model(
///                 DeploymentModel::builder()
///                     .format("OpenAI")
///                     .name("text-curie-001")
///                     .version("1")
///                     .build_struct(),
///             )
///             .name("example-cd")
///             .sku(DeploymentSku::builder().name("Standard").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cognitive Services Account Deployment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/deployment:Deployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.CognitiveServices/accounts/account1/deployments/deployment1
/// ```
///
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// The ID of the Cognitive Services Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_wasm_rust::Output<String>,
        /// Whether dynamic throttling is enabled.
        #[builder(into, default)]
        pub dynamic_throttling_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `model` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub model: pulumi_wasm_rust::Output<
            super::super::types::cognitive::DeploymentModel,
        >,
        /// The name of the Cognitive Services Account Deployment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of RAI policy.
        #[builder(into, default)]
        pub rai_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<super::super::types::cognitive::DeploymentSku>,
        /// Deployment model version upgrade option. Possible values are `OnceNewDefaultVersionAvailable`, `OnceCurrentVersionExpired`, and `NoAutoUpgrade`. Defaults to `OnceNewDefaultVersionAvailable`.
        #[builder(into, default)]
        pub version_upgrade_option: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// The ID of the Cognitive Services Account. Changing this forces a new resource to be created.
        pub cognitive_account_id: pulumi_wasm_rust::Output<String>,
        /// Whether dynamic throttling is enabled.
        pub dynamic_throttling_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `model` block as defined below. Changing this forces a new resource to be created.
        pub model: pulumi_wasm_rust::Output<
            super::super::types::cognitive::DeploymentModel,
        >,
        /// The name of the Cognitive Services Account Deployment. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of RAI policy.
        pub rai_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<super::super::types::cognitive::DeploymentSku>,
        /// Deployment model version upgrade option. Possible values are `OnceNewDefaultVersionAvailable`, `OnceCurrentVersionExpired`, and `NoAutoUpgrade`. Defaults to `OnceNewDefaultVersionAvailable`.
        pub version_upgrade_option: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentArgs) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cognitive_account_id_binding = args.cognitive_account_id.get_inner();
        let dynamic_throttling_enabled_binding = args
            .dynamic_throttling_enabled
            .get_inner();
        let model_binding = args.model.get_inner();
        let name_binding = args.name.get_inner();
        let rai_policy_name_binding = args.rai_policy_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let version_upgrade_option_binding = args.version_upgrade_option.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cognitive/deployment:Deployment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cognitiveAccountId".into(),
                    value: &cognitive_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dynamicThrottlingEnabled".into(),
                    value: &dynamic_throttling_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "model".into(),
                    value: &model_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "raiPolicyName".into(),
                    value: &rai_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "versionUpgradeOption".into(),
                    value: &version_upgrade_option_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cognitiveAccountId".into(),
                },
                register_interface::ResultField {
                    name: "dynamicThrottlingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "model".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "raiPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "versionUpgradeOption".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentResult {
            cognitive_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitiveAccountId").unwrap(),
            ),
            dynamic_throttling_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dynamicThrottlingEnabled").unwrap(),
            ),
            model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("model").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rai_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("raiPolicyName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            version_upgrade_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionUpgradeOption").unwrap(),
            ),
        }
    }
}