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
pub mod launch {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchArgs {
        /// Specifies the description of the launch.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// One or up to five blocks that contain the feature and variations that are to be used for the launch. Detailed below.
        #[builder(into)]
        pub groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::evidently::LaunchGroup>,
        >,
        /// One or up to three blocks that define the metrics that will be used to monitor the launch performance. Detailed below.
        #[builder(into, default)]
        pub metric_monitors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::evidently::LaunchMetricMonitor>>,
        >,
        /// The name for the new launch. Minimum length of `1`. Maximum length of `127`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name or ARN of the project that is to contain the new launch.
        #[builder(into)]
        pub project: pulumi_wasm_rust::Output<String>,
        /// When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and randomizationSalt. If you omit randomizationSalt, Evidently uses the launch name as the randomizationSalt.
        #[builder(into, default)]
        pub randomization_salt: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that defines the traffic allocation percentages among the feature variations during each step of the launch. Detailed below.
        #[builder(into, default)]
        pub scheduled_splits_config: pulumi_wasm_rust::Output<
            Option<super::super::types::evidently::LaunchScheduledSplitsConfig>,
        >,
        /// Tags to apply to the launch. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LaunchResult {
        /// The ARN of the launch.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that the launch is created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the launch.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that contains information about the start and end times of the launch. Detailed below
        pub executions: pulumi_wasm_rust::Output<
            Vec<super::super::types::evidently::LaunchExecution>,
        >,
        /// One or up to five blocks that contain the feature and variations that are to be used for the launch. Detailed below.
        pub groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::evidently::LaunchGroup>,
        >,
        /// The date and time that the launch was most recently updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// One or up to three blocks that define the metrics that will be used to monitor the launch performance. Detailed below.
        pub metric_monitors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::evidently::LaunchMetricMonitor>>,
        >,
        /// The name for the new launch. Minimum length of `1`. Maximum length of `127`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name or ARN of the project that is to contain the new launch.
        pub project: pulumi_wasm_rust::Output<String>,
        /// When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and randomizationSalt. If you omit randomizationSalt, Evidently uses the launch name as the randomizationSalt.
        pub randomization_salt: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that defines the traffic allocation percentages among the feature variations during each step of the launch. Detailed below.
        pub scheduled_splits_config: pulumi_wasm_rust::Output<
            Option<super::super::types::evidently::LaunchScheduledSplitsConfig>,
        >,
        /// The current state of the launch. Valid values are `CREATED`, `UPDATING`, `RUNNING`, `COMPLETED`, and `CANCELLED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// If the launch was stopped, this is the string that was entered by the person who stopped the launch, to explain why it was stopped.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the launch. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of launch.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LaunchArgs) -> LaunchResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let groups_binding = args.groups.get_inner();
        let metric_monitors_binding = args.metric_monitors.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let randomization_salt_binding = args.randomization_salt.get_inner();
        let scheduled_splits_config_binding = args.scheduled_splits_config.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:evidently/launch:Launch".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "executions".into(),
                },
                register_interface::ResultField {
                    name: "groups".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "metricMonitors".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "randomizationSalt".into(),
                },
                register_interface::ResultField {
                    name: "scheduledSplitsConfig".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LaunchResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            executions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executions").unwrap(),
            ),
            groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groups").unwrap(),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            metric_monitors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricMonitors").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            randomization_salt: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("randomizationSalt").unwrap(),
            ),
            scheduled_splits_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledSplitsConfig").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}