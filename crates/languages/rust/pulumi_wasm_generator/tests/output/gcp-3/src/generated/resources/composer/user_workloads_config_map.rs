/// User workloads ConfigMap used by Airflow tasks that run with Kubernetes Executor or KubernetesPodOperator.
/// Intended for Composer 3 Environments.
///
///
/// To get more information about UserWorkloadsConfigMap, see:
///
/// * [API documentation](https://cloud.google.com/composer/docs/reference/rest/v1/projects.locations.environments.userWorkloadsConfigMaps)
///
/// ## Example Usage
///
/// ### Composer User Workloads Config Map Basic
///
///
/// ```yaml
/// resources:
///   environment:
///     type: gcp:composer:Environment
///     properties:
///       name: test-environment
///       region: us-central1
///       config:
///         softwareConfig:
///           imageVersion: composer-3-airflow-2
///   configMap:
///     type: gcp:composer:UserWorkloadsConfigMap
///     name: config_map
///     properties:
///       name: test-config-map
///       region: us-central1
///       environment: ${environment.name}
///       data:
///         api_host: apihost:443
/// ```
///
/// ## Import
///
/// UserWorkloadsConfigMap can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/environments/{{environment}}/userWorkloadsConfigMaps/{{name}}`
///
/// * `{{project}}/{{region}}/{{environment}}/{{name}}`
///
/// * `{{region}}/{{environment}}/{{name}}`
///
/// * `{{environment}}/{{name}}`
///
/// When using the `pulumi import` command, UserWorkloadsConfigMap can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsConfigMap:UserWorkloadsConfigMap default projects/{{project}}/locations/{{region}}/environments/{{environment}}/userWorkloadsConfigMaps/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsConfigMap:UserWorkloadsConfigMap default {{project}}/{{region}}/{{environment}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsConfigMap:UserWorkloadsConfigMap default {{region}}/{{environment}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsConfigMap:UserWorkloadsConfigMap default {{environment}}/{{name}}
/// ```
///
pub mod user_workloads_config_map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserWorkloadsConfigMapArgs {
        /// The "data" field of Kubernetes ConfigMap, organized in key-value pairs.
        /// For details see: https://kubernetes.io/docs/concepts/configuration/configmap/
        #[builder(into, default)]
        pub data: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes ConfigMap will be stored and used.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub environment: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Kubernetes ConfigMap.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location or Compute Engine region for the environment.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserWorkloadsConfigMapResult {
        /// The "data" field of Kubernetes ConfigMap, organized in key-value pairs.
        /// For details see: https://kubernetes.io/docs/concepts/configuration/configmap/
        pub data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes ConfigMap will be stored and used.
        ///
        ///
        /// - - -
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Name of the Kubernetes ConfigMap.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The location or Compute Engine region for the environment.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserWorkloadsConfigMapArgs,
    ) -> UserWorkloadsConfigMapResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_output(context).get_inner();
        let environment_binding = args.environment.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:composer/userWorkloadsConfigMap:UserWorkloadsConfigMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserWorkloadsConfigMapResult {
            data: pulumi_wasm_rust::__private::into_domain(o.extract_field("data")),
            environment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
        }
    }
}
