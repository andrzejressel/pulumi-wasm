/// Provides an FIS Experiment Template, which can be used to run an experiment.
/// An experiment template contains one or more actions to run on specified targets during an experiment.
/// It also contains the stop conditions that prevent the experiment from going out of bounds.
/// See [Amazon Fault Injection Simulator](https://docs.aws.amazon.com/fis/index.html)
/// for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = experiment_template::create(
///         "example",
///         ExperimentTemplateArgs::builder()
///             .actions(
///                 vec![
///                     ExperimentTemplateAction::builder()
///                     .actionId("aws:ec2:terminate-instances").name("example-action")
///                     .target(ExperimentTemplateActionTarget::builder().key("Instances")
///                     .value("example-target").build_struct()).build_struct(),
///                 ],
///             )
///             .description("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .stop_conditions(
///                 vec![
///                     ExperimentTemplateStopCondition::builder().source("none")
///                     .build_struct(),
///                 ],
///             )
///             .targets(
///                 vec![
///                     ExperimentTemplateTarget::builder().name("example-target")
///                     .resourceTags(vec![ExperimentTemplateTargetResourceTag::builder()
///                     .key("env").value("example").build_struct(),])
///                     .resourceType("aws:ec2:instance").selectionMode("COUNT(1)")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FIS Experiment Templates using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fis/experimentTemplate:ExperimentTemplate template EXT123AbCdEfGhIjK
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod experiment_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperimentTemplateArgs {
        /// Action to be performed during an experiment. See below.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::fis::ExperimentTemplateAction>,
        >,
        /// Description for the experiment template.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The experiment options for the experiment template. See experiment_options below for more details!
        #[builder(into, default)]
        pub experiment_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fis::ExperimentTemplateExperimentOptions>,
        >,
        /// The configuration for experiment logging. See below.
        #[builder(into, default)]
        pub log_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fis::ExperimentTemplateLogConfiguration>,
        >,
        /// ARN of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When an ongoing experiment should be stopped. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub stop_conditions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::fis::ExperimentTemplateStopCondition>,
        >,
        /// Key-value mapping of tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target of an action. See below.
        #[builder(into, default)]
        pub targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::fis::ExperimentTemplateTarget>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExperimentTemplateResult {
        /// Action to be performed during an experiment. See below.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::fis::ExperimentTemplateAction>,
        >,
        /// Description for the experiment template.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The experiment options for the experiment template. See experiment_options below for more details!
        pub experiment_options: pulumi_gestalt_rust::Output<
            super::super::types::fis::ExperimentTemplateExperimentOptions,
        >,
        /// The configuration for experiment logging. See below.
        pub log_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::fis::ExperimentTemplateLogConfiguration>,
        >,
        /// ARN of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// When an ongoing experiment should be stopped. See below.
        ///
        /// The following arguments are optional:
        pub stop_conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::fis::ExperimentTemplateStopCondition>,
        >,
        /// Key-value mapping of tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target of an action. See below.
        pub targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::fis::ExperimentTemplateTarget>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExperimentTemplateArgs,
    ) -> ExperimentTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let description_binding = args.description.get_output(context);
        let experiment_options_binding = args.experiment_options.get_output(context);
        let log_configuration_binding = args.log_configuration.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let stop_conditions_binding = args.stop_conditions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let targets_binding = args.targets.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fis/experimentTemplate:ExperimentTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "experimentOptions".into(),
                    value: experiment_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfiguration".into(),
                    value: log_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stopConditions".into(),
                    value: stop_conditions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targets".into(),
                    value: targets_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExperimentTemplateResult {
            actions: o.get_field("actions"),
            description: o.get_field("description"),
            experiment_options: o.get_field("experimentOptions"),
            log_configuration: o.get_field("logConfiguration"),
            role_arn: o.get_field("roleArn"),
            stop_conditions: o.get_field("stopConditions"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            targets: o.get_field("targets"),
        }
    }
}
