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
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// SCM configuration of the app as described below.
        #[builder(into, default)]
        pub app_sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationAppSource>>,
        >,
        /// Run bundle install when deploying for application of type `rails`.
        #[builder(into, default)]
        pub auto_bundle_on_deploy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify activity and workflow workers for your app using the aws-flow gem.
        #[builder(into, default)]
        pub aws_flow_ruby_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// The data source's ARN.
        #[builder(into, default)]
        pub data_source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The database name.
        #[builder(into, default)]
        pub data_source_database_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The data source's type one of `AutoSelectOpsworksMysqlInstance`, `OpsworksMysqlInstance`, or `RdsDbInstance`.
        #[builder(into, default)]
        pub data_source_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the app.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Subfolder for the document root for application of type `rails`.
        #[builder(into, default)]
        pub document_root: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of virtual host alias.
        #[builder(into, default)]
        pub domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to enable SSL for the app. This must be set in order to let `ssl_configuration.private_key`, `ssl_configuration.certificate` and `ssl_configuration.chain` take effect.
        #[builder(into, default)]
        pub enable_ssl: pulumi_wasm_rust::Output<Option<bool>>,
        /// Object to define environment variables.  Object is described below.
        #[builder(into, default)]
        pub environments: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationEnvironment>>,
        >,
        /// A human-readable name for the application.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Rails environment for application of type `rails`.
        #[builder(into, default)]
        pub rails_env: pulumi_wasm_rust::Output<Option<String>>,
        /// A short, machine-readable name for the application. This can only be defined on resource creation and ignored on resource update.
        #[builder(into, default)]
        pub short_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The SSL configuration of the app. Object is described below.
        #[builder(into, default)]
        pub ssl_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationSslConfiguration>>,
        >,
        /// ID of the stack the application will belong to.
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Opsworks application type. One of `aws-flow-ruby`, `java`, `rails`, `php`, `nodejs`, `static` or `other`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// SCM configuration of the app as described below.
        pub app_sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::ApplicationAppSource>,
        >,
        /// Run bundle install when deploying for application of type `rails`.
        pub auto_bundle_on_deploy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify activity and workflow workers for your app using the aws-flow gem.
        pub aws_flow_ruby_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// The data source's ARN.
        pub data_source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The database name.
        pub data_source_database_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The data source's type one of `AutoSelectOpsworksMysqlInstance`, `OpsworksMysqlInstance`, or `RdsDbInstance`.
        pub data_source_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the app.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Subfolder for the document root for application of type `rails`.
        pub document_root: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of virtual host alias.
        pub domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to enable SSL for the app. This must be set in order to let `ssl_configuration.private_key`, `ssl_configuration.certificate` and `ssl_configuration.chain` take effect.
        pub enable_ssl: pulumi_wasm_rust::Output<Option<bool>>,
        /// Object to define environment variables.  Object is described below.
        pub environments: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationEnvironment>>,
        >,
        /// A human-readable name for the application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Rails environment for application of type `rails`.
        pub rails_env: pulumi_wasm_rust::Output<Option<String>>,
        /// A short, machine-readable name for the application. This can only be defined on resource creation and ignored on resource update.
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// The SSL configuration of the app. Object is described below.
        pub ssl_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::ApplicationSslConfiguration>>,
        >,
        /// ID of the stack the application will belong to.
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Opsworks application type. One of `aws-flow-ruby`, `java`, `rails`, `php`, `nodejs`, `static` or `other`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_sources_binding = args.app_sources.get_inner();
        let auto_bundle_on_deploy_binding = args.auto_bundle_on_deploy.get_inner();
        let aws_flow_ruby_settings_binding = args.aws_flow_ruby_settings.get_inner();
        let data_source_arn_binding = args.data_source_arn.get_inner();
        let data_source_database_name_binding = args
            .data_source_database_name
            .get_inner();
        let data_source_type_binding = args.data_source_type.get_inner();
        let description_binding = args.description.get_inner();
        let document_root_binding = args.document_root.get_inner();
        let domains_binding = args.domains.get_inner();
        let enable_ssl_binding = args.enable_ssl.get_inner();
        let environments_binding = args.environments.get_inner();
        let name_binding = args.name.get_inner();
        let rails_env_binding = args.rails_env.get_inner();
        let short_name_binding = args.short_name.get_inner();
        let ssl_configurations_binding = args.ssl_configurations.get_inner();
        let stack_id_binding = args.stack_id.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/application:Application".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appSources".into(),
                },
                register_interface::ResultField {
                    name: "autoBundleOnDeploy".into(),
                },
                register_interface::ResultField {
                    name: "awsFlowRubySettings".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceArn".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceDatabaseName".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "documentRoot".into(),
                },
                register_interface::ResultField {
                    name: "domains".into(),
                },
                register_interface::ResultField {
                    name: "enableSsl".into(),
                },
                register_interface::ResultField {
                    name: "environments".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "railsEnv".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "sslConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            app_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSources").unwrap(),
            ),
            auto_bundle_on_deploy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoBundleOnDeploy").unwrap(),
            ),
            aws_flow_ruby_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsFlowRubySettings").unwrap(),
            ),
            data_source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceArn").unwrap(),
            ),
            data_source_database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceDatabaseName").unwrap(),
            ),
            data_source_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            document_root: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentRoot").unwrap(),
            ),
            domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domains").unwrap(),
            ),
            enable_ssl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSsl").unwrap(),
            ),
            environments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environments").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rails_env: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("railsEnv").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            ssl_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslConfigurations").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
