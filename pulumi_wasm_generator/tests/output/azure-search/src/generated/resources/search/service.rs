/// Manages a Search Service.
///
/// ## Example Usage
///
/// ### Supporting API Keys)
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-resource")
///             .resource_group_name("${example.name}")
///             .sku("standard")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Using Both AzureAD And API Keys)
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .authentication_failure_mode("http403")
///             .local_authentication_enabled(true)
///             .location("${example.location}")
///             .name("example-resource")
///             .resource_group_name("${example.name}")
///             .sku("standard")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Supporting Only AzureAD Authentication)
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .local_authentication_enabled(false)
///             .location("${example.location}")
///             .name("example-resource")
///             .resource_group_name("${example.name}")
///             .sku("standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Search Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:search/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Search/searchServices/service1
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Specifies a list of inbound IPv4 or CIDRs that are allowed to access the Search Service. If the incoming IP request is from an IP address which is not included in the `allowed_ips` it will be blocked by the Search Services firewall.
        ///
        /// > **NOTE:** The `allowed_ips` are only applied if the `public_network_access_enabled` field has been set to `true`, else all traffic over the public interface will be rejected, even if the `allowed_ips` field has been defined. When the `public_network_access_enabled` field has been set to `false` the private endpoint connections are the only allowed access point to the Search Service.
        #[builder(into, default)]
        pub allowed_ips: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the response that the Search Service should return for requests that fail authentication. Possible values include `http401WithBearerChallenge` or `http403`.
        ///
        /// > **NOTE:** `authentication_failure_mode` can only be configured when using `local_authentication_enabled` is set to `true` - which when set together specifies that both API Keys and AzureAD Authentication should be supported.
        #[builder(into, default)]
        pub authentication_failure_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the Search Service should enforce that non-customer resources are encrypted. Defaults to `false`.
        #[builder(into, default)]
        pub customer_managed_key_enforcement_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Hosting Mode, which allows for High Density partitions (that allow for up to 1000 indexes) should be supported. Possible values are `highDensity` or `default`. Defaults to `default`. Changing this forces a new Search Service to be created.
        ///
        /// > **NOTE:** `hosting_mode` can only be configured when `sku` is set to `standard3`.
        #[builder(into, default)]
        pub hosting_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::search::ServiceIdentity>,
        >,
        /// Specifies whether the Search Service allows authenticating using API Keys? Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Search Service should exist. Changing this forces a new Search Service to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name which should be used for this Search Service. Changing this forces a new Search Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `None`.
        #[builder(into, default)]
        pub network_rule_bypass_option: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the number of partitions which should be created. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)). Possible values include `1`, `2`, `3`, `4`, `6`, or `12`. Defaults to `1`.
        ///
        /// > **NOTE:** when `hosting_mode` is set to `highDensity` the maximum number of partitions allowed is `3`.
        #[builder(into, default)]
        pub partition_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies whether Public Network Access is allowed for this resource. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the number of Replica's which should be created for this Search Service. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)).
        #[builder(into, default)]
        pub replica_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the Search Service should exist. Changing this forces a new Search Service to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Semantic Search SKU which should be used for this Search Service. Possible values include `free` and `standard`.
        ///
        /// > **NOTE:** The `semantic_search_sku` cannot be defined if your Search Services `sku` is set to `free`. The Semantic Search feature is only available in certain regions, please see the [product documentation](https://learn.microsoft.com/azure/search/semantic-search-overview#availability-and-pricing) for more information.
        #[builder(into, default)]
        pub semantic_search_sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU which should be used for this Search Service. Possible values include `basic`, `free`, `standard`, `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2`. Changing this forces a new Search Service to be created.
        ///
        /// > The `basic` and `free` SKUs provision the Search Service in a Shared Cluster - the `standard` SKUs use a Dedicated Cluster.
        ///
        /// > **NOTE:** The SKUs `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2` are only available by submitting a quota increase request to Microsoft. Please see the [product documentation](https://learn.microsoft.com/azure/azure-resource-manager/troubleshooting/error-resource-quota?tabs=azure-cli) on how to submit a quota increase request.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// Specifies a mapping of tags which should be assigned to this Search Service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Specifies a list of inbound IPv4 or CIDRs that are allowed to access the Search Service. If the incoming IP request is from an IP address which is not included in the `allowed_ips` it will be blocked by the Search Services firewall.
        ///
        /// > **NOTE:** The `allowed_ips` are only applied if the `public_network_access_enabled` field has been set to `true`, else all traffic over the public interface will be rejected, even if the `allowed_ips` field has been defined. When the `public_network_access_enabled` field has been set to `false` the private endpoint connections are the only allowed access point to the Search Service.
        pub allowed_ips: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the response that the Search Service should return for requests that fail authentication. Possible values include `http401WithBearerChallenge` or `http403`.
        ///
        /// > **NOTE:** `authentication_failure_mode` can only be configured when using `local_authentication_enabled` is set to `true` - which when set together specifies that both API Keys and AzureAD Authentication should be supported.
        pub authentication_failure_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes whether the search service is compliant or not with respect to having non-customer encrypted resources. If a service has more than one non-customer encrypted resource and `Enforcement` is `enabled` then the service will be marked as `NonCompliant`. If all the resources are customer encrypted, then the service will be marked as `Compliant`.
        pub customer_managed_key_encryption_compliance_status: pulumi_wasm_rust::Output<
            String,
        >,
        /// Specifies whether the Search Service should enforce that non-customer resources are encrypted. Defaults to `false`.
        pub customer_managed_key_enforcement_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Hosting Mode, which allows for High Density partitions (that allow for up to 1000 indexes) should be supported. Possible values are `highDensity` or `default`. Defaults to `default`. Changing this forces a new Search Service to be created.
        ///
        /// > **NOTE:** `hosting_mode` can only be configured when `sku` is set to `standard3`.
        pub hosting_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::search::ServiceIdentity>,
        >,
        /// Specifies whether the Search Service allows authenticating using API Keys? Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Search Service should exist. Changing this forces a new Search Service to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Name which should be used for this Search Service. Changing this forces a new Search Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `None`.
        pub network_rule_bypass_option: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the number of partitions which should be created. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)). Possible values include `1`, `2`, `3`, `4`, `6`, or `12`. Defaults to `1`.
        ///
        /// > **NOTE:** when `hosting_mode` is set to `highDensity` the maximum number of partitions allowed is `3`.
        pub partition_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Primary Key used for Search Service Administration.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// Specifies whether Public Network Access is allowed for this resource. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `query_keys` block as defined below.
        pub query_keys: pulumi_wasm_rust::Output<
            Vec<super::super::types::search::ServiceQueryKey>,
        >,
        /// Specifies the number of Replica's which should be created for this Search Service. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)).
        pub replica_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the Search Service should exist. Changing this forces a new Search Service to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key used for Search Service Administration.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the Semantic Search SKU which should be used for this Search Service. Possible values include `free` and `standard`.
        ///
        /// > **NOTE:** The `semantic_search_sku` cannot be defined if your Search Services `sku` is set to `free`. The Semantic Search feature is only available in certain regions, please see the [product documentation](https://learn.microsoft.com/azure/search/semantic-search-overview#availability-and-pricing) for more information.
        pub semantic_search_sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU which should be used for this Search Service. Possible values include `basic`, `free`, `standard`, `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2`. Changing this forces a new Search Service to be created.
        ///
        /// > The `basic` and `free` SKUs provision the Search Service in a Shared Cluster - the `standard` SKUs use a Dedicated Cluster.
        ///
        /// > **NOTE:** The SKUs `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2` are only available by submitting a quota increase request to Microsoft. Please see the [product documentation](https://learn.microsoft.com/azure/azure-resource-manager/troubleshooting/error-resource-quota?tabs=azure-cli) on how to submit a quota increase request.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// Specifies a mapping of tags which should be assigned to this Search Service.
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
        let allowed_ips_binding = args.allowed_ips.get_inner();
        let authentication_failure_mode_binding = args
            .authentication_failure_mode
            .get_inner();
        let customer_managed_key_enforcement_enabled_binding = args
            .customer_managed_key_enforcement_enabled
            .get_inner();
        let hosting_mode_binding = args.hosting_mode.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_rule_bypass_option_binding = args
            .network_rule_bypass_option
            .get_inner();
        let partition_count_binding = args.partition_count.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let replica_count_binding = args.replica_count.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let semantic_search_sku_binding = args.semantic_search_sku.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:search/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedIps".into(),
                    value: &allowed_ips_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationFailureMode".into(),
                    value: &authentication_failure_mode_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKeyEnforcementEnabled".into(),
                    value: &customer_managed_key_enforcement_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hostingMode".into(),
                    value: &hosting_mode_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding,
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
                    name: "networkRuleBypassOption".into(),
                    value: &network_rule_bypass_option_binding,
                },
                register_interface::ObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "replicaCount".into(),
                    value: &replica_count_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "semanticSearchSku".into(),
                    value: &semantic_search_sku_binding,
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
                    name: "allowedIps".into(),
                },
                register_interface::ResultField {
                    name: "authenticationFailureMode".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyEncryptionComplianceStatus".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyEnforcementEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hostingMode".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRuleBypassOption".into(),
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
                    name: "semanticSearchSku".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
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
            allowed_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedIps").unwrap(),
            ),
            authentication_failure_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationFailureMode").unwrap(),
            ),
            customer_managed_key_encryption_compliance_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyEncryptionComplianceStatus").unwrap(),
            ),
            customer_managed_key_enforcement_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyEnforcementEnabled").unwrap(),
            ),
            hosting_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostingMode").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthenticationEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rule_bypass_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRuleBypassOption").unwrap(),
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
            semantic_search_sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("semanticSearchSku").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
