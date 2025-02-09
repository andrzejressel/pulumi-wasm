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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_workloads_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserWorkloadsSecretArgs {
        /// A map of the secret data.
        #[builder(into, default)]
        pub data: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes Secret will be stored and used.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Kubernetes Secret.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location or Compute Engine region for the environment.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserWorkloadsSecretResult {
        /// A map of the secret data.
        pub data: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Environment where the Kubernetes Secret will be stored and used.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// Name of the Kubernetes Secret.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The location or Compute Engine region for the environment.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserWorkloadsSecretArgs,
    ) -> UserWorkloadsSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_binding = args.data.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:composer/userWorkloadsSecret:UserWorkloadsSecret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: environment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserWorkloadsSecretResult {
            data: o.get_field("data"),
            environment: o.get_field("environment"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
