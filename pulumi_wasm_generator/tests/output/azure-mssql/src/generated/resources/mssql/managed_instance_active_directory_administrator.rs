/// Allows you to set a user, group or service principal as the AAD Administrator for an Azure SQL Managed Instance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleManagedInstance:
///     type: azure:mssql:ManagedInstance
///     name: example
///     properties:
///       name: managedsqlinstance
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       licenseType: BasePrice
///       skuName: GP_Gen5
///       storageSizeInGb: 32
///       subnetId: ${exampleSubnet.id}
///       vcores: 4
///       administratorLogin: msadministrator
///       administratorLoginPassword: thisIsDog11
///       identity:
///         type: SystemAssigned
///   reader:
///     type: azuread:DirectoryRole
///     properties:
///       displayName: Directory Readers
///   exampleDirectoryRoleMember:
///     type: azuread:DirectoryRoleMember
///     name: example
///     properties:
///       roleObjectId: ${reader.objectId}
///       memberObjectId: ${exampleManagedInstance.identity.principalId}
///   admin:
///     type: azuread:User
///     properties:
///       userPrincipalName: ms.admin@example.com
///       displayName: Ms Admin
///       mailNickname: ms.admin
///       password: SecretP@sswd99!
///   exampleManagedInstanceActiveDirectoryAdministrator:
///     type: azure:mssql:ManagedInstanceActiveDirectoryAdministrator
///     name: example
///     properties:
///       managedInstanceId: ${exampleManagedInstance.id}
///       loginUsername: msadmin
///       objectId: ${admin.objectId}
///       tenantId: ${current.tenantId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An Azure SQL Active Directory Administrator can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/managedInstanceActiveDirectoryAdministrator:ManagedInstanceActiveDirectoryAdministrator administrator /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/managedInstances/mymanagedinstance/administrators/activeDirectory
/// ```
///
pub mod managed_instance_active_directory_administrator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedInstanceActiveDirectoryAdministratorArgs {
        /// When `true`, only permit logins from AAD users and administrators. When `false`, also allow local database users.
        #[builder(into, default)]
        pub azuread_authentication_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// The login name of the principal to set as the Managed Instance Administrator.
        #[builder(into)]
        pub login_username: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure SQL Managed Instance for which to set the administrator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_instance_id: pulumi_wasm_rust::Output<String>,
        /// The Object ID of the principal to set as the Managed Instance Administrator.
        #[builder(into)]
        pub object_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Active Directory Tenant ID.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedInstanceActiveDirectoryAdministratorResult {
        /// When `true`, only permit logins from AAD users and administrators. When `false`, also allow local database users.
        pub azuread_authentication_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// The login name of the principal to set as the Managed Instance Administrator.
        pub login_username: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure SQL Managed Instance for which to set the administrator. Changing this forces a new resource to be created.
        pub managed_instance_id: pulumi_wasm_rust::Output<String>,
        /// The Object ID of the principal to set as the Managed Instance Administrator.
        pub object_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Active Directory Tenant ID.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedInstanceActiveDirectoryAdministratorArgs,
    ) -> ManagedInstanceActiveDirectoryAdministratorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let azuread_authentication_only_binding = args
            .azuread_authentication_only
            .get_inner();
        let login_username_binding = args.login_username.get_inner();
        let managed_instance_id_binding = args.managed_instance_id.get_inner();
        let object_id_binding = args.object_id.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/managedInstanceActiveDirectoryAdministrator:ManagedInstanceActiveDirectoryAdministrator"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "azureadAuthenticationOnly".into(),
                    value: &azuread_authentication_only_binding,
                },
                register_interface::ObjectField {
                    name: "loginUsername".into(),
                    value: &login_username_binding,
                },
                register_interface::ObjectField {
                    name: "managedInstanceId".into(),
                    value: &managed_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureadAuthenticationOnly".into(),
                },
                register_interface::ResultField {
                    name: "loginUsername".into(),
                },
                register_interface::ResultField {
                    name: "managedInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "objectId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedInstanceActiveDirectoryAdministratorResult {
            azuread_authentication_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureadAuthenticationOnly").unwrap(),
            ),
            login_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loginUsername").unwrap(),
            ),
            managed_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedInstanceId").unwrap(),
            ),
            object_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}