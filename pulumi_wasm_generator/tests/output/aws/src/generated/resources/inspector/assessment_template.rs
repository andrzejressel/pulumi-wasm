/// Provides an Inspector Classic Assessment Template
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = assessment_template::create(
///         "example",
///         AssessmentTemplateArgs::builder()
///             .duration(3600)
///             .event_subscriptions(
///                 vec![
///                     AssessmentTemplateEventSubscription::builder()
///                     .event("ASSESSMENT_RUN_COMPLETED")
///                     .topicArn("${exampleAwsSnsTopic.arn}").build_struct(),
///                 ],
///             )
///             .name("example")
///             .rules_package_arns(
///                 vec![
///                     "arn:aws:inspector:us-west-2:758058086616:rulespackage/0-9hgA516p",
///                     "arn:aws:inspector:us-west-2:758058086616:rulespackage/0-H5hpSawc",
///                     "arn:aws:inspector:us-west-2:758058086616:rulespackage/0-JJOtZiqQ",
///                     "arn:aws:inspector:us-west-2:758058086616:rulespackage/0-vg5GGHSD",
///                 ],
///             )
///             .target_arn("${exampleAwsInspectorAssessmentTarget.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_inspector_assessment_template` using the template assessment ARN. For example:
///
/// ```sh
/// $ pulumi import aws:inspector/assessmentTemplate:AssessmentTemplate example arn:aws:inspector:us-west-2:123456789012:target/0-9IaAzhGR/template/0-WEcjR8CH
/// ```
pub mod assessment_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentTemplateArgs {
        /// The duration of the inspector run.
        #[builder(into)]
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// A block that enables sending notifications about a specified assessment template event to a designated SNS topic. See Event Subscriptions for details.
        #[builder(into, default)]
        pub event_subscriptions: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::inspector::AssessmentTemplateEventSubscription>,
            >,
        >,
        /// The name of the assessment template.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The rules to be used during the run.
        #[builder(into)]
        pub rules_package_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of tags for the Inspector assessment template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The assessment target ARN to attach the template to.
        #[builder(into)]
        pub target_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AssessmentTemplateResult {
        /// The template assessment ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The duration of the inspector run.
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// A block that enables sending notifications about a specified assessment template event to a designated SNS topic. See Event Subscriptions for details.
        pub event_subscriptions: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::inspector::AssessmentTemplateEventSubscription>,
            >,
        >,
        /// The name of the assessment template.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The rules to be used during the run.
        pub rules_package_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of tags for the Inspector assessment template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The assessment target ARN to attach the template to.
        pub target_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssessmentTemplateArgs) -> AssessmentTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let duration_binding = args.duration.get_inner();
        let event_subscriptions_binding = args.event_subscriptions.get_inner();
        let name_binding = args.name.get_inner();
        let rules_package_arns_binding = args.rules_package_arns.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_arn_binding = args.target_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector/assessmentTemplate:AssessmentTemplate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "eventSubscriptions".into(),
                    value: &event_subscriptions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rulesPackageArns".into(),
                    value: &rules_package_arns_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "duration".into(),
                },
                register_interface::ResultField {
                    name: "eventSubscriptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rulesPackageArns".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssessmentTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duration").unwrap(),
            ),
            event_subscriptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSubscriptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rules_package_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulesPackageArns").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetArn").unwrap(),
            ),
        }
    }
}
