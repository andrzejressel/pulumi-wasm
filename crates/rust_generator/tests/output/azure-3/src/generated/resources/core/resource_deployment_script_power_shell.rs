/// Manages a Resource Deployment Script of Azure PowerShell.
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
///   exampleResourceDeploymentScriptPowerShell:
///     type: azure:core:ResourceDeploymentScriptPowerShell
///     name: example
///     properties:
///       name: example-rdsaps
///       resourceGroupName: ${example.name}
///       location: West Europe
///       version: '8.3'
///       retentionInterval: P1D
///       commandLine: -name "John Dole"
///       cleanupPreference: OnSuccess
///       forceUpdateTag: '1'
///       timeout: PT30M
///       scriptContent: |2
///                   param([string] $name)
///                     $output = 'Hello {0}.' -f $name
///                     Write-Output $output
///                     $DeploymentScriptOutputs = @{}
///                     $DeploymentScriptOutputs['text'] = $output
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
/// $ pulumi import azure:core/resourceDeploymentScriptPowerShell:ResourceDeploymentScriptPowerShell example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Resources/deploymentScripts/script1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_deployment_script_power_shell {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptPowerShellArgs {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub cleanup_preference: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub command_line: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub container: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellContainer,
            >,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptPowerShellEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub force_update_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::core::ResourceDeploymentScriptPowerShellIdentity>,
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
                super::super::types::core::ResourceDeploymentScriptPowerShellStorageAccount,
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
        /// Specifies the version of the Azure PowerShell that should be used in the format `X.Y` (e.g. `9.7`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-powershell/tags/list). Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptPowerShellResult {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        pub cleanup_preference: pulumi_gestalt_rust::Output<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        pub command_line: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub container: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellContainer,
            >,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptPowerShellEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        pub force_update_tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::core::ResourceDeploymentScriptPowerShellIdentity>,
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
                super::super::types::core::ResourceDeploymentScriptPowerShellStorageAccount,
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
        /// Specifies the version of the Azure PowerShell that should be used in the format `X.Y` (e.g. `9.7`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-powershell/tags/list). Changing this forces a new Resource Deployment Script to be created.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceDeploymentScriptPowerShellArgs,
    ) -> ResourceDeploymentScriptPowerShellResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cleanup_preference_binding_1 = args.cleanup_preference.get_output(context);
        let cleanup_preference_binding = cleanup_preference_binding_1.get_inner();
        let command_line_binding_1 = args.command_line.get_output(context);
        let command_line_binding = command_line_binding_1.get_inner();
        let container_binding_1 = args.container.get_output(context);
        let container_binding = container_binding_1.get_inner();
        let environment_variables_binding_1 = args
            .environment_variables
            .get_output(context);
        let environment_variables_binding = environment_variables_binding_1.get_inner();
        let force_update_tag_binding_1 = args.force_update_tag.get_output(context);
        let force_update_tag_binding = force_update_tag_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let primary_script_uri_binding_1 = args.primary_script_uri.get_output(context);
        let primary_script_uri_binding = primary_script_uri_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let retention_interval_binding_1 = args.retention_interval.get_output(context);
        let retention_interval_binding = retention_interval_binding_1.get_inner();
        let script_content_binding_1 = args.script_content.get_output(context);
        let script_content_binding = script_content_binding_1.get_inner();
        let storage_account_binding_1 = args.storage_account.get_output(context);
        let storage_account_binding = storage_account_binding_1.get_inner();
        let supporting_script_uris_binding_1 = args
            .supporting_script_uris
            .get_output(context);
        let supporting_script_uris_binding = supporting_script_uris_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeout_binding_1 = args.timeout.get_output(context);
        let timeout_binding = timeout_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceDeploymentScriptPowerShell:ResourceDeploymentScriptPowerShell"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cleanupPreference".into(),
                    value: &cleanup_preference_binding,
                },
                register_interface::ObjectField {
                    name: "commandLine".into(),
                    value: &command_line_binding,
                },
                register_interface::ObjectField {
                    name: "container".into(),
                    value: &container_binding,
                },
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdateTag".into(),
                    value: &force_update_tag_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "primaryScriptUri".into(),
                    value: &primary_script_uri_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionInterval".into(),
                    value: &retention_interval_binding,
                },
                register_interface::ObjectField {
                    name: "scriptContent".into(),
                    value: &script_content_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccount".into(),
                    value: &storage_account_binding,
                },
                register_interface::ObjectField {
                    name: "supportingScriptUris".into(),
                    value: &supporting_script_uris_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceDeploymentScriptPowerShellResult {
            cleanup_preference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cleanupPreference"),
            ),
            command_line: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commandLine"),
            ),
            container: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("container"),
            ),
            environment_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentVariables"),
            ),
            force_update_tag: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceUpdateTag"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outputs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputs"),
            ),
            primary_script_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryScriptUri"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retention_interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionInterval"),
            ),
            script_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptContent"),
            ),
            storage_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccount"),
            ),
            supporting_script_uris: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportingScriptUris"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeout"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
