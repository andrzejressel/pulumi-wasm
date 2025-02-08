/// Allows you to set a user or group as the AD administrator for a PostgreSQL Flexible Server.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFlexibleServer:
///     type: azure:postgresql:FlexibleServer
///     name: example
///     properties:
///       name: example-fs
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       administratorLogin: adminTerraform
///       administratorPassword: QAZwsx123
///       storageMb: 32768
///       version: '12'
///       skuName: GP_Standard_D2s_v3
///       zone: '2'
///       authentication:
///         activeDirectoryAuthEnabled: true
///         tenantId: ${current.tenantId}
///   exampleFlexibleServerActiveDirectoryAdministrator:
///     type: azure:postgresql:FlexibleServerActiveDirectoryAdministrator
///     name: example
///     properties:
///       serverName: ${exampleFlexibleServer.name}
///       resourceGroupName: ${exampleResourceGroup.name}
///       tenantId: ${current.tenantId}
///       objectId: ${example.objectId}
///       principalName: ${example.displayName}
///       principalType: ServicePrincipal
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         objectId: ${current.objectId}
/// ```
///
/// ## Import
///
/// A PostgreSQL Flexible Server Active Directory Administrator can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServerActiveDirectoryAdministrator:FlexibleServerActiveDirectoryAdministrator example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.DBforPostgreSQL/flexibleServers/myserver/administrators/objectId
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_server_active_directory_administrator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratorArgs {
        /// The object ID of a user, service principal or security group in the Azure Active Directory tenant set as the Flexible Server Admin. Changing this forces a new resource to be created.
        #[builder(into)]
        pub object_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of Azure Active Directory principal. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Azure Active Directory principal. Possible values are `Group`, `ServicePrincipal` and `User`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group for the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the PostgreSQL Flexible Server on which to set the administrator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Tenant ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratorResult {
        /// The object ID of a user, service principal or security group in the Azure Active Directory tenant set as the Flexible Server Admin. Changing this forces a new resource to be created.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// The name of Azure Active Directory principal. Changing this forces a new resource to be created.
        pub principal_name: pulumi_gestalt_rust::Output<String>,
        /// The type of Azure Active Directory principal. Possible values are `Group`, `ServicePrincipal` and `User`. Changing this forces a new resource to be created.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group for the PostgreSQL Server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the PostgreSQL Flexible Server on which to set the administrator. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Tenant ID. Changing this forces a new resource to be created.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FlexibleServerActiveDirectoryAdministratorArgs,
    ) -> FlexibleServerActiveDirectoryAdministratorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let object_id_binding = args.object_id.get_output(context).get_inner();
        let principal_name_binding = args.principal_name.get_output(context).get_inner();
        let principal_type_binding = args.principal_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let server_name_binding = args.server_name.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServerActiveDirectoryAdministrator:FlexibleServerActiveDirectoryAdministrator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalName".into(),
                    value: &principal_name_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FlexibleServerActiveDirectoryAdministratorResult {
            object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectId"),
            ),
            principal_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalName"),
            ),
            principal_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            server_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverName"),
            ),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
