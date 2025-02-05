/// Manages a Spring Cloud Container Deployment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: E0
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleSpringCloudService.resourceGroupName}
///       serviceName: ${exampleSpringCloudService.name}
///   exampleSpringCloudContainerDeployment:
///     type: azure:appplatform:SpringCloudContainerDeployment
///     name: example
///     properties:
///       name: example
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       instanceCount: 2
///       arguments:
///         - -cp
///         - /app/resources:/app/classes:/app/libs/*
///         - hello.Application
///       commands:
///         - java
///       environmentVariables:
///         Foo: Bar
///         Env: Staging
///       server: docker.io
///       image: springio/gs-spring-boot-docker
///       languageFramework: springboot
/// ```
///
/// ## Import
///
/// Spring Cloud Container Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudContainerDeployment:SpringCloudContainerDeployment example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.AppPlatform/spring/spring1/apps/app1/deployments/deploy1
/// ```
///
pub mod spring_cloud_container_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudContainerDeploymentArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Container Deployment.
        #[builder(into, default)]
        pub addon_json: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        #[builder(into, default)]
        pub application_performance_monitoring_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the arguments to the entrypoint. The docker image's `CMD` is used if not specified.
        #[builder(into, default)]
        pub arguments: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the entrypoint array. It will not be executed within a shell. The docker image's `ENTRYPOINT` is used if not specified.
        #[builder(into, default)]
        pub commands: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Container image of the custom container. This should be in the form of `<repository>:<tag>` without the server name of the registry.
        #[builder(into)]
        pub image: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the language framework of the container image. The only possible value is `springboot`.
        #[builder(into, default)]
        pub language_framework: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Spring Cloud Container Deployment. Changing this forces a new Spring Cloud Container Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudContainerDeploymentQuota>,
        >,
        /// The name of the registry that contains the container image.
        #[builder(into)]
        pub server: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Container Deployment to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudContainerDeploymentResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Container Deployment.
        pub addon_json: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        pub application_performance_monitoring_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies the arguments to the entrypoint. The docker image's `CMD` is used if not specified.
        pub arguments: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the entrypoint array. It will not be executed within a shell. The docker image's `ENTRYPOINT` is used if not specified.
        pub commands: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Container image of the custom container. This should be in the form of `<repository>:<tag>` without the server name of the registry.
        pub image: pulumi_wasm_rust::Output<String>,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the language framework of the container image. The only possible value is `springboot`.
        pub language_framework: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Container Deployment. Changing this forces a new Spring Cloud Container Deployment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudContainerDeploymentQuota,
        >,
        /// The name of the registry that contains the container image.
        pub server: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Container Deployment to be created.
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpringCloudContainerDeploymentArgs,
    ) -> SpringCloudContainerDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addon_json_binding = args.addon_json.get_output(context).get_inner();
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_output(context)
            .get_inner();
        let arguments_binding = args.arguments.get_output(context).get_inner();
        let commands_binding = args.commands.get_output(context).get_inner();
        let environment_variables_binding = args
            .environment_variables
            .get_output(context)
            .get_inner();
        let image_binding = args.image.get_output(context).get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let language_framework_binding = args
            .language_framework
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let quota_binding = args.quota.get_output(context).get_inner();
        let server_binding = args.server.get_output(context).get_inner();
        let spring_cloud_app_id_binding = args
            .spring_cloud_app_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudContainerDeployment:SpringCloudContainerDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonJson".into(),
                    value: &addon_json_binding,
                },
                register_interface::ObjectField {
                    name: "applicationPerformanceMonitoringIds".into(),
                    value: &application_performance_monitoring_ids_binding,
                },
                register_interface::ObjectField {
                    name: "arguments".into(),
                    value: &arguments_binding,
                },
                register_interface::ObjectField {
                    name: "commands".into(),
                    value: &commands_binding,
                },
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "image".into(),
                    value: &image_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "languageFramework".into(),
                    value: &language_framework_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "quota".into(),
                    value: &quota_binding,
                },
                register_interface::ObjectField {
                    name: "server".into(),
                    value: &server_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudContainerDeploymentResult {
            addon_json: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addonJson"),
            ),
            application_performance_monitoring_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationPerformanceMonitoringIds"),
            ),
            arguments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arguments"),
            ),
            commands: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("commands"),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environmentVariables"),
            ),
            image: pulumi_wasm_rust::__private::into_domain(o.extract_field("image")),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            language_framework: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("languageFramework"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            quota: pulumi_wasm_rust::__private::into_domain(o.extract_field("quota")),
            server: pulumi_wasm_rust::__private::into_domain(o.extract_field("server")),
            spring_cloud_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("springCloudAppId"),
            ),
        }
    }
}
