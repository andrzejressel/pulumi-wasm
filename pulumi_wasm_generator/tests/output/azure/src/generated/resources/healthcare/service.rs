/// Manages a Healthcare Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:healthcare:Service
///     properties:
///       name: uniquefhirname
///       resourceGroupName: sample-resource-group
///       location: westus2
///       kind: fhir-R4
///       cosmosdbThroughput: '2000'
///       identity:
///         type: SystemAssigned
///       accessPolicyObjectIds: ${current.objectId}
///       configurationExportStorageAccountName: teststorage
///       tags:
///         environment: testenv
///         purpose: AcceptanceTests
///       authenticationConfiguration:
///         authority: https://login.microsoftonline.com/$%7Bdata.azurerm_client_config.current.tenant_id%7D
///         audience: https://azurehealthcareapis.com/
///         smartProxyEnabled: 'true'
///       corsConfiguration:
///         allowedOrigins:
///           - http://www.example.com
///           - http://www.example2.com
///         allowedHeaders:
///           - x-tempo-*
///           - x-tempo2-*
///         allowedMethods:
///           - GET
///           - PUT
///         maxAgeInSeconds: '500'
///         allowCredentials: 'true'
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Healthcare Service can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource_group/providers/Microsoft.HealthcareApis/services/service_name
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        #[builder(into, default)]
        pub access_policy_object_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `authentication_configuration` block as defined below.
        #[builder(into, default)]
        pub authentication_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ServiceAuthenticationConfiguration>,
        >,
        /// Specifies the name of the storage account which the operation configuration information is exported to.
        #[builder(into, default)]
        pub configuration_export_storage_account_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// A `cors_configuration` block as defined below.
        #[builder(into, default)]
        pub cors_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ServiceCorsConfiguration>,
        >,
        /// A versionless Key Vault Key ID for CMK encryption of the backing database. Changing this forces a new resource to be created.
        ///
        /// > **Please Note** In order to use a `Custom Key` from Key Vault for encryption you must grant Azure Cosmos DB Service access to your key vault. For instructions on how to configure your Key Vault correctly please refer to the [product documentation](https://docs.microsoft.com/azure/cosmos-db/how-to-setup-cmk#add-an-access-policy-to-your-azure-key-vault-instance)
        #[builder(into, default)]
        pub cosmosdb_key_vault_key_versionless_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The provisioned throughput for the backing database. Range of `400`-`100000`. Defaults to `1000`.
        #[builder(into, default)]
        pub cosmosdb_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ServiceIdentity>,
        >,
        /// The type of the service. Values at time of publication are: `fhir`, `fhir-Stu3` and `fhir-R4`. Default value is `fhir`.
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure Region where the Service should be created. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Not all locations support this resource. Some are `West US 2`, `North Central US`, and `UK West`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service instance. Used for service endpoint, must be unique within the audience. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is enabled or disabled for this service instance. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group in which to create the Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        pub access_policy_object_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `authentication_configuration` block as defined below.
        pub authentication_configuration: pulumi_wasm_rust::Output<
            super::super::types::healthcare::ServiceAuthenticationConfiguration,
        >,
        /// Specifies the name of the storage account which the operation configuration information is exported to.
        pub configuration_export_storage_account_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// A `cors_configuration` block as defined below.
        pub cors_configuration: pulumi_wasm_rust::Output<
            super::super::types::healthcare::ServiceCorsConfiguration,
        >,
        /// A versionless Key Vault Key ID for CMK encryption of the backing database. Changing this forces a new resource to be created.
        ///
        /// > **Please Note** In order to use a `Custom Key` from Key Vault for encryption you must grant Azure Cosmos DB Service access to your key vault. For instructions on how to configure your Key Vault correctly please refer to the [product documentation](https://docs.microsoft.com/azure/cosmos-db/how-to-setup-cmk#add-an-access-policy-to-your-azure-key-vault-instance)
        pub cosmosdb_key_vault_key_versionless_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The provisioned throughput for the backing database. Range of `400`-`100000`. Defaults to `1000`.
        pub cosmosdb_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ServiceIdentity>,
        >,
        /// The type of the service. Values at time of publication are: `fhir`, `fhir-Stu3` and `fhir-R4`. Default value is `fhir`.
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure Region where the Service should be created. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Not all locations support this resource. Some are `West US 2`, `North Central US`, and `UK West`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the service instance. Used for service endpoint, must be unique within the audience. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is enabled or disabled for this service instance. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group in which to create the Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policy_object_ids_binding = args.access_policy_object_ids.get_inner();
        let authentication_configuration_binding = args
            .authentication_configuration
            .get_inner();
        let configuration_export_storage_account_name_binding = args
            .configuration_export_storage_account_name
            .get_inner();
        let cors_configuration_binding = args.cors_configuration.get_inner();
        let cosmosdb_key_vault_key_versionless_id_binding = args
            .cosmosdb_key_vault_key_versionless_id
            .get_inner();
        let cosmosdb_throughput_binding = args.cosmosdb_throughput.get_inner();
        let identity_binding = args.identity.get_inner();
        let kind_binding = args.kind.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicyObjectIds".into(),
                    value: &access_policy_object_ids_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationConfiguration".into(),
                    value: &authentication_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "configurationExportStorageAccountName".into(),
                    value: &configuration_export_storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "corsConfiguration".into(),
                    value: &cors_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "cosmosdbKeyVaultKeyVersionlessId".into(),
                    value: &cosmosdb_key_vault_key_versionless_id_binding,
                },
                register_interface::ObjectField {
                    name: "cosmosdbThroughput".into(),
                    value: &cosmosdb_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
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
                    name: "accessPolicyObjectIds".into(),
                },
                register_interface::ResultField {
                    name: "authenticationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "configurationExportStorageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "corsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "cosmosdbKeyVaultKeyVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "cosmosdbThroughput".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
            access_policy_object_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicyObjectIds").unwrap(),
            ),
            authentication_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationConfiguration").unwrap(),
            ),
            configuration_export_storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationExportStorageAccountName").unwrap(),
            ),
            cors_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsConfiguration").unwrap(),
            ),
            cosmosdb_key_vault_key_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cosmosdbKeyVaultKeyVersionlessId").unwrap(),
            ),
            cosmosdb_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cosmosdbThroughput").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}