/// Manages the Queue Properties of an Azure Storage Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///       tags:
///         environment: staging
///   exampleAccountQueueProperties:
///     type: azure:storage:AccountQueueProperties
///     name: example
///     properties:
///       storageAccountId: ${exampleAccount.id}
///       corsRules:
///         - allowedOrigins:
///             - http://www.example.com
///           exposedHeaders:
///             - x-tempo-*
///           allowedHeaders:
///             - x-tempo-*
///           allowedMethods:
///             - GET
///             - PUT
///           maxAgeInSeconds: '500'
///       logging:
///         version: '1.0'
///         delete: true
///         read: true
///         write: true
///         retentionPolicyDays: 7
///       hourMetrics:
///         version: '1.0'
///         retentionPolicyDays: 7
///       minuteMetrics:
///         version: '1.0'
///         retentionPolicyDays: 7
/// ```
///
/// ## Import
///
/// Storage Account Queue Properties can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/accountQueueProperties:AccountQueueProperties queueprops /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount
/// ```
///
pub mod account_queue_properties {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountQueuePropertiesArgs {
        /// A `cors_rule` block as defined above.
        #[builder(into, default)]
        pub cors_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
        >,
        /// A `hour_metrics` block as defined below.
        ///
        /// > **NOTE:** At least one of `cors_rule`, `logging`, `minute_metrics`, or `hour_metrics` must be specified.
        #[builder(into, default)]
        pub hour_metrics: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountQueuePropertiesHourMetrics>,
        >,
        /// A `logging` block as defined below.
        #[builder(into, default)]
        pub logging: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountQueuePropertiesLogging>,
        >,
        /// A `minute_metrics` block as defined below.
        #[builder(into, default)]
        pub minute_metrics: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountQueuePropertiesMinuteMetrics>,
        >,
        /// The ID of the Storage Account to set Queue Properties on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccountQueuePropertiesResult {
        /// A `cors_rule` block as defined above.
        pub cors_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
        >,
        /// A `hour_metrics` block as defined below.
        ///
        /// > **NOTE:** At least one of `cors_rule`, `logging`, `minute_metrics`, or `hour_metrics` must be specified.
        pub hour_metrics: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountQueuePropertiesHourMetrics,
        >,
        /// A `logging` block as defined below.
        pub logging: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountQueuePropertiesLogging,
        >,
        /// A `minute_metrics` block as defined below.
        pub minute_metrics: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountQueuePropertiesMinuteMetrics,
        >,
        /// The ID of the Storage Account to set Queue Properties on. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccountQueuePropertiesArgs,
    ) -> AccountQueuePropertiesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cors_rules_binding = args.cors_rules.get_inner();
        let hour_metrics_binding = args.hour_metrics.get_inner();
        let logging_binding = args.logging.get_inner();
        let minute_metrics_binding = args.minute_metrics.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/accountQueueProperties:AccountQueueProperties".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "corsRules".into(),
                    value: &cors_rules_binding,
                },
                register_interface::ObjectField {
                    name: "hourMetrics".into(),
                    value: &hour_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "logging".into(),
                    value: &logging_binding,
                },
                register_interface::ObjectField {
                    name: "minuteMetrics".into(),
                    value: &minute_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "corsRules".into(),
                },
                register_interface::ResultField {
                    name: "hourMetrics".into(),
                },
                register_interface::ResultField {
                    name: "logging".into(),
                },
                register_interface::ResultField {
                    name: "minuteMetrics".into(),
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
        AccountQueuePropertiesResult {
            cors_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsRules").unwrap(),
            ),
            hour_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hourMetrics").unwrap(),
            ),
            logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logging").unwrap(),
            ),
            minute_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minuteMetrics").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}
