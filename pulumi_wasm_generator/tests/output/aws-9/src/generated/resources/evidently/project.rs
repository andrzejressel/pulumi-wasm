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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// A block that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view. See below.
        #[builder(into, default)]
        pub data_delivery: pulumi_wasm_rust::Output<
            Option<super::super::types::evidently::ProjectDataDelivery>,
        >,
        /// Specifies the description of the project.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A name for the project.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the project. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: ProjectArgs) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_delivery_binding = args.data_delivery.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:evidently/project:Project".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeExperimentCount".into(),
                },
                register_interface::ResultField {
                    name: "activeLaunchCount".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "dataDelivery".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "experimentCount".into(),
                },
                register_interface::ResultField {
                    name: "featureCount".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "launchCount".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectResult {
            active_experiment_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeExperimentCount").unwrap(),
            ),
            active_launch_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeLaunchCount").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            data_delivery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDelivery").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            experiment_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("experimentCount").unwrap(),
            ),
            feature_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureCount").unwrap(),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            launch_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchCount").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
