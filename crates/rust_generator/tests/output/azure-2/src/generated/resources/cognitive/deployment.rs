/// Manages a Cognitive Services Account Deployment.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// The ID of the Cognitive Services Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether dynamic throttling is enabled.
        #[builder(into, default)]
        pub dynamic_throttling_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `model` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub model: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cognitive::DeploymentModel,
        >,
        /// The name of the Cognitive Services Account Deployment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of RAI policy.
        #[builder(into, default)]
        pub rai_policy_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cognitive::DeploymentSku,
        >,
        /// Deployment model version upgrade option. Possible values are `OnceNewDefaultVersionAvailable`, `OnceCurrentVersionExpired`, and `NoAutoUpgrade`. Defaults to `OnceNewDefaultVersionAvailable`.
        #[builder(into, default)]
        pub version_upgrade_option: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// The ID of the Cognitive Services Account. Changing this forces a new resource to be created.
        pub cognitive_account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether dynamic throttling is enabled.
        pub dynamic_throttling_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `model` block as defined below. Changing this forces a new resource to be created.
        pub model: pulumi_gestalt_rust::Output<
            super::super::types::cognitive::DeploymentModel,
        >,
        /// The name of the Cognitive Services Account Deployment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of RAI policy.
        pub rai_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::types::cognitive::DeploymentSku,
        >,
        /// Deployment model version upgrade option. Possible values are `OnceNewDefaultVersionAvailable`, `OnceCurrentVersionExpired`, and `NoAutoUpgrade`. Defaults to `OnceNewDefaultVersionAvailable`.
        pub version_upgrade_option: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cognitive_account_id_binding_1 = args
            .cognitive_account_id
            .get_output(context);
        let cognitive_account_id_binding = cognitive_account_id_binding_1.get_inner();
        let dynamic_throttling_enabled_binding_1 = args
            .dynamic_throttling_enabled
            .get_output(context);
        let dynamic_throttling_enabled_binding = dynamic_throttling_enabled_binding_1
            .get_inner();
        let model_binding_1 = args.model.get_output(context);
        let model_binding = model_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let rai_policy_name_binding_1 = args.rai_policy_name.get_output(context);
        let rai_policy_name_binding = rai_policy_name_binding_1.get_inner();
        let sku_binding_1 = args.sku.get_output(context);
        let sku_binding = sku_binding_1.get_inner();
        let version_upgrade_option_binding_1 = args
            .version_upgrade_option
            .get_output(context);
        let version_upgrade_option_binding = version_upgrade_option_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cognitive/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeploymentResult {
            cognitive_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitiveAccountId"),
            ),
            dynamic_throttling_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dynamicThrottlingEnabled"),
            ),
            model: pulumi_gestalt_rust::__private::into_domain(o.extract_field("model")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            rai_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("raiPolicyName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            version_upgrade_option: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionUpgradeOption"),
            ),
        }
    }
}
