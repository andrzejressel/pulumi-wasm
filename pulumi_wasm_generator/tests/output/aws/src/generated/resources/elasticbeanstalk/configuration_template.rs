/// Provides an Elastic Beanstalk Configuration Template, which are associated with
/// a specific application and are used to deploy different versions of the
/// application with the same configuration settings.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myTemplate = configuration_template::create(
///         "myTemplate",
///         ConfigurationTemplateArgs::builder()
///             .application("${tftest.name}")
///             .name("tf-test-template-config")
///             .solution_stack_name("64bit Amazon Linux 2015.09 v2.0.8 running Go 1.4")
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
/// The `setting` field supports the following format:
///
/// * `namespace` - unique namespace identifying the option's associated AWS resource
/// * `name` - name of the configuration option
/// * `value` - value for the configuration option
/// * `resource` - (Optional) resource name for [scheduled action](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options-general.html#command-options-general-autoscalingscheduledaction)
pub mod configuration_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationTemplateArgs {
        /// name of the application to associate with this configuration template
        #[builder(into)]
        pub application: pulumi_wasm_rust::Output<String>,
        /// Short description of the Template
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the environment used with this configuration template
        #[builder(into, default)]
        pub environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for this Template.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::elasticbeanstalk::ConfigurationTemplateSetting>,
            >,
        >,
        /// A solution stack to base your Template
        /// off of. Example stacks can be found in the [Amazon API documentation][1]
        #[builder(into, default)]
        pub solution_stack_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationTemplateResult {
        /// name of the application to associate with this configuration template
        pub application: pulumi_wasm_rust::Output<String>,
        /// Short description of the Template
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the environment used with this configuration template
        pub environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for this Template.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Option settings to configure the new Environment. These
        /// override specific values that are set as defaults. The format is detailed
        /// below in Option Settings
        pub settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::elasticbeanstalk::ConfigurationTemplateSetting>,
        >,
        /// A solution stack to base your Template
        /// off of. Example stacks can be found in the [Amazon API documentation][1]
        pub solution_stack_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConfigurationTemplateArgs,
    ) -> ConfigurationTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_binding = args.application.get_inner();
        let description_binding = args.description.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let name_binding = args.name.get_inner();
        let settings_binding = args.settings.get_inner();
        let solution_stack_name_binding = args.solution_stack_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/configurationTemplate:ConfigurationTemplate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "application".into(),
                    value: &application_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "solutionStackName".into(),
                    value: &solution_stack_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "application".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "solutionStackName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationTemplateResult {
            application: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("application").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            solution_stack_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("solutionStackName").unwrap(),
            ),
        }
    }
}