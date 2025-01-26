/// Provides a CloudWatch Evidently Project resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       tags:
///         Key1: example Project
/// ```
///
/// ### Store evaluation events in a CloudWatch Log Group
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       dataDelivery:
///         cloudwatchLogs:
///           logGroup: example-log-group-name
///       tags:
///         Key1: example Project
/// ```
///
/// ### Store evaluation events in an S3 bucket
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       dataDelivery:
///         s3Destination:
///           bucket: example-bucket-name
///           prefix: example
///       tags:
///         Key1: example Project
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Evidently Project using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:evidently/project:Project example arn:aws:evidently:us-east-1:123456789012:segment/example
/// ```
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// A block that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view. See below.
        #[builder(into, default)]
        pub data_delivery: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::evidently::ProjectDataDelivery>,
        >,
        /// Specifies the description of the project.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A name for the project.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the project. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The number of ongoing experiments currently in the project.
        pub active_experiment_count: pulumi_wasm_rust::Output<i32>,
        /// The number of ongoing launches currently in the project.
        pub active_launch_count: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the project.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that the project is created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// A block that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view. See below.
        pub data_delivery: pulumi_wasm_rust::Output<
            Option<super::super::types::evidently::ProjectDataDelivery>,
        >,
        /// Specifies the description of the project.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of experiments currently in the project. This includes all experiments that have been created and not deleted, whether they are ongoing or not.
        pub experiment_count: pulumi_wasm_rust::Output<i32>,
        /// The number of features currently in the project.
        pub feature_count: pulumi_wasm_rust::Output<i32>,
        /// The date and time that the project was most recently updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// The number of launches currently in the project. This includes all launches that have been created and not deleted, whether they are ongoing or not.
        pub launch_count: pulumi_wasm_rust::Output<i32>,
        /// A name for the project.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The current state of the project. Valid values are `AVAILABLE` and `UPDATING`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the project. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_delivery_binding = args.data_delivery.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:evidently/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataDelivery".into(),
                    value: &data_delivery_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            active_experiment_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeExperimentCount"),
            ),
            active_launch_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeLaunchCount"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            data_delivery: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataDelivery"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            experiment_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("experimentCount"),
            ),
            feature_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("featureCount"),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            launch_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchCount"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
