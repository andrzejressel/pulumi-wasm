/// Manages a Synapse Role Assignment.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
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
///   exampleFirewallRule:
///     type: azure:synapse:FirewallRule
///     name: example
///     properties:
///       name: AllowAll
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 255.255.255.255
///   exampleRoleAssignment:
///     type: azure:synapse:RoleAssignment
///     name: example
///     properties:
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       roleName: Synapse SQL Administrator
///       principalId: ${current.objectId}
///     options:
///       dependsOn:
///         - ${exampleFirewallRule}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Synapse Role Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/roleAssignment:RoleAssignment example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1|000000000000"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleAssignmentArgs {
        /// The ID of the Principal (User, Group or Service Principal) to assign the Synapse Role Definition to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Type of the Principal. One of `User`, `Group` or `ServicePrincipal`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** While `principal_type` is optional, it's still recommended to set this value, as some Synapse use-cases may not work correctly if this is not specified. Service Principals for example can't run SQL statements using `Entra ID` authentication if `principal_type` is not set to `ServicePrincipal`.
        #[builder(into, default)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Role Name of the Synapse Built-In Role. Possible values are `Apache Spark Administrator`, `Synapse Administrator`, `Synapse Artifact Publisher`, `Synapse Artifact User`, `Synapse Compute Operator`, `Synapse Contributor`, `Synapse Credential User`, `Synapse Linked Data Manager`, `Synapse Monitoring Operator`, `Synapse SQL Administrator` and `Synapse User`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Currently, the Synapse built-in roles are `Apache Spark Administrator`, `Synapse Administrator`, `Synapse Artifact Publisher`, `Synapse Artifact User`, `Synapse Compute Operator`, `Synapse Contributor`, `Synapse Credential User`, `Synapse Linked Data Manager`, `Synapse Monitoring Operator`, `Synapse SQL Administrator` and `Synapse User`.
        ///
        /// > **NOTE:** Old roles are still supported: `Workspace Admin`, `Apache Spark Admin`, `Sql Admin`. These values will be removed in the next Major Version 3.0.
        #[builder(into)]
        pub role_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Synapse Spark Pool which the Synapse Role Assignment applies to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Synapse firewall rule including local IP is needed to allow access. Only one of `synapse_workspace_id`, `synapse_spark_pool_id` must be set.
        #[builder(into, default)]
        pub synapse_spark_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Synapse Workspace which the Synapse Role Assignment applies to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RoleAssignmentResult {
        /// The ID of the Principal (User, Group or Service Principal) to assign the Synapse Role Definition to. Changing this forces a new resource to be created.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// The Type of the Principal. One of `User`, `Group` or `ServicePrincipal`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** While `principal_type` is optional, it's still recommended to set this value, as some Synapse use-cases may not work correctly if this is not specified. Service Principals for example can't run SQL statements using `Entra ID` authentication if `principal_type` is not set to `ServicePrincipal`.
        pub principal_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Role Name of the Synapse Built-In Role. Possible values are `Apache Spark Administrator`, `Synapse Administrator`, `Synapse Artifact Publisher`, `Synapse Artifact User`, `Synapse Compute Operator`, `Synapse Contributor`, `Synapse Credential User`, `Synapse Linked Data Manager`, `Synapse Monitoring Operator`, `Synapse SQL Administrator` and `Synapse User`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Currently, the Synapse built-in roles are `Apache Spark Administrator`, `Synapse Administrator`, `Synapse Artifact Publisher`, `Synapse Artifact User`, `Synapse Compute Operator`, `Synapse Contributor`, `Synapse Credential User`, `Synapse Linked Data Manager`, `Synapse Monitoring Operator`, `Synapse SQL Administrator` and `Synapse User`.
        ///
        /// > **NOTE:** Old roles are still supported: `Workspace Admin`, `Apache Spark Admin`, `Sql Admin`. These values will be removed in the next Major Version 3.0.
        pub role_name: pulumi_gestalt_rust::Output<String>,
        /// The Synapse Spark Pool which the Synapse Role Assignment applies to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Synapse firewall rule including local IP is needed to allow access. Only one of `synapse_workspace_id`, `synapse_spark_pool_id` must be set.
        pub synapse_spark_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Synapse Workspace which the Synapse Role Assignment applies to. Changing this forces a new resource to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoleAssignmentArgs,
    ) -> RoleAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let principal_id_binding = args.principal_id.get_output(context);
        let principal_type_binding = args.principal_type.get_output(context);
        let role_name_binding = args.role_name.get_output(context);
        let synapse_spark_pool_id_binding = args
            .synapse_spark_pool_id
            .get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/roleAssignment:RoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalType".into(),
                    value: principal_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleName".into(),
                    value: role_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseSparkPoolId".into(),
                    value: synapse_spark_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RoleAssignmentResult {
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
            role_name: o.get_field("roleName"),
            synapse_spark_pool_id: o.get_field("synapseSparkPoolId"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
        }
    }
}
