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
pub mod resource_deployment_script_power_shell {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptPowerShellArgs {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub cleanup_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub command_line: pulumi_wasm_rust::Output<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub container: pulumi_wasm_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellContainer,
            >,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptPowerShellEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub force_update_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::core::ResourceDeploymentScriptPowerShellIdentity>,
        >,
        /// Specifies the Azure Region where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Resource Deployment Script. The name length must be from 1 to 260 characters. The name can only contain alphanumeric, underscore, parentheses, hyphen and period, and it cannot end with a period. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Uri for the script. This is the entry point for the external script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub primary_script_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. The time duration should be between `1` hour and `26` hours (inclusive) and should be specified in ISO 8601 format. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub retention_interval: pulumi_wasm_rust::Output<String>,
        /// Script body. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub script_content: pulumi_wasm_rust::Output<Option<String>>,
        /// A `storage_account` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub storage_account: pulumi_wasm_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellStorageAccount,
            >,
        >,
        /// Supporting files for the external script. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub supporting_script_uris: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Resource Deployment Script.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maximum allowed script execution time specified in ISO 8601 format. Needs to be greater than 0 and smaller than 1 day. Defaults to `P1D`. Changing this forces a new Resource Deployment Script to be created.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the version of the Azure PowerShell that should be used in the format `X.Y` (e.g. `9.7`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-powershell/tags/list). Changing this forces a new Resource Deployment Script to be created.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceDeploymentScriptPowerShellResult {
        /// Specifies the cleanup preference when the script execution gets in a terminal state. Possible values are `Always`, `OnExpiration`, `OnSuccess`. Defaults to `Always`. Changing this forces a new Resource Deployment Script to be created.
        pub cleanup_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// Command line arguments to pass to the script. Changing this forces a new Resource Deployment Script to be created.
        pub command_line: pulumi_wasm_rust::Output<Option<String>>,
        /// A `container` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub container: pulumi_wasm_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellContainer,
            >,
        >,
        /// An `environment_variable` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourceDeploymentScriptPowerShellEnvironmentVariable,
                >,
            >,
        >,
        /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID. Changing this forces a new Resource Deployment Script to be created.
        pub force_update_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::core::ResourceDeploymentScriptPowerShellIdentity>,
        >,
        /// Specifies the Azure Region where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Resource Deployment Script. The name length must be from 1 to 260 characters. The name can only contain alphanumeric, underscore, parentheses, hyphen and period, and it cannot end with a period. Changing this forces a new Resource Deployment Script to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of script outputs.
        pub outputs: pulumi_wasm_rust::Output<String>,
        /// Uri for the script. This is the entry point for the external script. Changing this forces a new Resource Deployment Script to be created.
        pub primary_script_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Resource Deployment Script should exist. Changing this forces a new Resource Deployment Script to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. The time duration should be between `1` hour and `26` hours (inclusive) and should be specified in ISO 8601 format. Changing this forces a new Resource Deployment Script to be created.
        pub retention_interval: pulumi_wasm_rust::Output<String>,
        /// Script body. Changing this forces a new Resource Deployment Script to be created.
        pub script_content: pulumi_wasm_rust::Output<Option<String>>,
        /// A `storage_account` block as defined below. Changing this forces a new Resource Deployment Script to be created.
        pub storage_account: pulumi_wasm_rust::Output<
            Option<
                super::super::types::core::ResourceDeploymentScriptPowerShellStorageAccount,
            >,
        >,
        /// Supporting files for the external script. Changing this forces a new Resource Deployment Script to be created.
        pub supporting_script_uris: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Resource Deployment Script.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maximum allowed script execution time specified in ISO 8601 format. Needs to be greater than 0 and smaller than 1 day. Defaults to `P1D`. Changing this forces a new Resource Deployment Script to be created.
        pub timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the version of the Azure PowerShell that should be used in the format `X.Y` (e.g. `9.7`). A canonical list of versions [is available from the Microsoft Container Registry API](https://mcr.microsoft.com/v2/azure-powershell/tags/list). Changing this forces a new Resource Deployment Script to be created.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourceDeploymentScriptPowerShellArgs,
    ) -> ResourceDeploymentScriptPowerShellResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cleanup_preference_binding = args.cleanup_preference.get_inner();
        let command_line_binding = args.command_line.get_inner();
        let container_binding = args.container.get_inner();
        let environment_variables_binding = args.environment_variables.get_inner();
        let force_update_tag_binding = args.force_update_tag.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let primary_script_uri_binding = args.primary_script_uri.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let retention_interval_binding = args.retention_interval.get_inner();
        let script_content_binding = args.script_content.get_inner();
        let storage_account_binding = args.storage_account.get_inner();
        let supporting_script_uris_binding = args.supporting_script_uris.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceDeploymentScriptPowerShell:ResourceDeploymentScriptPowerShell"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "cleanupPreference".into(),
                },
                register_interface::ResultField {
                    name: "commandLine".into(),
                },
                register_interface::ResultField {
                    name: "container".into(),
                },
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "forceUpdateTag".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputs".into(),
                },
                register_interface::ResultField {
                    name: "primaryScriptUri".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionInterval".into(),
                },
                register_interface::ResultField {
                    name: "scriptContent".into(),
                },
                register_interface::ResultField {
                    name: "storageAccount".into(),
                },
                register_interface::ResultField {
                    name: "supportingScriptUris".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceDeploymentScriptPowerShellResult {
            cleanup_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cleanupPreference").unwrap(),
            ),
            command_line: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commandLine").unwrap(),
            ),
            container: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("container").unwrap(),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            force_update_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceUpdateTag").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputs").unwrap(),
            ),
            primary_script_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryScriptUri").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionInterval").unwrap(),
            ),
            script_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptContent").unwrap(),
            ),
            storage_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccount").unwrap(),
            ),
            supporting_script_uris: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportingScriptUris").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}