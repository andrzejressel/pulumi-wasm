/// Provides an AppConfig Environment resource for an `aws.appconfig.Application` resource. One or more environments can be defined for an application.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:Environment
///     properties:
///       name: example-environment-tf
///       description: Example AppConfig Environment
///       applicationId: ${exampleApplication.id}
///       monitors:
///         - alarmArn: ${exampleAwsCloudwatchMetricAlarm.arn}
///           alarmRoleArn: ${exampleAwsIamRole.arn}
///       tags:
///         Type: AppConfig Environment
///   exampleApplication:
///     type: aws:appconfig:Application
///     name: example
///     properties:
///       name: example-application-tf
///       description: Example AppConfig Application
///       tags:
///         Type: AppConfig Application
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Environments using the environment ID and application ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/environment:Environment example 71abcde:11xxxxx
/// ```
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// AppConfig application ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Description of the environment. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
        #[builder(into, default)]
        pub monitors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfig::EnvironmentMonitor>>,
        >,
        /// Name for the environment. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// AppConfig application ID. Must be between 4 and 7 characters in length.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the AppConfig Environment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the environment. Can be at most 1024 characters.
        pub description: pulumi_wasm_rust::Output<String>,
        /// AppConfig environment ID.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
        pub monitors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfig::EnvironmentMonitor>>,
        >,
        /// Name for the environment. Must be between 1 and 64 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// State of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
        /// or `ROLLED_BACK`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let description_binding = args.description.get_inner();
        let monitors_binding = args.monitors.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "monitors".into(),
                    value: &monitors_binding,
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
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "monitors".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        EnvironmentResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            monitors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitors").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
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
