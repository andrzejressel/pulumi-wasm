pub mod get_elastic_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticPoolArgs {
        /// The name of the elastic pool.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group which contains the elastic pool.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL Server which contains the elastic pool.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetElasticPoolResult {
        /// The type of enclave being used by the elastic pool.
        pub enclave_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The license type to apply for this elastic pool.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The max data size of the elastic pool in bytes.
        pub max_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The max data size of the elastic pool in gigabytes.
        pub max_size_gb: pulumi_wasm_rust::Output<f64>,
        /// Specifies the SKU Name for this Elasticpool.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The maximum capacity any one database can consume.
        pub per_db_max_capacity: pulumi_wasm_rust::Output<i32>,
        /// The minimum capacity all databases are guaranteed.
        pub per_db_min_capacity: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub skus: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mssql::GetElasticPoolSkus>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not this elastic pool is zone redundant.
        pub zone_redundant: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetElasticPoolArgs) -> GetElasticPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getElasticPool:getElasticPool".into(),
            version: super::super::super::get_version(),
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
                    name: "serverName".into(),
                    value: &server_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enclaveType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "maxSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "perDbMaxCapacity".into(),
                },
                register_interface::ResultField {
                    name: "perDbMinCapacity".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "skus".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zoneRedundant".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetElasticPoolResult {
            enclave_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSizeBytes").unwrap(),
            ),
            max_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSizeGb").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            per_db_max_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perDbMaxCapacity").unwrap(),
            ),
            per_db_min_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perDbMinCapacity").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            skus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skus").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zone_redundant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneRedundant").unwrap(),
            ),
        }
    }
}
