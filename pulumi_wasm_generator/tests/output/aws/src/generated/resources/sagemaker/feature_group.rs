/// Provides a SageMaker Feature Group resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = feature_group::create(
///         "example",
///         FeatureGroupArgs::builder()
///             .event_time_feature_name("example")
///             .feature_definitions(
///                 vec![
///                     FeatureGroupFeatureDefinition::builder().featureName("example")
///                     .featureType("String").build_struct(),
///                 ],
///             )
///             .feature_group_name("example")
///             .online_store_config(
///                 FeatureGroupOnlineStoreConfig::builder()
///                     .enableOnlineStore(true)
///                     .build_struct(),
///             )
///             .record_identifier_feature_name("example")
///             .role_arn("${test.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Feature Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/featureGroup:FeatureGroup test_feature_group feature_group-foo
/// ```
pub mod feature_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureGroupArgs {
        /// A free-form description of a Feature Group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the feature that stores the EventTime of a Record in a Feature Group.
        #[builder(into)]
        pub event_time_feature_name: pulumi_wasm_rust::Output<String>,
        /// A list of Feature names and types. See Feature Definition Below.
        #[builder(into)]
        pub feature_definitions: pulumi_wasm_rust::Output<
            Vec<super::super::types::sagemaker::FeatureGroupFeatureDefinition>,
        >,
        /// The name of the Feature Group. The name must be unique within an AWS Region in an AWS account.
        #[builder(into)]
        pub feature_group_name: pulumi_wasm_rust::Output<String>,
        /// The Offline Feature Store Configuration. See Offline Store Config Below.
        #[builder(into, default)]
        pub offline_store_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOfflineStoreConfig>,
        >,
        /// The Online Feature Store Configuration. See Online Store Config Below.
        #[builder(into, default)]
        pub online_store_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfig>,
        >,
        /// The name of the Feature whose value uniquely identifies a Record defined in the Feature Store. Only the latest record per identifier value will be stored in the Online Store.
        #[builder(into)]
        pub record_identifier_feature_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the Offline Store if an `offline_store_config` is provided.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags for the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub throughput_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupThroughputConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FeatureGroupResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this feature_group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A free-form description of a Feature Group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the feature that stores the EventTime of a Record in a Feature Group.
        pub event_time_feature_name: pulumi_wasm_rust::Output<String>,
        /// A list of Feature names and types. See Feature Definition Below.
        pub feature_definitions: pulumi_wasm_rust::Output<
            Vec<super::super::types::sagemaker::FeatureGroupFeatureDefinition>,
        >,
        /// The name of the Feature Group. The name must be unique within an AWS Region in an AWS account.
        pub feature_group_name: pulumi_wasm_rust::Output<String>,
        /// The Offline Feature Store Configuration. See Offline Store Config Below.
        pub offline_store_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOfflineStoreConfig>,
        >,
        /// The Online Feature Store Configuration. See Online Store Config Below.
        pub online_store_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfig>,
        >,
        /// The name of the Feature whose value uniquely identifies a Record defined in the Feature Store. Only the latest record per identifier value will be stored in the Online Store.
        pub record_identifier_feature_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the Offline Store if an `offline_store_config` is provided.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags for the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub throughput_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::FeatureGroupThroughputConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FeatureGroupArgs) -> FeatureGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let event_time_feature_name_binding = args.event_time_feature_name.get_inner();
        let feature_definitions_binding = args.feature_definitions.get_inner();
        let feature_group_name_binding = args.feature_group_name.get_inner();
        let offline_store_config_binding = args.offline_store_config.get_inner();
        let online_store_config_binding = args.online_store_config.get_inner();
        let record_identifier_feature_name_binding = args
            .record_identifier_feature_name
            .get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let throughput_config_binding = args.throughput_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/featureGroup:FeatureGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventTimeFeatureName".into(),
                    value: &event_time_feature_name_binding,
                },
                register_interface::ObjectField {
                    name: "featureDefinitions".into(),
                    value: &feature_definitions_binding,
                },
                register_interface::ObjectField {
                    name: "featureGroupName".into(),
                    value: &feature_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "offlineStoreConfig".into(),
                    value: &offline_store_config_binding,
                },
                register_interface::ObjectField {
                    name: "onlineStoreConfig".into(),
                    value: &online_store_config_binding,
                },
                register_interface::ObjectField {
                    name: "recordIdentifierFeatureName".into(),
                    value: &record_identifier_feature_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "throughputConfig".into(),
                    value: &throughput_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventTimeFeatureName".into(),
                },
                register_interface::ResultField {
                    name: "featureDefinitions".into(),
                },
                register_interface::ResultField {
                    name: "featureGroupName".into(),
                },
                register_interface::ResultField {
                    name: "offlineStoreConfig".into(),
                },
                register_interface::ResultField {
                    name: "onlineStoreConfig".into(),
                },
                register_interface::ResultField {
                    name: "recordIdentifierFeatureName".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "throughputConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FeatureGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            event_time_feature_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventTimeFeatureName").unwrap(),
            ),
            feature_definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureDefinitions").unwrap(),
            ),
            feature_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureGroupName").unwrap(),
            ),
            offline_store_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offlineStoreConfig").unwrap(),
            ),
            online_store_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onlineStoreConfig").unwrap(),
            ),
            record_identifier_feature_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recordIdentifierFeatureName").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            throughput_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughputConfig").unwrap(),
            ),
        }
    }
}