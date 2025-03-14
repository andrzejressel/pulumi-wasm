/// Manages a Search Service.
///
/// ## Example Usage
///
/// ### Supporting API Keys)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Specifies a list of inbound IPv4 or CIDRs that are allowed to access the Search Service. If the incoming IP request is from an IP address which is not included in the `allowed_ips` it will be blocked by the Search Services firewall.
        ///
        /// > **NOTE:** The `allowed_ips` are only applied if the `public_network_access_enabled` field has been set to `true`, else all traffic over the public interface will be rejected, even if the `allowed_ips` field has been defined. When the `public_network_access_enabled` field has been set to `false` the private endpoint connections are the only allowed access point to the Search Service.
        #[builder(into, default)]
        pub allowed_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the response that the Search Service should return for requests that fail authentication. Possible values include `http401WithBearerChallenge` or `http403`.
        ///
        /// > **NOTE:** `authentication_failure_mode` can only be configured when using `local_authentication_enabled` is set to `true` - which when set together specifies that both API Keys and AzureAD Authentication should be supported.
        #[builder(into, default)]
        pub authentication_failure_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether the Search Service should enforce that non-customer resources are encrypted. Defaults to `false`.
        #[builder(into, default)]
        pub customer_managed_key_enforcement_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the Hosting Mode, which allows for High Density partitions (that allow for up to 1000 indexes) should be supported. Possible values are `highDensity` or `default`. Defaults to `default`. Changing this forces a new Search Service to be created.
        ///
        /// > **NOTE:** `hosting_mode` can only be configured when `sku` is set to `standard3`.
        #[builder(into, default)]
        pub hosting_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::search::ServiceIdentity>,
        >,
        /// Specifies whether the Search Service allows authenticating using API Keys? Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Azure Region where the Search Service should exist. Changing this forces a new Search Service to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name which should be used for this Search Service. Changing this forces a new Search Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `None`.
        #[builder(into, default)]
        pub network_rule_bypass_option: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the number of partitions which should be created. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)). Possible values include `1`, `2`, `3`, `4`, `6`, or `12`. Defaults to `1`.
        ///
        /// > **NOTE:** when `hosting_mode` is set to `highDensity` the maximum number of partitions allowed is `3`.
        #[builder(into, default)]
        pub partition_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether Public Network Access is allowed for this resource. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the number of Replica's which should be created for this Search Service. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)).
        #[builder(into, default)]
        pub replica_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Resource Group where the Search Service should exist. Changing this forces a new Search Service to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Semantic Search SKU which should be used for this Search Service. Possible values include `free` and `standard`.
        ///
        /// > **NOTE:** The `semantic_search_sku` cannot be defined if your Search Services `sku` is set to `free`. The Semantic Search feature is only available in certain regions, please see the [product documentation](https://learn.microsoft.com/azure/search/semantic-search-overview#availability-and-pricing) for more information.
        #[builder(into, default)]
        pub semantic_search_sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SKU which should be used for this Search Service. Possible values include `basic`, `free`, `standard`, `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2`. Changing this forces a new Search Service to be created.
        ///
        /// > The `basic` and `free` SKUs provision the Search Service in a Shared Cluster - the `standard` SKUs use a Dedicated Cluster.
        ///
        /// > **NOTE:** The SKUs `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2` are only available by submitting a quota increase request to Microsoft. Please see the [product documentation](https://learn.microsoft.com/azure/azure-resource-manager/troubleshooting/error-resource-quota?tabs=azure-cli) on how to submit a quota increase request.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a mapping of tags which should be assigned to this Search Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Specifies a list of inbound IPv4 or CIDRs that are allowed to access the Search Service. If the incoming IP request is from an IP address which is not included in the `allowed_ips` it will be blocked by the Search Services firewall.
        ///
        /// > **NOTE:** The `allowed_ips` are only applied if the `public_network_access_enabled` field has been set to `true`, else all traffic over the public interface will be rejected, even if the `allowed_ips` field has been defined. When the `public_network_access_enabled` field has been set to `false` the private endpoint connections are the only allowed access point to the Search Service.
        pub allowed_ips: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the response that the Search Service should return for requests that fail authentication. Possible values include `http401WithBearerChallenge` or `http403`.
        ///
        /// > **NOTE:** `authentication_failure_mode` can only be configured when using `local_authentication_enabled` is set to `true` - which when set together specifies that both API Keys and AzureAD Authentication should be supported.
        pub authentication_failure_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Describes whether the search service is compliant or not with respect to having non-customer encrypted resources. If a service has more than one non-customer encrypted resource and `Enforcement` is `enabled` then the service will be marked as `NonCompliant`. If all the resources are customer encrypted, then the service will be marked as `Compliant`.
        pub customer_managed_key_encryption_compliance_status: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Specifies whether the Search Service should enforce that non-customer resources are encrypted. Defaults to `false`.
        pub customer_managed_key_enforcement_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Hosting Mode, which allows for High Density partitions (that allow for up to 1000 indexes) should be supported. Possible values are `highDensity` or `default`. Defaults to `default`. Changing this forces a new Search Service to be created.
        ///
        /// > **NOTE:** `hosting_mode` can only be configured when `sku` is set to `standard3`.
        pub hosting_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::search::ServiceIdentity>,
        >,
        /// Specifies whether the Search Service allows authenticating using API Keys? Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Search Service should exist. Changing this forces a new Search Service to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this Search Service. Changing this forces a new Search Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `None`.
        pub network_rule_bypass_option: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the number of partitions which should be created. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)). Possible values include `1`, `2`, `3`, `4`, `6`, or `12`. Defaults to `1`.
        ///
        /// > **NOTE:** when `hosting_mode` is set to `highDensity` the maximum number of partitions allowed is `3`.
        pub partition_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Primary Key used for Search Service Administration.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether Public Network Access is allowed for this resource. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `query_keys` block as defined below.
        pub query_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::search::ServiceQueryKey>,
        >,
        /// Specifies the number of Replica's which should be created for this Search Service. This field cannot be set when using a `free` sku ([see the Microsoft documentation](https://learn.microsoft.com/azure/search/search-sku-tier)).
        pub replica_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the Search Service should exist. Changing this forces a new Search Service to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key used for Search Service Administration.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Semantic Search SKU which should be used for this Search Service. Possible values include `free` and `standard`.
        ///
        /// > **NOTE:** The `semantic_search_sku` cannot be defined if your Search Services `sku` is set to `free`. The Semantic Search feature is only available in certain regions, please see the [product documentation](https://learn.microsoft.com/azure/search/semantic-search-overview#availability-and-pricing) for more information.
        pub semantic_search_sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SKU which should be used for this Search Service. Possible values include `basic`, `free`, `standard`, `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2`. Changing this forces a new Search Service to be created.
        ///
        /// > The `basic` and `free` SKUs provision the Search Service in a Shared Cluster - the `standard` SKUs use a Dedicated Cluster.
        ///
        /// > **NOTE:** The SKUs `standard2`, `standard3`, `storage_optimized_l1` and `storage_optimized_l2` are only available by submitting a quota increase request to Microsoft. Please see the [product documentation](https://learn.microsoft.com/azure/azure-resource-manager/troubleshooting/error-resource-quota?tabs=azure-cli) on how to submit a quota increase request.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Specifies a mapping of tags which should be assigned to this Search Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_ips_binding = args.allowed_ips.get_output(context);
        let authentication_failure_mode_binding = args
            .authentication_failure_mode
            .get_output(context);
        let customer_managed_key_enforcement_enabled_binding = args
            .customer_managed_key_enforcement_enabled
            .get_output(context);
        let hosting_mode_binding = args.hosting_mode.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_rule_bypass_option_binding = args
            .network_rule_bypass_option
            .get_output(context);
        let partition_count_binding = args.partition_count.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let replica_count_binding = args.replica_count.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let semantic_search_sku_binding = args.semantic_search_sku.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:search/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedIps".into(),
                    value: &allowed_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationFailureMode".into(),
                    value: &authentication_failure_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyEnforcementEnabled".into(),
                    value: &customer_managed_key_enforcement_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostingMode".into(),
                    value: &hosting_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkRuleBypassOption".into(),
                    value: &network_rule_bypass_option_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaCount".into(),
                    value: &replica_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "semanticSearchSku".into(),
                    value: &semantic_search_sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            allowed_ips: o.get_field("allowedIps"),
            authentication_failure_mode: o.get_field("authenticationFailureMode"),
            customer_managed_key_encryption_compliance_status: o
                .get_field("customerManagedKeyEncryptionComplianceStatus"),
            customer_managed_key_enforcement_enabled: o
                .get_field("customerManagedKeyEnforcementEnabled"),
            hosting_mode: o.get_field("hostingMode"),
            identity: o.get_field("identity"),
            local_authentication_enabled: o.get_field("localAuthenticationEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_rule_bypass_option: o.get_field("networkRuleBypassOption"),
            partition_count: o.get_field("partitionCount"),
            primary_key: o.get_field("primaryKey"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            query_keys: o.get_field("queryKeys"),
            replica_count: o.get_field("replicaCount"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_key: o.get_field("secondaryKey"),
            semantic_search_sku: o.get_field("semanticSearchSku"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
