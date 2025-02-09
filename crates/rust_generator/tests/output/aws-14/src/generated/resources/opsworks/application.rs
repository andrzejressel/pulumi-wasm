/// Provides an OpsWorks application resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo-app:
///     type: aws:opsworks:Application
///     properties:
///       name: foobar application
///       shortName: foobar
///       stackId: ${main.id}
///       type: rails
///       description: This is a Rails application
///       domains:
///         - example.com
///         - sub.example.com
///       environments:
///         - key: key
///           value: value
///           secure: false
///       appSources:
///         - type: git
///           revision: master
///           url: https://github.com/example.git
///       enableSsl: true
///       sslConfigurations:
///         - privateKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ./foobar.key
///               return: result
///           certificate:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ./foobar.crt
///               return: result
///       documentRoot: public
///       autoBundleOnDeploy: true
///       railsEnv: staging
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Opsworks Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opsworks/application:Application test <id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// SCM configuration of the app as described below.
        #[builder(into, default)]
        pub app_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::ApplicationAppSource>>,
        >,
        /// Run bundle install when deploying for application of type `rails`.
        #[builder(into, default)]
        pub auto_bundle_on_deploy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify activity and workflow workers for your app using the aws-flow gem.
        #[builder(into, default)]
        pub aws_flow_ruby_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The data source's ARN.
        #[builder(into, default)]
        pub data_source_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The database name.
        #[builder(into, default)]
        pub data_source_database_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The data source's type one of `AutoSelectOpsworksMysqlInstance`, `OpsworksMysqlInstance`, or `RdsDbInstance`.
        #[builder(into, default)]
        pub data_source_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of the app.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Subfolder for the document root for application of type `rails`.
        #[builder(into, default)]
        pub document_root: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of virtual host alias.
        #[builder(into, default)]
        pub domains: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to enable SSL for the app. This must be set in order to let `ssl_configuration.private_key`, `ssl_configuration.certificate` and `ssl_configuration.chain` take effect.
        #[builder(into, default)]
        pub enable_ssl: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Object to define environment variables.  Object is described below.
        #[builder(into, default)]
        pub environments: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::ApplicationEnvironment>>,
        >,
        /// A human-readable name for the application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Rails environment for application of type `rails`.
        #[builder(into, default)]
        pub rails_env: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short, machine-readable name for the application. This can only be defined on resource creation and ignored on resource update.
        #[builder(into, default)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SSL configuration of the app. Object is described below.
        #[builder(into, default)]
        pub ssl_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::ApplicationSslConfiguration>>,
        >,
        /// ID of the stack the application will belong to.
        #[builder(into)]
        pub stack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Opsworks application type. One of `aws-flow-ruby`, `java`, `rails`, `php`, `nodejs`, `static` or `other`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// SCM configuration of the app as described below.
        pub app_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::opsworks::ApplicationAppSource>,
        >,
        /// Run bundle install when deploying for application of type `rails`.
        pub auto_bundle_on_deploy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify activity and workflow workers for your app using the aws-flow gem.
        pub aws_flow_ruby_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// The data source's ARN.
        pub data_source_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The database name.
        pub data_source_database_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The data source's type one of `AutoSelectOpsworksMysqlInstance`, `OpsworksMysqlInstance`, or `RdsDbInstance`.
        pub data_source_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of the app.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Subfolder for the document root for application of type `rails`.
        pub document_root: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of virtual host alias.
        pub domains: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Whether to enable SSL for the app. This must be set in order to let `ssl_configuration.private_key`, `ssl_configuration.certificate` and `ssl_configuration.chain` take effect.
        pub enable_ssl: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Object to define environment variables.  Object is described below.
        pub environments: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationEnvironment>>,
        >,
        /// A human-readable name for the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Rails environment for application of type `rails`.
        pub rails_env: pulumi_gestalt_rust::Output<Option<String>>,
        /// A short, machine-readable name for the application. This can only be defined on resource creation and ignored on resource update.
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// The SSL configuration of the app. Object is described below.
        pub ssl_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationSslConfiguration>>,
        >,
        /// ID of the stack the application will belong to.
        pub stack_id: pulumi_gestalt_rust::Output<String>,
        /// Opsworks application type. One of `aws-flow-ruby`, `java`, `rails`, `php`, `nodejs`, `static` or `other`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_sources_binding_1 = args.app_sources.get_output(context);
        let app_sources_binding = app_sources_binding_1.get_inner();
        let auto_bundle_on_deploy_binding_1 = args
            .auto_bundle_on_deploy
            .get_output(context);
        let auto_bundle_on_deploy_binding = auto_bundle_on_deploy_binding_1.get_inner();
        let aws_flow_ruby_settings_binding_1 = args
            .aws_flow_ruby_settings
            .get_output(context);
        let aws_flow_ruby_settings_binding = aws_flow_ruby_settings_binding_1
            .get_inner();
        let data_source_arn_binding_1 = args.data_source_arn.get_output(context);
        let data_source_arn_binding = data_source_arn_binding_1.get_inner();
        let data_source_database_name_binding_1 = args
            .data_source_database_name
            .get_output(context);
        let data_source_database_name_binding = data_source_database_name_binding_1
            .get_inner();
        let data_source_type_binding_1 = args.data_source_type.get_output(context);
        let data_source_type_binding = data_source_type_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let document_root_binding_1 = args.document_root.get_output(context);
        let document_root_binding = document_root_binding_1.get_inner();
        let domains_binding_1 = args.domains.get_output(context);
        let domains_binding = domains_binding_1.get_inner();
        let enable_ssl_binding_1 = args.enable_ssl.get_output(context);
        let enable_ssl_binding = enable_ssl_binding_1.get_inner();
        let environments_binding_1 = args.environments.get_output(context);
        let environments_binding = environments_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let rails_env_binding_1 = args.rails_env.get_output(context);
        let rails_env_binding = rails_env_binding_1.get_inner();
        let short_name_binding_1 = args.short_name.get_output(context);
        let short_name_binding = short_name_binding_1.get_inner();
        let ssl_configurations_binding_1 = args.ssl_configurations.get_output(context);
        let ssl_configurations_binding = ssl_configurations_binding_1.get_inner();
        let stack_id_binding_1 = args.stack_id.get_output(context);
        let stack_id_binding = stack_id_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appSources".into(),
                    value: &app_sources_binding,
                },
                register_interface::ObjectField {
                    name: "autoBundleOnDeploy".into(),
                    value: &auto_bundle_on_deploy_binding,
                },
                register_interface::ObjectField {
                    name: "awsFlowRubySettings".into(),
                    value: &aws_flow_ruby_settings_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceArn".into(),
                    value: &data_source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceDatabaseName".into(),
                    value: &data_source_database_name_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceType".into(),
                    value: &data_source_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "documentRoot".into(),
                    value: &document_root_binding,
                },
                register_interface::ObjectField {
                    name: "domains".into(),
                    value: &domains_binding,
                },
                register_interface::ObjectField {
                    name: "enableSsl".into(),
                    value: &enable_ssl_binding,
                },
                register_interface::ObjectField {
                    name: "environments".into(),
                    value: &environments_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "railsEnv".into(),
                    value: &rails_env_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
                register_interface::ObjectField {
                    name: "sslConfigurations".into(),
                    value: &ssl_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationResult {
            app_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appSources"),
            ),
            auto_bundle_on_deploy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoBundleOnDeploy"),
            ),
            aws_flow_ruby_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsFlowRubySettings"),
            ),
            data_source_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceArn"),
            ),
            data_source_database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceDatabaseName"),
            ),
            data_source_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceType"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            document_root: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentRoot"),
            ),
            domains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domains"),
            ),
            enable_ssl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableSsl"),
            ),
            environments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environments"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            rails_env: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("railsEnv"),
            ),
            short_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
            ssl_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslConfigurations"),
            ),
            stack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackId"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
