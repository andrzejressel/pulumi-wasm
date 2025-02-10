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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_queue_properties {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountQueuePropertiesArgs {
        /// A `cors_rule` block as defined above.
        #[builder(into, default)]
        pub cors_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
        >,
        /// A `hour_metrics` block as defined below.
        ///
        /// > **NOTE:** At least one of `cors_rule`, `logging`, `minute_metrics`, or `hour_metrics` must be specified.
        #[builder(into, default)]
        pub hour_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::AccountQueuePropertiesHourMetrics>,
        >,
        /// A `logging` block as defined below.
        #[builder(into, default)]
        pub logging: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::AccountQueuePropertiesLogging>,
        >,
        /// A `minute_metrics` block as defined below.
        #[builder(into, default)]
        pub minute_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::AccountQueuePropertiesMinuteMetrics>,
        >,
        /// The ID of the Storage Account to set Queue Properties on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountQueuePropertiesResult {
        /// A `cors_rule` block as defined above.
        pub cors_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>,
        >,
        /// A `hour_metrics` block as defined below.
        ///
        /// > **NOTE:** At least one of `cors_rule`, `logging`, `minute_metrics`, or `hour_metrics` must be specified.
        pub hour_metrics: pulumi_gestalt_rust::Output<
            super::super::types::storage::AccountQueuePropertiesHourMetrics,
        >,
        /// A `logging` block as defined below.
        pub logging: pulumi_gestalt_rust::Output<
            super::super::types::storage::AccountQueuePropertiesLogging,
        >,
        /// A `minute_metrics` block as defined below.
        pub minute_metrics: pulumi_gestalt_rust::Output<
            super::super::types::storage::AccountQueuePropertiesMinuteMetrics,
        >,
        /// The ID of the Storage Account to set Queue Properties on. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountQueuePropertiesArgs,
    ) -> AccountQueuePropertiesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cors_rules_binding = args.cors_rules.get_output(context);
        let hour_metrics_binding = args.hour_metrics.get_output(context);
        let logging_binding = args.logging.get_output(context);
        let minute_metrics_binding = args.minute_metrics.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/accountQueueProperties:AccountQueueProperties".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "corsRules".into(),
                    value: cors_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hourMetrics".into(),
                    value: hour_metrics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logging".into(),
                    value: logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minuteMetrics".into(),
                    value: minute_metrics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountQueuePropertiesResult {
            cors_rules: o.get_field("corsRules"),
            hour_metrics: o.get_field("hourMetrics"),
            logging: o.get_field("logging"),
            minute_metrics: o.get_field("minuteMetrics"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
