pub mod get_managed_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedInstanceArgs {
        /// The name of the SQL Managed Instance.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the SQL Managed Instance exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedInstanceResult {
        /// The administrator login name for the SQL Managed Instance.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be collated.
        pub collation: pulumi_wasm_rust::Output<String>,
        pub customer_managed_key_id: pulumi_wasm_rust::Output<String>,
        /// The Dns Zone where the SQL Managed Instance is located.
        pub dns_zone: pulumi_wasm_rust::Output<String>,
        /// The ID of the SQL Managed Instance which shares the DNS zone.
        pub dns_zone_partner_id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the Azure Managed SQL Instance.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mssql::GetManagedInstanceIdentity>,
        >,
        /// What type of license the SQL Managed Instance uses.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Minimum TLS Version.
        pub minimum_tls_version: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be accessed.
        pub proxy_override: pulumi_wasm_rust::Output<String>,
        /// Whether the public data endpoint is enabled.
        pub public_data_endpoint_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the SKU Name of the SQL Managed Instance.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account type used to store backups for this database.
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// Maximum storage space allocated for the SQL Managed Instance.
        pub storage_size_in_gb: pulumi_wasm_rust::Output<i32>,
        /// The subnet resource ID that the SQL Managed Instance is associated with.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The TimeZone ID that the SQL Managed Instance is running in.
        pub timezone_id: pulumi_wasm_rust::Output<String>,
        /// Number of cores that are assigned to the SQL Managed Instance.
        pub vcores: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetManagedInstanceArgs) -> GetManagedInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getManagedInstance:getManagedInstance".into(),
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
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyId".into(),
                },
                register_interface::ResultField {
                    name: "dnsZone".into(),
                },
                register_interface::ResultField {
                    name: "dnsZonePartnerId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "proxyOverride".into(),
                },
                register_interface::ResultField {
                    name: "publicDataEndpointEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountType".into(),
                },
                register_interface::ResultField {
                    name: "storageSizeInGb".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timezoneId".into(),
                },
                register_interface::ResultField {
                    name: "vcores".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedInstanceResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            customer_managed_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyId").unwrap(),
            ),
            dns_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsZone").unwrap(),
            ),
            dns_zone_partner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsZonePartnerId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            proxy_override: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyOverride").unwrap(),
            ),
            public_data_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDataEndpointEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_account_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountType").unwrap(),
            ),
            storage_size_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageSizeInGb").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timezone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezoneId").unwrap(),
            ),
            vcores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcores").unwrap(),
            ),
        }
    }
}
