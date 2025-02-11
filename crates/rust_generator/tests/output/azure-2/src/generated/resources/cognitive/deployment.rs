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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cognitive_account_id_binding = args.cognitive_account_id.get_output(context);
        let dynamic_throttling_enabled_binding = args
            .dynamic_throttling_enabled
            .get_output(context);
        let model_binding = args.model.get_output(context);
        let name_binding = args.name.get_output(context);
        let rai_policy_name_binding = args.rai_policy_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let version_upgrade_option_binding = args
            .version_upgrade_option
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cognitive/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cognitiveAccountId".into(),
                    value: &cognitive_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicThrottlingEnabled".into(),
                    value: &dynamic_throttling_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "model".into(),
                    value: &model_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "raiPolicyName".into(),
                    value: &rai_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionUpgradeOption".into(),
                    value: &version_upgrade_option_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentResult {
            cognitive_account_id: o.get_field("cognitiveAccountId"),
            dynamic_throttling_enabled: o.get_field("dynamicThrottlingEnabled"),
            model: o.get_field("model"),
            name: o.get_field("name"),
            rai_policy_name: o.get_field("raiPolicyName"),
            sku: o.get_field("sku"),
            version_upgrade_option: o.get_field("versionUpgradeOption"),
        }
    }
}
