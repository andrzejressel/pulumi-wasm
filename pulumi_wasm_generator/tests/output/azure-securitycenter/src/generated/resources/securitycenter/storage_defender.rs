/// Manages the Defender for Storage.
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
///             .location("westus2")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("exampleacc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStorageDefender = storage_defender::create(
///         "exampleStorageDefender",
///         StorageDefenderArgs::builder()
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The setting can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/storageDefender:StorageDefender example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageacc
/// ```
///
pub mod storage_defender {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageDefenderArgs {
        /// The max GB to be scanned per Month. Must be `-1` or above `0`. Omit this property or set to `-1` if no capping is needed. Defaults to `-1`.
        #[builder(into, default)]
        pub malware_scanning_on_upload_cap_gb_per_month: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Whether On Upload malware scanning should be enabled. Defaults to `false`.
        #[builder(into, default)]
        pub malware_scanning_on_upload_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the settings defined for this storage account should override the settings defined for the subscription. Defaults to `false`.
        #[builder(into, default)]
        pub override_subscription_settings_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The Event Grid Topic where every scan result will be sent to. When you set an Event Grid custom topic, you must set `override_subscription_settings_enabled` to `true` to override the subscription-level settings.
        #[builder(into, default)]
        pub scan_results_event_grid_topic_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Sensitive Data Discovery should be enabled. Defaults to `false`.
        #[builder(into, default)]
        pub sensitive_data_discovery_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the storage account the defender applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StorageDefenderResult {
        /// The max GB to be scanned per Month. Must be `-1` or above `0`. Omit this property or set to `-1` if no capping is needed. Defaults to `-1`.
        pub malware_scanning_on_upload_cap_gb_per_month: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Whether On Upload malware scanning should be enabled. Defaults to `false`.
        pub malware_scanning_on_upload_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the settings defined for this storage account should override the settings defined for the subscription. Defaults to `false`.
        pub override_subscription_settings_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The Event Grid Topic where every scan result will be sent to. When you set an Event Grid custom topic, you must set `override_subscription_settings_enabled` to `true` to override the subscription-level settings.
        pub scan_results_event_grid_topic_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Sensitive Data Discovery should be enabled. Defaults to `false`.
        pub sensitive_data_discovery_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the storage account the defender applied to. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StorageDefenderArgs) -> StorageDefenderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let malware_scanning_on_upload_cap_gb_per_month_binding = args
            .malware_scanning_on_upload_cap_gb_per_month
            .get_inner();
        let malware_scanning_on_upload_enabled_binding = args
            .malware_scanning_on_upload_enabled
            .get_inner();
        let override_subscription_settings_enabled_binding = args
            .override_subscription_settings_enabled
            .get_inner();
        let scan_results_event_grid_topic_id_binding = args
            .scan_results_event_grid_topic_id
            .get_inner();
        let sensitive_data_discovery_enabled_binding = args
            .sensitive_data_discovery_enabled
            .get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/storageDefender:StorageDefender".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "malwareScanningOnUploadCapGbPerMonth".into(),
                    value: &malware_scanning_on_upload_cap_gb_per_month_binding,
                },
                register_interface::ObjectField {
                    name: "malwareScanningOnUploadEnabled".into(),
                    value: &malware_scanning_on_upload_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "overrideSubscriptionSettingsEnabled".into(),
                    value: &override_subscription_settings_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "scanResultsEventGridTopicId".into(),
                    value: &scan_results_event_grid_topic_id_binding,
                },
                register_interface::ObjectField {
                    name: "sensitiveDataDiscoveryEnabled".into(),
                    value: &sensitive_data_discovery_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "malwareScanningOnUploadCapGbPerMonth".into(),
                },
                register_interface::ResultField {
                    name: "malwareScanningOnUploadEnabled".into(),
                },
                register_interface::ResultField {
                    name: "overrideSubscriptionSettingsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "scanResultsEventGridTopicId".into(),
                },
                register_interface::ResultField {
                    name: "sensitiveDataDiscoveryEnabled".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StorageDefenderResult {
            malware_scanning_on_upload_cap_gb_per_month: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("malwareScanningOnUploadCapGbPerMonth").unwrap(),
            ),
            malware_scanning_on_upload_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("malwareScanningOnUploadEnabled").unwrap(),
            ),
            override_subscription_settings_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrideSubscriptionSettingsEnabled").unwrap(),
            ),
            scan_results_event_grid_topic_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scanResultsEventGridTopicId").unwrap(),
            ),
            sensitive_data_discovery_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sensitiveDataDiscoveryEnabled").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}