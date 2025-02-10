/// Manages a Resource Deployment Script of Azure Cli.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-uai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleResourceDeploymentScriptAzureCli:
///     type: azure:core:ResourceDeploymentScriptAzureCli
///     name: example
///     properties:
///       name: example-rdsac
///       resourceGroupName: ${example.name}
///       location: West Europe
///       version: 2.40.0
///       retentionInterval: P1D
///       commandLine: '''foo'' ''bar'''
///       cleanupPreference: OnSuccess
///       forceUpdateTag: '1'
///       timeout: PT30M
///       scriptContent: |2
///                     echo "{\"name\":{\"displayName\":\"$1 $2\"}}" > $AZ_SCRIPTS_OUTPUT_PATH
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Resource Deployment Script can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceDeploymentScriptAzureCli:ResourceDeploymentScriptAzureCli example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Resources/deploymentScripts/script1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_deployment_script_azure_cli {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptAzureCliArgs {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub cleanup_preference: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub command_line: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub container: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::core::ResourceDeploymentScriptAzureCliContainer>,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptAzureCliEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub force_update_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::core::ResourceDeploymentScriptAzureCliIdentity>,
        >,
        /// Specifies the Azure Region where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Resource Deployment Script. The name length must be from 1 to 260 characters. The name can only contain alphanumeric, underscore, parentheses, hyphen and period, and it cannot end with a period. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Uri for the script. This is the entry point for the external script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub primary_script_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. The time duration should be between `1` hour and `26` hours (inclusive) and should be specified in ISO 8601 format. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub retention_interval: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Script body. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub script_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `storage_account` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub storage_account: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::core::ResourceDeploymentScriptAzureCliStorageAccount,
            >,
        >,
        /// Supporting files for the external script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub supporting_script_uris: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A mapping of tags which should be assigned to the Resource Deployment Script.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maximum allowed script execution time specified in ISO 8601 format. Needs to be greater than 0 and smaller than 1 day. Defaults to `P1D`. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the version of the Azure CLI that should be used in the format `X.Y.Z` (e.g. `2.30.0`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-cli/tags/list). Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptAzureCliResult {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        pub cleanup_preference: pulumi_gestalt_rust::Output<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        pub command_line: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub container: pulumi_gestalt_rust::Output<
            Option<super::super::types::core::ResourceDeploymentScriptAzureCliContainer>,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptAzureCliEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        pub force_update_tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::core::ResourceDeploymentScriptAzureCliIdentity>,
        >,
        /// Specifies the Azure Region where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Resource Deployment Script. The name length must be from 1 to 260 characters. The name can only contain alphanumeric, underscore, parentheses, hyphen and period, and it cannot end with a period. Changing this forces a new Resource Deployment Script to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of script outputs.
        pub outputs: pulumi_gestalt_rust::Output<String>,
        /// Uri for the script. This is the entry point for the external script. Changing this forces a new Resource Deployment Script to be created.
        pub primary_script_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. The time duration should be between `1` hour and `26` hours (inclusive) and should be specified in ISO 8601 format. Changing this forces a new Resource Deployment Script to be created.
        pub retention_interval: pulumi_gestalt_rust::Output<String>,
        /// Script body. Changing this forces a new Resource Deployment Script to be created.
        pub script_content: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `storage_account` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub storage_account: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptAzureCliStorageAccount,
            >,
        >,
        /// Supporting files for the external script. Changing this forces a new Resource Deployment Script to be created.
        pub supporting_script_uris: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Resource Deployment Script.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maximum allowed script execution time specified in ISO 8601 format. Needs to be greater than 0 and smaller than 1 day. Defaults to `P1D`. Changing this forces a new Resource Deployment Script to be created.
        pub timeout: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the version of the Azure CLI that should be used in the format `X.Y.Z` (e.g. `2.30.0`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-cli/tags/list). Changing this forces a new Resource Deployment Script to be created.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceDeploymentScriptAzureCliArgs,
    ) -> ResourceDeploymentScriptAzureCliResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cleanup_preference_binding = args.cleanup_preference.get_output(context);
        let command_line_binding = args.command_line.get_output(context);
        let container_binding = args.container.get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let force_update_tag_binding = args.force_update_tag.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let primary_script_uri_binding = args.primary_script_uri.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_interval_binding = args.retention_interval.get_output(context);
        let script_content_binding = args.script_content.get_output(context);
        let storage_account_binding = args.storage_account.get_output(context);
        let supporting_script_uris_binding = args
            .supporting_script_uris
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/resourceDeploymentScriptAzureCli:ResourceDeploymentScriptAzureCli"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cleanupPreference".into(),
                    value: cleanup_preference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commandLine".into(),
                    value: command_line_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "container".into(),
                    value: container_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: environment_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceUpdateTag".into(),
                    value: force_update_tag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryScriptUri".into(),
                    value: primary_script_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInterval".into(),
                    value: retention_interval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptContent".into(),
                    value: script_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccount".into(),
                    value: storage_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportingScriptUris".into(),
                    value: supporting_script_uris_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceDeploymentScriptAzureCliResult {
            cleanup_preference: o.get_field("cleanupPreference"),
            command_line: o.get_field("commandLine"),
            container: o.get_field("container"),
            environment_variables: o.get_field("environmentVariables"),
            force_update_tag: o.get_field("forceUpdateTag"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outputs: o.get_field("outputs"),
            primary_script_uri: o.get_field("primaryScriptUri"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_interval: o.get_field("retentionInterval"),
            script_content: o.get_field("scriptContent"),
            storage_account: o.get_field("storageAccount"),
            supporting_script_uris: o.get_field("supportingScriptUris"),
            tags: o.get_field("tags"),
            timeout: o.get_field("timeout"),
            version: o.get_field("version"),
        }
    }
}
