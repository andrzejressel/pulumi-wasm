/// Provides a CloudWatch Evidently Feature resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Feature
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       description: example description
///       variations:
///         - name: Variation1
///           value:
///             stringValue: example
///       tags:
///         Key1: example Feature
/// ```
///
/// ### With default variation
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = feature::create(
///         "example",
///         FeatureArgs::builder()
///             .default_variation("Variation2")
///             .name("example")
///             .project("${exampleAwsEvidentlyProject.name}")
///             .variations(
///                 vec![
///                     FeatureVariation::builder().name("Variation1")
///                     .value(FeatureVariationValue::builder().stringValue("exampleval1")
///                     .build_struct()).build_struct(), FeatureVariation::builder()
///                     .name("Variation2").value(FeatureVariationValue::builder()
///                     .stringValue("exampleval2").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With entity overrides
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Feature
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       entityOverrides:
///         test1: Variation1
///       variations:
///         - name: Variation1
///           value:
///             stringValue: exampleval1
///         - name: Variation2
///           value:
///             stringValue: exampleval2
/// ```
///
/// ### With evaluation strategy
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Feature
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       evaluationStrategy: ALL_RULES
///       entityOverrides:
///         test1: Variation1
///       variations:
///         - name: Variation1
///           value:
///             stringValue: exampleval1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Evidently Feature using the feature `name` and `name` or `arn` of the hosting CloudWatch Evidently Project separated by a `:`. For example:
///
/// ```sh
/// $ pulumi import aws:evidently/feature:Feature example exampleFeatureName:arn:aws:evidently:us-east-1:123456789012:project/example
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod feature {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureArgs {
        /// The name of the variation to use as the default variation. The default variation is served to users who are not allocated to any ongoing launches or experiments of this feature. This variation must also be listed in the `variations` structure. If you omit `default_variation`, the first variation listed in the `variations` structure is used as the default variation.
        #[builder(into, default)]
        pub default_variation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the description of the feature.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify users that should always be served a specific variation of a feature. Each user is specified by a key-value pair . For each key, specify a user by entering their user ID, account ID, or some other identifier. For the value, specify the name of the variation that they are to be served.
        #[builder(into, default)]
        pub entity_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify `ALL_RULES` to activate the traffic allocation specified by any ongoing launches or experiments. Specify `DEFAULT_VARIATION` to serve the default variation to all users instead.
        #[builder(into, default)]
        pub evaluation_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the new feature. Minimum length of `1`. Maximum length of `127`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or ARN of the project that is to contain the new feature.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags to apply to the feature. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more blocks that contain the configuration of the feature's different variations. Detailed below
        #[builder(into)]
        pub variations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::evidently::FeatureVariation>,
        >,
    }
    #[allow(dead_code)]
    pub struct FeatureResult {
        /// The ARN of the feature.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the feature is created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the variation to use as the default variation. The default variation is served to users who are not allocated to any ongoing launches or experiments of this feature. This variation must also be listed in the `variations` structure. If you omit `default_variation`, the first variation listed in the `variations` structure is used as the default variation.
        pub default_variation: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the feature.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify users that should always be served a specific variation of a feature. Each user is specified by a key-value pair . For each key, specify a user by entering their user ID, account ID, or some other identifier. For the value, specify the name of the variation that they are to be served.
        pub entity_overrides: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more blocks that define the evaluation rules for the feature. Detailed below
        pub evaluation_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::evidently::FeatureEvaluationRule>,
        >,
        /// Specify `ALL_RULES` to activate the traffic allocation specified by any ongoing launches or experiments. Specify `DEFAULT_VARIATION` to serve the default variation to all users instead.
        pub evaluation_strategy: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the feature was most recently updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// The name for the new feature. Minimum length of `1`. Maximum length of `127`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name or ARN of the project that is to contain the new feature.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The current state of the feature. Valid values are `AVAILABLE` and `UPDATING`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the feature. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines the type of value used to define the different feature variations. Valid Values: `STRING`, `LONG`, `DOUBLE`, `BOOLEAN`.
        pub value_type: pulumi_gestalt_rust::Output<String>,
        /// One or more blocks that contain the configuration of the feature's different variations. Detailed below
        pub variations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::evidently::FeatureVariation>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FeatureArgs,
    ) -> FeatureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_variation_binding = args
            .default_variation
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let entity_overrides_binding = args
            .entity_overrides
            .get_output(context)
            .get_inner();
        let evaluation_strategy_binding = args
            .evaluation_strategy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let variations_binding = args.variations.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:evidently/feature:Feature".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultVariation".into(),
                    value: &default_variation_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "entityOverrides".into(),
                    value: &entity_overrides_binding,
                },
                register_interface::ObjectField {
                    name: "evaluationStrategy".into(),
                    value: &evaluation_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "variations".into(),
                    value: &variations_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FeatureResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            default_variation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultVariation"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            entity_overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entityOverrides"),
            ),
            evaluation_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("evaluationRules"),
            ),
            evaluation_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("evaluationStrategy"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            value_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("valueType"),
            ),
            variations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("variations"),
            ),
        }
    }
}
