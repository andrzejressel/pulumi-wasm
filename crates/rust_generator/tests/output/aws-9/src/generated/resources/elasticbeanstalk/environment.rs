/// Provides an Elastic Beanstalk Environment Resource. Elastic Beanstalk allows
/// you to deploy and manage applications in the AWS cloud without worrying about
/// the infrastructure that runs those applications.
///
/// Environments are often things such as `development`, `integration`, or
/// `production`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tfenvtest = environment::create(
///         "tfenvtest",
///         EnvironmentArgs::builder()
///             .application("${tftest.name}")
///             .name("tf-test-name")
///             .solution_stack_name("64bit Amazon Linux 2015.03 v2.0.3 running Go 1.4")
///             .build_struct(),
///     );
///     let tftest = application::create(
///         "tftest",
///         ApplicationArgs::builder()
///             .description("tf-test-desc")
///             .name("tf-test-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Option Settings
///
/// Some options can be stack-specific, check [AWS Docs](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options-general.html)
/// for supported options and examples.
///
/// The `setting` and `all_settings` mappings support the following format:
///
/// * `namespace` - unique namespace identifying the option's associated AWS resource
/// * `name` - name of the configuration option
/// * `value` - value for the configuration option
/// * `resource` - (Optional) resource name for [scheduled action](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options-general.html#command-options-general-autoscalingscheduledaction)
///
/// ### Example With Options
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tfenvtest = environment::create(
///         "tfenvtest",
///         EnvironmentArgs::builder()
///             .application("${tftest.name}")
///             .name("tf-test-name")
///             .settings(
///                 vec![
///                     EnvironmentSetting::builder().name("VPCId").namespace("aws:ec2:vpc")
///                     .value("vpc-xxxxxxxx").build_struct(), EnvironmentSetting::builder()
///                     .name("Subnets").namespace("aws:ec2:vpc").value("subnet-xxxxxxxx")
///                     .build_struct(),
///                 ],
///             )
///             .solution_stack_name("64bit Amazon Linux 2015.03 v2.0.3 running Go 1.4")
///             .build_struct(),
///     );
///     let tftest = application::create(
///         "tftest",
///         ApplicationArgs::builder()
///             .description("tf-test-desc")
///             .name("tf-test-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Beanstalk Environments using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticbeanstalk/environment:Environment prodenv e-rpqsewtp2j
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Name of the application that contains the version
        /// to be deployed
        #[builder(into)]
        pub application: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Prefix to use for the fully qualified DNS name of
        /// the Environment.
        #[builder(into, default)]
        pub cname_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short description of the Environment
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique name for this Environment. This name is used
        /// in the application URL
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) of the Elastic Beanstalk [Platform](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html#cfn-beanstalk-environment-platformarn)
        /// to use in deployment
        #[builder(into, default)]
        pub platform_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time between polling the AWS API to
        /// check if changes have been applied. Use this to adjust the rate of API calls
        /// for any `create` or `update` action. Minimum `10s`, maximum `180s`. Omit this to
        /// use the default behavior, which is an exponential backoff
        #[builder(into, default)]
        pub poll_interval: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::elasticbeanstalk::EnvironmentSetting>>,
        >,
        /// A solution stack to base your environment
        /// off of. Example stacks can be found in the [Amazon API documentation](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html)
        #[builder(into, default)]
        pub solution_stack_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of tags to apply to the Environment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Elastic Beanstalk Configuration
        /// template to use in deployment
        #[builder(into, default)]
        pub template_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Elastic Beanstalk Environment tier. Valid values are `Worker`
        /// or `WebServer`. If tier is left blank `WebServer` will be used.
        #[builder(into, default)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Elastic Beanstalk Application Version
        /// to use in deployment.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that this provider should
        /// wait for an Elastic Beanstalk Environment to be in a ready state before timing
        /// out.
        #[builder(into, default)]
        pub wait_for_ready_timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// List of all option settings configured in this Environment. These
        /// are a combination of default settings and their overrides from `setting` in
        /// the configuration.
        pub all_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::elasticbeanstalk::EnvironmentAllSetting>,
        >,
        /// Name of the application that contains the version
        /// to be deployed
        pub application: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The autoscaling groups used by this Environment.
        pub autoscaling_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Fully qualified DNS name for this Environment.
        pub cname: pulumi_gestalt_rust::Output<String>,
        /// Prefix to use for the fully qualified DNS name of
        /// the Environment.
        pub cname_prefix: pulumi_gestalt_rust::Output<String>,
        /// Short description of the Environment
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL to the Load Balancer for this Environment
        pub endpoint_url: pulumi_gestalt_rust::Output<String>,
        /// Instances used by this Environment.
        pub instances: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Launch configurations in use by this Environment.
        pub launch_configurations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Elastic load balancers in use by this Environment.
        pub load_balancers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A unique name for this Environment. This name is used
        /// in the application URL
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The [ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) of the Elastic Beanstalk [Platform](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html#cfn-beanstalk-environment-platformarn)
        /// to use in deployment
        pub platform_arn: pulumi_gestalt_rust::Output<String>,
        /// The time between polling the AWS API to
        /// check if changes have been applied. Use this to adjust the rate of API calls
        /// for any `create` or `update` action. Minimum `10s`, maximum `180s`. Omit this to
        /// use the default behavior, which is an exponential backoff
        pub poll_interval: pulumi_gestalt_rust::Output<Option<String>>,
        /// SQS queues in use by this Environment.
        pub queues: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        pub settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::elasticbeanstalk::EnvironmentSetting>>,
        >,
        /// A solution stack to base your environment
        /// off of. Example stacks can be found in the [Amazon API documentation](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html)
        pub solution_stack_name: pulumi_gestalt_rust::Output<String>,
        /// A set of tags to apply to the Environment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Elastic Beanstalk Configuration
        /// template to use in deployment
        pub template_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Elastic Beanstalk Environment tier. Valid values are `Worker`
        /// or `WebServer`. If tier is left blank `WebServer` will be used.
        pub tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Autoscaling triggers in use by this Environment.
        pub triggers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Elastic Beanstalk Application Version
        /// to use in deployment.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that this provider should
        /// wait for an Elastic Beanstalk Environment to be in a ready state before timing
        /// out.
        pub wait_for_ready_timeout: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_binding = args.application.get_output(context);
        let cname_prefix_binding = args.cname_prefix.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let platform_arn_binding = args.platform_arn.get_output(context);
        let poll_interval_binding = args.poll_interval.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let solution_stack_name_binding = args.solution_stack_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_name_binding = args.template_name.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let version_binding = args.version.get_output(context);
        let wait_for_ready_timeout_binding = args
            .wait_for_ready_timeout
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "application".into(),
                    value: application_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cnamePrefix".into(),
                    value: cname_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformArn".into(),
                    value: platform_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pollInterval".into(),
                    value: poll_interval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "solutionStackName".into(),
                    value: solution_stack_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateName".into(),
                    value: template_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForReadyTimeout".into(),
                    value: wait_for_ready_timeout_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            all_settings: o.get_field("allSettings"),
            application: o.get_field("application"),
            arn: o.get_field("arn"),
            autoscaling_groups: o.get_field("autoscalingGroups"),
            cname: o.get_field("cname"),
            cname_prefix: o.get_field("cnamePrefix"),
            description: o.get_field("description"),
            endpoint_url: o.get_field("endpointUrl"),
            instances: o.get_field("instances"),
            launch_configurations: o.get_field("launchConfigurations"),
            load_balancers: o.get_field("loadBalancers"),
            name: o.get_field("name"),
            platform_arn: o.get_field("platformArn"),
            poll_interval: o.get_field("pollInterval"),
            queues: o.get_field("queues"),
            settings: o.get_field("settings"),
            solution_stack_name: o.get_field("solutionStackName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            template_name: o.get_field("templateName"),
            tier: o.get_field("tier"),
            triggers: o.get_field("triggers"),
            version: o.get_field("version"),
            wait_for_ready_timeout: o.get_field("waitForReadyTimeout"),
        }
    }
}
