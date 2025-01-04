pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The Name of the Search Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Search Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Describes whether the search service is compliant or not with respect to having non-customer encrypted resources. If a service has more than one non-customer encrypted resource and `Enforcement` is `enabled` then the service will be marked as `NonCompliant`. If all the resources are customer encrypted, then the service will be marked as `Compliant`.
        pub customer_managed_key_encryption_compliance_status: pulumi_wasm_rust::Output<
            String,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::search::GetServiceIdentity>,
        >,
        /// The name of this Query Key.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of partitions which have been created.
        pub partition_count: pulumi_wasm_rust::Output<i32>,
        /// The Primary Key used for Search Service Administration.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is enabled for this resource.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// A `query_keys` block as defined below.
        pub query_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::search::GetServiceQueryKey>,
        >,
        /// The number of replica's which have been created.
        pub replica_count: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key used for Search Service Administration.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
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
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:search/getService:getService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customerManagedKeyEncryptionComplianceStatus".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partitionCount".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "queryKeys".into(),
                },
                register_interface::ResultField {
                    name: "replicaCount".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
            customer_managed_key_encryption_compliance_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyEncryptionComplianceStatus").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partition_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionCount").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            query_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryKeys").unwrap(),
            ),
            replica_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaCount").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
