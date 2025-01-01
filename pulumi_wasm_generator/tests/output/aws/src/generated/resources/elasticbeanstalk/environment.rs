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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Name of the application that contains the version
        /// to be deployed
        #[builder(into)]
        pub application: pulumi_wasm_rust::Output<String>,
        /// Prefix to use for the fully qualified DNS name of
        /// the Environment.
        #[builder(into, default)]
        pub cname_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Short description of the Environment
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for this Environment. This name is used
        /// in the application URL
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The [ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) of the Elastic Beanstalk [Platform](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html#cfn-beanstalk-environment-platformarn)
        /// to use in deployment
        #[builder(into, default)]
        pub platform_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The time between polling the AWS API to
        /// check if changes have been applied. Use this to adjust the rate of API calls
        /// for any `create` or `update` action. Minimum `10s`, maximum `180s`. Omit this to
        /// use the default behavior, which is an exponential backoff
        #[builder(into, default)]
        pub poll_interval: pulumi_wasm_rust::Output<Option<String>>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticbeanstalk::EnvironmentSetting>>,
        >,
        /// A solution stack to base your environment
        /// off of. Example stacks can be found in the [Amazon API documentation](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html)
        #[builder(into, default)]
        pub solution_stack_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of tags to apply to the Environment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Elastic Beanstalk Configuration
        /// template to use in deployment
        #[builder(into, default)]
        pub template_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Elastic Beanstalk Environment tier. Valid values are `Worker`
        /// or `WebServer`. If tier is left blank `WebServer` will be used.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Elastic Beanstalk Application Version
        /// to use in deployment.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticbeanstalk::ApplicationVersion>,
        >,
        /// The maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that this provider should
        /// wait for an Elastic Beanstalk Environment to be in a ready state before timing
        /// out.
        #[builder(into, default)]
        pub wait_for_ready_timeout: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// List of all option settings configured in this Environment. These
        /// are a combination of default settings and their overrides from `setting` in
        /// the configuration.
        pub all_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::elasticbeanstalk::EnvironmentAllSetting>,
        >,
        /// Name of the application that contains the version
        /// to be deployed
        pub application: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The autoscaling groups used by this Environment.
        pub autoscaling_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Fully qualified DNS name for this Environment.
        pub cname: pulumi_wasm_rust::Output<String>,
        /// Prefix to use for the fully qualified DNS name of
        /// the Environment.
        pub cname_prefix: pulumi_wasm_rust::Output<String>,
        /// Short description of the Environment
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL to the Load Balancer for this Environment
        pub endpoint_url: pulumi_wasm_rust::Output<String>,
        /// Instances used by this Environment.
        pub instances: pulumi_wasm_rust::Output<Vec<String>>,
        /// Launch configurations in use by this Environment.
        pub launch_configurations: pulumi_wasm_rust::Output<Vec<String>>,
        /// Elastic load balancers in use by this Environment.
        pub load_balancers: pulumi_wasm_rust::Output<Vec<String>>,
        /// A unique name for this Environment. This name is used
        /// in the application URL
        pub name: pulumi_wasm_rust::Output<String>,
        /// The [ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) of the Elastic Beanstalk [Platform](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-beanstalk-environment.html#cfn-beanstalk-environment-platformarn)
        /// to use in deployment
        pub platform_arn: pulumi_wasm_rust::Output<String>,
        /// The time between polling the AWS API to
        /// check if changes have been applied. Use this to adjust the rate of API calls
        /// for any `create` or `update` action. Minimum `10s`, maximum `180s`. Omit this to
        /// use the default behavior, which is an exponential backoff
        pub poll_interval: pulumi_wasm_rust::Output<Option<String>>,
        /// SQS queues in use by this Environment.
        pub queues: pulumi_wasm_rust::Output<Vec<String>>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticbeanstalk::EnvironmentSetting>>,
        >,
        /// A solution stack to base your environment
        /// off of. Example stacks can be found in the [Amazon API documentation](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html)
        pub solution_stack_name: pulumi_wasm_rust::Output<String>,
        /// A set of tags to apply to the Environment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Elastic Beanstalk Configuration
        /// template to use in deployment
        pub template_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Elastic Beanstalk Environment tier. Valid values are `Worker`
        /// or `WebServer`. If tier is left blank `WebServer` will be used.
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// Autoscaling triggers in use by this Environment.
        pub triggers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the Elastic Beanstalk Application Version
        /// to use in deployment.
        pub version: pulumi_wasm_rust::Output<
            super::super::types::elasticbeanstalk::ApplicationVersion,
        >,
        /// The maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that this provider should
        /// wait for an Elastic Beanstalk Environment to be in a ready state before timing
        /// out.
        pub wait_for_ready_timeout: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_binding = args.application.get_inner();
        let cname_prefix_binding = args.cname_prefix.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let platform_arn_binding = args.platform_arn.get_inner();
        let poll_interval_binding = args.poll_interval.get_inner();
        let settings_binding = args.settings.get_inner();
        let solution_stack_name_binding = args.solution_stack_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_name_binding = args.template_name.get_inner();
        let tier_binding = args.tier.get_inner();
        let version_binding = args.version.get_inner();
        let wait_for_ready_timeout_binding = args.wait_for_ready_timeout.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/environment:Environment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "application".into(),
                    value: &application_binding,
                },
                register_interface::ObjectField {
                    name: "cnamePrefix".into(),
                    value: &cname_prefix_binding,
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
                    name: "platformArn".into(),
                    value: &platform_arn_binding,
                },
                register_interface::ObjectField {
                    name: "pollInterval".into(),
                    value: &poll_interval_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "solutionStackName".into(),
                    value: &solution_stack_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateName".into(),
                    value: &template_name_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "waitForReadyTimeout".into(),
                    value: &wait_for_ready_timeout_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allSettings".into(),
                },
                register_interface::ResultField {
                    name: "application".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoscalingGroups".into(),
                },
                register_interface::ResultField {
                    name: "cname".into(),
                },
                register_interface::ResultField {
                    name: "cnamePrefix".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endpointUrl".into(),
                },
                register_interface::ResultField {
                    name: "instances".into(),
                },
                register_interface::ResultField {
                    name: "launchConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platformArn".into(),
                },
                register_interface::ResultField {
                    name: "pollInterval".into(),
                },
                register_interface::ResultField {
                    name: "queues".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "solutionStackName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "templateName".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "waitForReadyTimeout".into(),
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
            all_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allSettings").unwrap(),
            ),
            application: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("application").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            autoscaling_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingGroups").unwrap(),
            ),
            cname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cname").unwrap(),
            ),
            cname_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cnamePrefix").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            endpoint_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUrl").unwrap(),
            ),
            instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instances").unwrap(),
            ),
            launch_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchConfigurations").unwrap(),
            ),
            load_balancers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformArn").unwrap(),
            ),
            poll_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pollInterval").unwrap(),
            ),
            queues: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queues").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            solution_stack_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("solutionStackName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            template_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateName").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            wait_for_ready_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForReadyTimeout").unwrap(),
            ),
        }
    }
}
