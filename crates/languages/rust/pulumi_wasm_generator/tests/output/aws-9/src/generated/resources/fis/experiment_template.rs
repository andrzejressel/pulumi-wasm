/// Provides an FIS Experiment Template, which can be used to run an experiment.
/// An experiment template contains one or more actions to run on specified targets during an experiment.
/// It also contains the stop conditions that prevent the experiment from going out of bounds.
/// See [Amazon Fault Injection Simulator](https://docs.aws.amazon.com/fis/index.html)
/// for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod experiment_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperimentTemplateArgs {
        /// Action to be performed during an experiment. See below.
        #[builder(into)]
        pub actions: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::fis::ExperimentTemplateAction>,
        >,
        /// Description for the experiment template.
        #[builder(into)]
        pub description: pulumi_wasm_rust::InputOrOutput<String>,
        /// The experiment options for the experiment template. See experiment_options below for more details!
        #[builder(into, default)]
        pub experiment_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fis::ExperimentTemplateExperimentOptions>,
        >,
        /// The configuration for experiment logging. See below.
        #[builder(into, default)]
        pub log_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fis::ExperimentTemplateLogConfiguration>,
        >,
        /// ARN of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// When an ongoing experiment should be stopped. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub stop_conditions: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::fis::ExperimentTemplateStopCondition>,
        >,
        /// Key-value mapping of tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target of an action. See below.
        #[builder(into, default)]
        pub targets: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::fis::ExperimentTemplateTarget>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExperimentTemplateResult {
        /// Action to be performed during an experiment. See below.
        pub actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::fis::ExperimentTemplateAction>,
        >,
        /// Description for the experiment template.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The experiment options for the experiment template. See experiment_options below for more details!
        pub experiment_options: pulumi_wasm_rust::Output<
            super::super::types::fis::ExperimentTemplateExperimentOptions,
        >,
        /// The configuration for experiment logging. See below.
        pub log_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fis::ExperimentTemplateLogConfiguration>,
        >,
        /// ARN of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// When an ongoing experiment should be stopped. See below.
        ///
        /// The following arguments are optional:
        pub stop_conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::fis::ExperimentTemplateStopCondition>,
        >,
        /// Key-value mapping of tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target of an action. See below.
        pub targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::fis::ExperimentTemplateTarget>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExperimentTemplateArgs,
    ) -> ExperimentTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let experiment_options_binding = args
            .experiment_options
            .get_output(context)
            .get_inner();
        let log_configuration_binding = args
            .log_configuration
            .get_output(context)
            .get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let stop_conditions_binding = args
            .stop_conditions
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let targets_binding = args.targets.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fis/experimentTemplate:ExperimentTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "experimentOptions".into(),
                    value: &experiment_options_binding,
                },
                register_interface::ObjectField {
                    name: "logConfiguration".into(),
                    value: &log_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "stopConditions".into(),
                    value: &stop_conditions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targets".into(),
                    value: &targets_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExperimentTemplateResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            experiment_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("experimentOptions"),
            ),
            log_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logConfiguration"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            stop_conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stopConditions"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            targets: pulumi_wasm_rust::__private::into_domain(o.extract_field("targets")),
        }
    }
}
