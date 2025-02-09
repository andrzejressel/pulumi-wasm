/// Allows you to set a user or group as the AD administrator for an PostgreSQL server in Azure
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
///   exampleServer:
///     type: azure:postgresql:Server
///     name: example
///     properties:
///       name: example-psqlserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '9.6'
///       administratorLogin: 4dm1n157r470r
///       administratorLoginPassword: 4-v3ry-53cr37-p455w0rd
///       skuName: GP_Gen5_2
///       sslEnforcementEnabled: true
///   exampleActiveDirectoryAdministrator:
///     type: azure:postgresql:ActiveDirectoryAdministrator
///     name: example
///     properties:
///       serverName: ${exampleServer.name}
///       resourceGroupName: ${example.name}
///       login: sqladmin
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// A PostgreSQL Active Directory Administrator can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/activeDirectoryAdministrator:ActiveDirectoryAdministrator administrator /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.DBforPostgreSQL/servers/myserver
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod active_directory_administrator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveDirectoryAdministratorArgs {
        /// The login name of the principal to set as the server administrator
        #[builder(into)]
        pub login: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the principal to set as the server administrator. For a managed identity this should be the Client ID of the identity.
        #[builder(into)]
        pub object_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group for the PostgreSQL server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the PostgreSQL Server on which to set the administrator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Tenant ID
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ActiveDirectoryAdministratorResult {
        /// The login name of the principal to set as the server administrator
        pub login: pulumi_gestalt_rust::Output<String>,
        /// The ID of the principal to set as the server administrator. For a managed identity this should be the Client ID of the identity.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group for the PostgreSQL server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the PostgreSQL Server on which to set the administrator. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Tenant ID
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ActiveDirectoryAdministratorArgs,
    ) -> ActiveDirectoryAdministratorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let login_binding_1 = args.login.get_output(context);
        let login_binding = login_binding_1.get_inner();
        let object_id_binding_1 = args.object_id.get_output(context);
        let object_id_binding = object_id_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let server_name_binding_1 = args.server_name.get_output(context);
        let server_name_binding = server_name_binding_1.get_inner();
        let tenant_id_binding_1 = args.tenant_id.get_output(context);
        let tenant_id_binding = tenant_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/activeDirectoryAdministrator:ActiveDirectoryAdministrator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "login".into(),
                    value: &login_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
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
        ActiveDirectoryAdministratorResult {
            login: pulumi_gestalt_rust::__private::into_domain(o.extract_field("login")),
            object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectId"),
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
