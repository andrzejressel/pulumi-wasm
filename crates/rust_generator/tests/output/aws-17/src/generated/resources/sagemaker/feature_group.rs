/// Provides a SageMaker Feature Group resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod feature_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureGroupArgs {
        /// A free-form description of a Feature Group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the feature that stores the EventTime of a Record in a Feature Group.
        #[builder(into)]
        pub event_time_feature_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of Feature names and types. See Feature Definition Below.
        #[builder(into)]
        pub feature_definitions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::sagemaker::FeatureGroupFeatureDefinition>,
        >,
        /// The name of the Feature Group. The name must be unique within an AWS Region in an AWS account.
        #[builder(into)]
        pub feature_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Offline Feature Store Configuration. See Offline Store Config Below.
        #[builder(into, default)]
        pub offline_store_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::FeatureGroupOfflineStoreConfig>,
        >,
        /// The Online Feature Store Configuration. See Online Store Config Below.
        #[builder(into, default)]
        pub online_store_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfig>,
        >,
        /// The name of the Feature whose value uniquely identifies a Record defined in the Feature Store. Only the latest record per identifier value will be stored in the Online Store.
        #[builder(into)]
        pub record_identifier_feature_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the Offline Store if an `offline_store_config` is provided.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of resource tags for the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub throughput_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::FeatureGroupThroughputConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FeatureGroupResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this feature_group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A free-form description of a Feature Group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the feature that stores the EventTime of a Record in a Feature Group.
        pub event_time_feature_name: pulumi_gestalt_rust::Output<String>,
        /// A list of Feature names and types. See Feature Definition Below.
        pub feature_definitions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sagemaker::FeatureGroupFeatureDefinition>,
        >,
        /// The name of the Feature Group. The name must be unique within an AWS Region in an AWS account.
        pub feature_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Offline Feature Store Configuration. See Offline Store Config Below.
        pub offline_store_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOfflineStoreConfig>,
        >,
        /// The Online Feature Store Configuration. See Online Store Config Below.
        pub online_store_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfig>,
        >,
        /// The name of the Feature whose value uniquely identifies a Record defined in the Feature Store. Only the latest record per identifier value will be stored in the Online Store.
        pub record_identifier_feature_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the Offline Store if an `offline_store_config` is provided.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags for the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub throughput_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::FeatureGroupThroughputConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FeatureGroupArgs,
    ) -> FeatureGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_time_feature_name_binding = args
            .event_time_feature_name
            .get_output(context);
        let feature_definitions_binding = args.feature_definitions.get_output(context);
        let feature_group_name_binding = args.feature_group_name.get_output(context);
        let offline_store_config_binding = args.offline_store_config.get_output(context);
        let online_store_config_binding = args.online_store_config.get_output(context);
        let record_identifier_feature_name_binding = args
            .record_identifier_feature_name
            .get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throughput_config_binding = args.throughput_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/featureGroup:FeatureGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTimeFeatureName".into(),
                    value: &event_time_feature_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureDefinitions".into(),
                    value: &feature_definitions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureGroupName".into(),
                    value: &feature_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offlineStoreConfig".into(),
                    value: &offline_store_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onlineStoreConfig".into(),
                    value: &online_store_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordIdentifierFeatureName".into(),
                    value: &record_identifier_feature_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughputConfig".into(),
                    value: &throughput_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FeatureGroupResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            event_time_feature_name: o.get_field("eventTimeFeatureName"),
            feature_definitions: o.get_field("featureDefinitions"),
            feature_group_name: o.get_field("featureGroupName"),
            offline_store_config: o.get_field("offlineStoreConfig"),
            online_store_config: o.get_field("onlineStoreConfig"),
            record_identifier_feature_name: o.get_field("recordIdentifierFeatureName"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            throughput_config: o.get_field("throughputConfig"),
        }
    }
}
