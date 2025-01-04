pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// A unique ID for the managed domain deployment.
        pub deployment_id: pulumi_wasm_rust::Output<String>,
        /// The forest type used by the managed domain. One of `ResourceTrusting`, for a _Resource Forest_, or blank, for a _User Forest_.
        pub domain_configuration_type: pulumi_wasm_rust::Output<String>,
        /// The Active Directory domain of the Domain Service. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Whether group-based filtered sync (also called scoped synchronisation) is enabled.
        pub filtered_sync_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location in which the replica set resides.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notifications` block as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceNotification>,
        >,
        /// One or more `replica_set` blocks as defined below.
        pub replica_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceReplicaSet>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// A `secure_ldap` block as defined below.
        pub secure_ldaps: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceSecureLdap>,
        >,
        /// A `security` block as defined below.
        pub securities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::domainservices::GetServiceSecurity>,
        >,
        /// The SKU of the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
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
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:domainservices/getService:getService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
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
                    name: "replicaSets".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "secureLdaps".into(),
                },
                register_interface::ResultField {
                    name: "securities".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            replica_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaSets").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            secure_ldaps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secureLdaps").unwrap(),
            ),
            securities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securities").unwrap(),
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
