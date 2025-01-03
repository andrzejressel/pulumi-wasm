/// Provides a CloudTrail Event Data Store.
///
/// More information about event data stores can be found in the [Event Data Store User Guide](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-event-data-store.html).
///
/// > **Tip:** For an organization event data store you must create this resource in the management account.
///
/// ## Example Usage
///
/// ### Basic
///
/// The most simple event data store configuration requires us to only set the `name` attribute. The event data store will automatically capture all management events. To capture management events from all the regions, `multi_region_enabled` must be `true`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_data_store::create(
///         "example",
///         EventDataStoreArgs::builder().name("example-event-data-store").build_struct(),
///     );
/// }
/// ```
///
/// ### Data Event Logging
///
/// CloudTrail can log [Data Events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html) for certain services such as S3 bucket objects and Lambda function invocations. Additional information about data event configuration can be found in the following links:
///
/// - [CloudTrail API AdvancedFieldSelector documentation](https://docs.aws.amazon.com/awscloudtrail/latest/APIReference/API_AdvancedFieldSelector.html)
///
/// ### Log all DynamoDB PutEvent actions for a specific DynamoDB table
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudtrail:EventDataStore
///     properties:
///       advancedEventSelectors:
///         - name: Log all DynamoDB PutEvent actions for a specific DynamoDB table
///           fieldSelectors:
///             - field: eventCategory
///               equals:
///                 - Data
///             - field: resources.type
///               equals:
///                 - AWS::DynamoDB::Table
///             - field: eventName
///               equals:
///                 - PutItem
///             - field: resources.ARN
///               equals:
///                 - ${table.arn}
/// variables:
///   table:
///     fn::invoke:
///       function: aws:dynamodb:getTable
///       arguments:
///         name: not-important-dynamodb-table
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import event data stores using their `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudtrail/eventDataStore:EventDataStore example arn:aws:cloudtrail:us-east-1:123456789123:eventdatastore/22333815-4414-412c-b155-dd254033gfhf
/// ```
pub mod event_data_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventDataStoreArgs {
        /// The advanced event selectors to use to select the events for the data store. For more information about how to use advanced event selectors, see [Log events by using advanced event selectors](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced) in the CloudTrail User Guide.
        #[builder(into, default)]
        pub advanced_event_selectors: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cloudtrail::EventDataStoreAdvancedEventSelector>,
            >,
        >,
        /// The billing mode for the event data store. The valid values are `EXTENDABLE_RETENTION_PRICING` and `FIXED_RETENTION_PRICING`. Defaults to `EXTENDABLE_RETENTION_PRICING`.
        #[builder(into, default)]
        pub billing_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the AWS KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by alias/, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created. Default: `true`.
        #[builder(into, default)]
        pub multi_region_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the event data store.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether an event data store collects events logged for an organization in AWS Organizations. Default: `false`.
        #[builder(into, default)]
        pub organization_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The retention period of the event data store, in days. You can set a retention period of up to 2555 days, the equivalent of seven years. Default: `2555`.
        #[builder(into, default)]
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled. Default: `true`.
        #[builder(into, default)]
        pub termination_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EventDataStoreResult {
        /// The advanced event selectors to use to select the events for the data store. For more information about how to use advanced event selectors, see [Log events by using advanced event selectors](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced) in the CloudTrail User Guide.
        pub advanced_event_selectors: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudtrail::EventDataStoreAdvancedEventSelector>,
        >,
        /// ARN of the event data store.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The billing mode for the event data store. The valid values are `EXTENDABLE_RETENTION_PRICING` and `FIXED_RETENTION_PRICING`. Defaults to `EXTENDABLE_RETENTION_PRICING`.
        pub billing_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the AWS KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by alias/, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the event data store includes events from all regions, or only from the region in which the event data store is created. Default: `true`.
        pub multi_region_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the event data store.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether an event data store collects events logged for an organization in AWS Organizations. Default: `false`.
        pub organization_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The retention period of the event data store, in days. You can set a retention period of up to 2555 days, the equivalent of seven years. Default: `2555`.
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether termination protection is enabled for the event data store. If termination protection is enabled, you cannot delete the event data store until termination protection is disabled. Default: `true`.
        pub termination_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventDataStoreArgs) -> EventDataStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_event_selectors_binding = args.advanced_event_selectors.get_inner();
        let billing_mode_binding = args.billing_mode.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let multi_region_enabled_binding = args.multi_region_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let organization_enabled_binding = args.organization_enabled.get_inner();
        let retention_period_binding = args.retention_period.get_inner();
        let tags_binding = args.tags.get_inner();
        let termination_protection_enabled_binding = args
            .termination_protection_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudtrail/eventDataStore:EventDataStore".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedEventSelectors".into(),
                    value: &advanced_event_selectors_binding,
                },
                register_interface::ObjectField {
                    name: "billingMode".into(),
                    value: &billing_mode_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "multiRegionEnabled".into(),
                    value: &multi_region_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "organizationEnabled".into(),
                    value: &organization_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminationProtectionEnabled".into(),
                    value: &termination_protection_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "advancedEventSelectors".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "billingMode".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "multiRegionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organizationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "terminationProtectionEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventDataStoreResult {
            advanced_event_selectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedEventSelectors").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            billing_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingMode").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            multi_region_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegionEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationEnabled").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            termination_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationProtectionEnabled").unwrap(),
            ),
        }
    }
}
