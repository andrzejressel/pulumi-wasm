/// ## Example Usage
///
/// ```yaml
/// resources:
///   deploy:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   deployVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: deploy
///     properties:
///       name: deploy-vnet
///       location: ${deploy.location}
///       resourceGroupName: ${deploy.name}
///       addressSpaces:
///         - 10.0.1.0/16
///   deploySubnet:
///     type: azure:network:Subnet
///     name: deploy
///     properties:
///       name: deploy-subnet
///       resourceGroupName: ${deploy.name}
///       virtualNetworkName: ${deployVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   deployNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: deploy
///     properties:
///       name: deploy-nsg
///       location: ${deploy.location}
///       resourceGroupName: ${deploy.name}
///       securityRules:
///         - name: AllowSyncWithAzureAD
///           priority: 101
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '443'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowRD
///           priority: 201
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '3389'
///           sourceAddressPrefix: CorpNetSaw
///           destinationAddressPrefix: '*'
///         - name: AllowPSRemoting
///           priority: 301
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '5986'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowLDAPS
///           priority: 401
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '636'
///           sourceAddressPrefix: '*'
///           destinationAddressPrefix: '*'
///   deploySubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: deploy
///     properties:
///       subnetId: ${deploySubnet.id}
///       networkSecurityGroupId: ${deployNetworkSecurityGroup.id}
///   dcAdmins:
///     type: azuread:Group
///     name: dc_admins
///     properties:
///       displayName: AAD DC Administrators
///       securityEnabled: true
///   admin:
///     type: azuread:User
///     properties:
///       userPrincipalName: dc-admin@hashicorp-example.com
///       displayName: DC Administrator
///       password: Pa55w0Rd!!1
///   adminGroupMember:
///     type: azuread:GroupMember
///     name: admin
///     properties:
///       groupObjectId: ${dcAdmins.objectId}
///       memberObjectId: ${admin.objectId}
///   example:
///     type: azuread:ServicePrincipal
///     properties:
///       applicationId: 2565bd9d-da50-47d4-8b85-4c97f669dc36
///   aadds:
///     type: azure:core:ResourceGroup
///     properties:
///       name: aadds-rg
///       location: westeurope
///   exampleService:
///     type: azure:domainservices:Service
///     name: example
///     properties:
///       name: example-aadds
///       location: ${aadds.location}
///       resourceGroupName: ${aadds.name}
///       domainName: widgetslogin.net
///       sku: Enterprise
///       filteredSyncEnabled: false
///       initialReplicaSet:
///         subnetId: ${deploySubnet.id}
///       notifications:
///         additionalRecipients:
///           - notifyA@example.net
///           - notifyB@example.org
///         notifyDcAdmins: true
///         notifyGlobalAdmins: true
///       security:
///         syncKerberosPasswords: true
///         syncNtlmPasswords: true
///         syncOnPremPasswords: true
///       tags:
///         Environment: prod
///     options:
///       dependsOn:
///         - ${example}
///         - ${deploySubnetNetworkSecurityGroupAssociation}
/// ```
///
/// ## Import
///
/// Domain Services can be imported using the resource ID, together with the Replica Set ID that you wish to designate as the initial replica set, e.g.
///
/// ```sh
/// $ pulumi import azure:domainservices/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AAD/domainServices/instance1/initialReplicaSetId/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub domain_configuration_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Whether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
        #[builder(into, default)]
        pub filtered_sync_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
        #[builder(into)]
        pub initial_replica_set: pulumi_wasm_rust::Output<
            super::super::types::domainservices::ServiceInitialReplicaSet,
        >,
        /// The Azure location where the Domain Service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `notifications` block as defined below.
        #[builder(into, default)]
        pub notifications: pulumi_wasm_rust::Output<
            Option<super::super::types::domainservices::ServiceNotifications>,
        >,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `secure_ldap` block as defined below.
        #[builder(into, default)]
        pub secure_ldap: pulumi_wasm_rust::Output<
            Option<super::super::types::domainservices::ServiceSecureLdap>,
        >,
        /// A `security` block as defined below.
        #[builder(into, default)]
        pub security: pulumi_wasm_rust::Output<
            Option<super::super::types::domainservices::ServiceSecurity>,
        >,
        /// The SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// A unique ID for the managed domain deployment.
        pub deployment_id: pulumi_wasm_rust::Output<String>,
        /// The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
        pub domain_configuration_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Whether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
        pub filtered_sync_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
        pub initial_replica_set: pulumi_wasm_rust::Output<
            super::super::types::domainservices::ServiceInitialReplicaSet,
        >,
        /// The Azure location where the Domain Service exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notifications` block as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            super::super::types::domainservices::ServiceNotifications,
        >,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Azure resource ID for the domain service.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// A `secure_ldap` block as defined below.
        pub secure_ldap: pulumi_wasm_rust::Output<
            super::super::types::domainservices::ServiceSecureLdap,
        >,
        /// A `security` block as defined below.
        pub security: pulumi_wasm_rust::Output<
            super::super::types::domainservices::ServiceSecurity,
        >,
        /// The SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
        pub sku: pulumi_wasm_rust::Output<String>,
        pub sync_owner: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_configuration_type_binding = args
            .domain_configuration_type
            .get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let filtered_sync_enabled_binding = args.filtered_sync_enabled.get_inner();
        let initial_replica_set_binding = args.initial_replica_set.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let notifications_binding = args.notifications.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let secure_ldap_binding = args.secure_ldap.get_inner();
        let security_binding = args.security.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:domainservices/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainConfigurationType".into(),
                    value: &domain_configuration_type_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "filteredSyncEnabled".into(),
                    value: &filtered_sync_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "initialReplicaSet".into(),
                    value: &initial_replica_set_binding,
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
                    name: "notifications".into(),
                    value: &notifications_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "secureLdap".into(),
                    value: &secure_ldap_binding,
                },
                register_interface::ObjectField {
                    name: "security".into(),
                    value: &security_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deploymentId".into(),
                },
                register_interface::ResultField {
                    name: "domainConfigurationType".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "filteredSyncEnabled".into(),
                },
                register_interface::ResultField {
                    name: "initialReplicaSet".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notifications".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "secureLdap".into(),
                },
                register_interface::ResultField {
                    name: "security".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "syncOwner".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
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
        ServiceResult {
            deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentId").unwrap(),
            ),
            domain_configuration_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainConfigurationType").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            filtered_sync_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filteredSyncEnabled").unwrap(),
            ),
            initial_replica_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialReplicaSet").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            secure_ldap: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secureLdap").unwrap(),
            ),
            security: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("security").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            sync_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncOwner").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
