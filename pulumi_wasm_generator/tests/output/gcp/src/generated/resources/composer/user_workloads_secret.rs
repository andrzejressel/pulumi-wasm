/// User workloads Secret used by Airflow tasks that run with Kubernetes Executor or KubernetesPodOperator.
/// Intended for Composer 3 Environments.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:composer:Environment
///     properties:
///       name: example-environment
///       project: example-project
///       region: us-central1
///       config:
///         softwareConfig:
///           imageVersion: example-image-version
///   exampleUserWorkloadsSecret:
///     type: gcp:composer:UserWorkloadsSecret
///     name: example
///     properties:
///       name: example-secret
///       project: example-project
///       region: us-central1
///       environment: ${example.name}
///       data:
///         email:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: example-email
///             return: result
///         password:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: example-password
///             return: result
/// ```
///
/// ## Import
///
/// Secret can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/environments/{{environment}}/userWorkloadsSecrets/{{name}}`
///
/// * `{{project}}/{{region}}/{{environment}}/{{name}}`
///
/// * `{{environment}}/{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsSecret:UserWorkloadsSecret example projects/{{project}}/locations/{{region}}/environments/{{environment}}/userWorkloadsSecrets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsSecret:UserWorkloadsSecret example {{project}}/{{region}}/{{environment}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/userWorkloadsSecret:UserWorkloadsSecret example {{environment}}/{{name}}
/// ```
///
pub mod user_workloads_secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserWorkloadsSecretArgs {
        /// A map of the secret data.
        #[builder(into, default)]
        pub data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes Secret will be stored and used.
        #[builder(into)]
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Name of the Kubernetes Secret.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The location or Compute Engine region for the environment.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserWorkloadsSecretResult {
        /// A map of the secret data.
        pub data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes Secret will be stored and used.
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Name of the Kubernetes Secret.
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
        name: &str,
        args: UserWorkloadsSecretArgs,
    ) -> UserWorkloadsSecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_inner();
        let environment_binding = args.environment.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:composer/userWorkloadsSecret:UserWorkloadsSecret".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserWorkloadsSecretResult {
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
