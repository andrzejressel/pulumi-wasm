/// Allows you to manage an Azure SQL Elastic Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("my-resource-group")
///             .build_struct(),
///     );
///     let exampleElasticPool = elastic_pool::create(
///         "exampleElasticPool",
///         ElasticPoolArgs::builder()
///             .license_type("LicenseIncluded")
///             .location("${example.location}")
///             .max_size_gb(756)
///             .name("test-epool")
///             .per_database_settings(
///                 ElasticPoolPerDatabaseSettings::builder()
///                     .maxCapacity(4)
///                     .minCapacity(0.25)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .sku(
///                 ElasticPoolSku::builder()
///                     .capacity(4)
///                     .family("Gen4")
///                     .name("BasicPool")
///                     .tier("Basic")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("my-sql-server")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Elastic Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/elasticPool:ElasticPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver/elasticPools/myelasticpoolname
/// ```
///
pub mod elastic_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticPoolArgs {
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the elastic pool. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** All databases that are added to the elastic pool must have the same `enclave_type` as the elastic pool.
        ///
        /// > **NOTE:** `enclave_type` is not supported for DC-series SKUs.
        ///
        /// > **NOTE:** The default value for `enclave_type` field is unset not `Default`.
        #[builder(into, default)]
        pub enclave_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        #[builder(into, default)]
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Public Maintenance Configuration window to apply to the elastic pool. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`, `SQL_SouthAfricaNorth_DB_1`, `SQL_SouthAfricaNorth_DB_2`, `SQL_WestUS3_DB_1`, `SQL_WestUS3_DB_2`. Defaults to `SQL_Default`.
        #[builder(into, default)]
        pub maintenance_configuration_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The max data size of the elastic pool in bytes. Conflicts with `max_size_gb`.
        ///
        /// > **NOTE:** One of either `max_size_gb` or `max_size_bytes` must be specified.
        #[builder(into, default)]
        pub max_size_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The max data size of the elastic pool in gigabytes. Conflicts with `max_size_bytes`.
        #[builder(into, default)]
        pub max_size_gb: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the elastic pool. This needs to be globally unique. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `per_database_settings` block as defined below.
        #[builder(into)]
        pub per_database_settings: pulumi_wasm_rust::Output<
            super::super::types::mssql::ElasticPoolPerDatabaseSettings,
        >,
        /// The name of the resource group in which to create the elastic pool. This must be the same as the resource group of the underlying SQL server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL Server on which to create the elastic pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<super::super::types::mssql::ElasticPoolSku>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether or not this elastic pool is zone redundant. `tier` needs to be `Premium` for `DTU` based or `BusinessCritical` for `vCore` based `sku`.
        #[builder(into, default)]
        pub zone_redundant: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ElasticPoolResult {
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the elastic pool. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** All databases that are added to the elastic pool must have the same `enclave_type` as the elastic pool.
        ///
        /// > **NOTE:** `enclave_type` is not supported for DC-series SKUs.
        ///
        /// > **NOTE:** The default value for `enclave_type` field is unset not `Default`.
        pub enclave_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Public Maintenance Configuration window to apply to the elastic pool. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`, `SQL_SouthAfricaNorth_DB_1`, `SQL_SouthAfricaNorth_DB_2`, `SQL_WestUS3_DB_1`, `SQL_WestUS3_DB_2`. Defaults to `SQL_Default`.
        pub maintenance_configuration_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The max data size of the elastic pool in bytes. Conflicts with `max_size_gb`.
        ///
        /// > **NOTE:** One of either `max_size_gb` or `max_size_bytes` must be specified.
        pub max_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The max data size of the elastic pool in gigabytes. Conflicts with `max_size_bytes`.
        pub max_size_gb: pulumi_wasm_rust::Output<f64>,
        /// The name of the elastic pool. This needs to be globally unique. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `per_database_settings` block as defined below.
        pub per_database_settings: pulumi_wasm_rust::Output<
            super::super::types::mssql::ElasticPoolPerDatabaseSettings,
        >,
        /// The name of the resource group in which to create the elastic pool. This must be the same as the resource group of the underlying SQL server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL Server on which to create the elastic pool. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<super::super::types::mssql::ElasticPoolSku>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether or not this elastic pool is zone redundant. `tier` needs to be `Premium` for `DTU` based or `BusinessCritical` for `vCore` based `sku`.
        pub zone_redundant: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ElasticPoolArgs) -> ElasticPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enclave_type_binding = args.enclave_type.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_configuration_name_binding = args
            .maintenance_configuration_name
            .get_inner();
        let max_size_bytes_binding = args.max_size_bytes.get_inner();
        let max_size_gb_binding = args.max_size_gb.get_inner();
        let name_binding = args.name.get_inner();
        let per_database_settings_binding = args.per_database_settings.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_redundant_binding = args.zone_redundant.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/elasticPool:ElasticPool".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enclaveType".into(),
                    value: &enclave_type_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationName".into(),
                    value: &maintenance_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "maxSizeBytes".into(),
                    value: &max_size_bytes_binding,
                },
                register_interface::ObjectField {
                    name: "maxSizeGb".into(),
                    value: &max_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "perDatabaseSettings".into(),
                    value: &per_database_settings_binding,
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
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enclaveType".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfigurationName".into(),
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
                    name: "perDatabaseSettings".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zoneRedundant".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ElasticPoolResult {
            enclave_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveType").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfigurationName").unwrap(),
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
            per_database_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perDatabaseSettings").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
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
