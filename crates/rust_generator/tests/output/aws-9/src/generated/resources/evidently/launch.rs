/// Provides a CloudWatch Evidently Launch resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///             startTime: 2024-01-07 01:43:59+00:00
/// ```
///
/// ### With description
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       description: example description
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///             startTime: 2024-01-07 01:43:59+00:00
/// ```
///
/// ### With multiple groups
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///           description: first-group
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation2
///           variation: Variation2
///           description: second-group
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///               Variation2: 0
///             startTime: 2024-01-07 01:43:59+00:00
/// ```
///
/// ### With metric_monitors
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///       metricMonitors:
///         - metricDefinition:
///             entityIdKey: entity_id_key1
///             eventPattern: '{"Price":[{"numeric":[">",11,"<=",22]}]}'
///             name: name1
///             unitLabel: unit_label1
///             valueKey: value_key1
///         - metricDefinition:
///             entityIdKey: entity_id_key2
///             eventPattern: '{"Price":[{"numeric":[">",9,"<=",19]}]}'
///             name: name2
///             unitLabel: unit_label2
///             valueKey: value_key2
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///             startTime: 2024-01-07 01:43:59+00:00
/// ```
///
/// ### With randomization_salt
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       randomizationSalt: example randomization salt
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///             startTime: 2024-01-07 01:43:59+00:00
/// ```
///
/// ### With multiple steps
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation2
///           variation: Variation2
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 15
///               Variation2: 10
///             startTime: 2024-01-07 01:43:59+00:00
///           - groupWeights:
///               Variation1: 20
///               Variation2: 25
///             startTime: 2024-01-08 01:43:59+00:00
/// ```
///
/// ### With segment overrides
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Launch
///     properties:
///       name: example
///       project: ${exampleAwsEvidentlyProject.name}
///       groups:
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation1
///           variation: Variation1
///         - feature: ${exampleAwsEvidentlyFeature.name}
///           name: Variation2
///           variation: Variation2
///       scheduledSplitsConfig:
///         steps:
///           - groupWeights:
///               Variation1: 0
///               Variation2: 0
///             segmentOverrides:
///               - evaluationOrder: 1
///                 segment: ${exampleAwsEvidentlySegment.name}
///                 weights:
///                   Variation2: 10000
///               - evaluationOrder: 2
///                 segment: ${exampleAwsEvidentlySegment.name}
///                 weights:
///                   Variation1: 40000
///                   Variation2: 30000
///             startTime: 2024-01-08 01:43:59+00:00
/// ```
///
/// ## Import
///
/// Import using the `name` of the launch and `arn` of the project separated by a `:`:
///
/// __Using `pulumi import` to import__ CloudWatch Evidently Launch using the `name` of the launch and `name` of the project or `arn` of the hosting CloudWatch Evidently Project separated by a `:`. For example:
///
/// Import using the `name` of the launch and `name` of the project separated by a `:`:
///
/// ```sh
/// $ pulumi import aws:evidently/launch:Launch example exampleLaunchName:exampleProjectName
/// ```
/// Import using the `name` of the launch and `arn` of the project separated by a `:`:
///
/// ```sh
/// $ pulumi import aws:evidently/launch:Launch example exampleLaunchName:arn:aws:evidently:us-east-1:123456789012:project/exampleProjectName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod launch {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchArgs {
        /// Specifies the description of the launch.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or up to five blocks that contain the feature and variations that are to be used for the launch. Detailed below.
        #[builder(into)]
        pub groups: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::evidently::LaunchGroup>,
        >,
        /// One or up to three blocks that define the metrics that will be used to monitor the launch performance. Detailed below.
        #[builder(into, default)]
        pub metric_monitors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::evidently::LaunchMetricMonitor>>,
        >,
        /// The name for the new launch. Minimum length of `1`. Maximum length of `127`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or ARN of the project that is to contain the new launch.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and randomizationSalt. If you omit randomizationSalt, Evidently uses the launch name as the randomizationSalt.
        #[builder(into, default)]
        pub randomization_salt: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that defines the traffic allocation percentages among the feature variations during each step of the launch. Detailed below.
        #[builder(into, default)]
        pub scheduled_splits_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::evidently::LaunchScheduledSplitsConfig>,
        >,
        /// Tags to apply to the launch. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LaunchResult {
        /// The ARN of the launch.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the launch is created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the launch.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that contains information about the start and end times of the launch. Detailed below
        pub executions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::evidently::LaunchExecution>,
        >,
        /// One or up to five blocks that contain the feature and variations that are to be used for the launch. Detailed below.
        pub groups: pulumi_gestalt_rust::Output<
            Vec<super::super::types::evidently::LaunchGroup>,
        >,
        /// The date and time that the launch was most recently updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// One or up to three blocks that define the metrics that will be used to monitor the launch performance. Detailed below.
        pub metric_monitors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::evidently::LaunchMetricMonitor>>,
        >,
        /// The name for the new launch. Minimum length of `1`. Maximum length of `127`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name or ARN of the project that is to contain the new launch.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and randomizationSalt. If you omit randomizationSalt, Evidently uses the launch name as the randomizationSalt.
        pub randomization_salt: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that defines the traffic allocation percentages among the feature variations during each step of the launch. Detailed below.
        pub scheduled_splits_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::evidently::LaunchScheduledSplitsConfig>,
        >,
        /// The current state of the launch. Valid values are `CREATED`, `UPDATING`, `RUNNING`, `COMPLETED`, and `CANCELLED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// If the launch was stopped, this is the string that was entered by the person who stopped the launch, to explain why it was stopped.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the launch. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of launch.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LaunchArgs,
    ) -> LaunchResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let groups_binding = args.groups.get_output(context).get_inner();
        let metric_monitors_binding = args
            .metric_monitors
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let randomization_salt_binding = args
            .randomization_salt
            .get_output(context)
            .get_inner();
        let scheduled_splits_config_binding = args
            .scheduled_splits_config
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:evidently/launch:Launch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "groups".into(),
                    value: &groups_binding,
                },
                register_interface::ObjectField {
                    name: "metricMonitors".into(),
                    value: &metric_monitors_binding,
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
                    name: "randomizationSalt".into(),
                    value: &randomization_salt_binding,
                },
                register_interface::ObjectField {
                    name: "scheduledSplitsConfig".into(),
                    value: &scheduled_splits_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LaunchResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            executions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executions"),
            ),
            groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groups"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            metric_monitors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricMonitors"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            randomization_salt: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("randomizationSalt"),
            ),
            scheduled_splits_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scheduledSplitsConfig"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
