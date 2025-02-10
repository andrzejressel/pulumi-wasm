/// Manages the linked service to link an Azure Machine learning workspace to an Azure Synapse workspace.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: west europe
///       tags:
///         stage: example
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-kv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-mlw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       identity:
///         type: SystemAssigned
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace2:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleSparkPool:
///     type: azure:synapse:SparkPool
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace2.id}
///       nodeSizeFamily: MemoryOptimized
///       nodeSize: Small
///       nodeCount: 3
///   exampleSynapseSpark:
///     type: azure:machinelearning:SynapseSpark
///     name: example
///     properties:
///       name: example
///       machineLearningWorkspaceId: ${exampleWorkspace.id}
///       location: ${example.location}
///       synapseSparkPoolId: ${exampleSparkPool.id}
///       identity:
///         type: SystemAssigned
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning Synapse Sparks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/synapseSpark:SynapseSpark example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.MachineLearningServices/workspaces/workspace1/computes/compute1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod synapse_spark {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SynapseSparkArgs {
        /// The description of the Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::SynapseSparkIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Machine Learning Synapse Spark should exist. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into)]
        pub machine_learning_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the linked Synapse Spark Pool. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into)]
        pub synapse_spark_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SynapseSparkResult {
        /// The description of the Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::SynapseSparkIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Machine Learning Synapse Spark should exist. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub machine_learning_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the linked Synapse Spark Pool. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub synapse_spark_pool_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Machine Learning Synapse Spark. Changing this forces a new Machine Learning Synapse Spark to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SynapseSparkArgs,
    ) -> SynapseSparkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_auth_enabled_binding = args.local_auth_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let machine_learning_workspace_id_binding = args
            .machine_learning_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let synapse_spark_pool_id_binding = args
            .synapse_spark_pool_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/synapseSpark:SynapseSpark".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthEnabled".into(),
                    value: local_auth_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineLearningWorkspaceId".into(),
                    value: machine_learning_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseSparkPoolId".into(),
                    value: synapse_spark_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SynapseSparkResult {
            description: o.get_field("description"),
            identity: o.get_field("identity"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            machine_learning_workspace_id: o.get_field("machineLearningWorkspaceId"),
            name: o.get_field("name"),
            synapse_spark_pool_id: o.get_field("synapseSparkPoolId"),
            tags: o.get_field("tags"),
        }
    }
}
