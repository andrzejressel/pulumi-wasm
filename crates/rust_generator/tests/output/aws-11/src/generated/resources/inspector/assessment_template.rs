/// Provides an Inspector Classic Assessment Template
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentTemplateArgs {
        /// The duration of the inspector run.
        #[builder(into)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A block that enables sending notifications about a specified assessment template event to a designated SNS topic. See Event Subscriptions for details.
        #[builder(into, default)]
        pub event_subscriptions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::inspector::AssessmentTemplateEventSubscription>,
            >,
        >,
        /// The name of the assessment template.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The rules to be used during the run.
        #[builder(into)]
        pub rules_package_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Key-value map of tags for the Inspector assessment template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The assessment target ARN to attach the template to.
        #[builder(into)]
        pub target_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssessmentTemplateResult {
        /// The template assessment ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The duration of the inspector run.
        pub duration: pulumi_gestalt_rust::Output<i32>,
        /// A block that enables sending notifications about a specified assessment template event to a designated SNS topic. See Event Subscriptions for details.
        pub event_subscriptions: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::inspector::AssessmentTemplateEventSubscription>,
            >,
        >,
        /// The name of the assessment template.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The rules to be used during the run.
        pub rules_package_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of tags for the Inspector assessment template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The assessment target ARN to attach the template to.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentTemplateArgs,
    ) -> AssessmentTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let duration_binding = args.duration.get_output(context);
        let event_subscriptions_binding = args.event_subscriptions.get_output(context);
        let name_binding = args.name.get_output(context);
        let rules_package_arns_binding = args.rules_package_arns.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_arn_binding = args.target_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector/assessmentTemplate:AssessmentTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "duration".into(),
                    value: duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventSubscriptions".into(),
                    value: event_subscriptions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulesPackageArns".into(),
                    value: rules_package_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetArn".into(),
                    value: target_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssessmentTemplateResult {
            arn: o.get_field("arn"),
            duration: o.get_field("duration"),
            event_subscriptions: o.get_field("eventSubscriptions"),
            name: o.get_field("name"),
            rules_package_arns: o.get_field("rulesPackageArns"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_arn: o.get_field("targetArn"),
        }
    }
}
