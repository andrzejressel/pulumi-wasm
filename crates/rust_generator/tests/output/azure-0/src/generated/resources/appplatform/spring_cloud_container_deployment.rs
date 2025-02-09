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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_container_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudContainerDeploymentArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Container Deployment.
        #[builder(into, default)]
        pub addon_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        #[builder(into, default)]
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the arguments to the entrypoint. The docker image's `CMD` is used if not specified.
        #[builder(into, default)]
        pub arguments: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the entrypoint array. It will not be executed within a shell. The docker image's `ENTRYPOINT` is used if not specified.
        #[builder(into, default)]
        pub commands: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Container image of the custom container. This should be in the form of `<repository>:<tag>` without the server name of the registry.
        #[builder(into)]
        pub image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the language framework of the container image. The only possible value is `springboot`.
        #[builder(into, default)]
        pub language_framework: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Spring Cloud Container Deployment. Changing this forces a new Spring Cloud Container Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudContainerDeploymentQuota>,
        >,
        /// The name of the registry that contains the container image.
        #[builder(into)]
        pub server: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Container Deployment to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudContainerDeploymentResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Container Deployment.
        pub addon_json: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies the arguments to the entrypoint. The docker image's `CMD` is used if not specified.
        pub arguments: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the entrypoint array. It will not be executed within a shell. The docker image's `ENTRYPOINT` is used if not specified.
        pub commands: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Container image of the custom container. This should be in the form of `<repository>:<tag>` without the server name of the registry.
        pub image: pulumi_gestalt_rust::Output<String>,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the language framework of the container image. The only possible value is `springboot`.
        pub language_framework: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Container Deployment. Changing this forces a new Spring Cloud Container Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudContainerDeploymentQuota,
        >,
        /// The name of the registry that contains the container image.
        pub server: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Container Deployment to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudContainerDeploymentArgs,
    ) -> SpringCloudContainerDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addon_json_binding = args.addon_json.get_output(context);
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_output(context);
        let arguments_binding = args.arguments.get_output(context);
        let commands_binding = args.commands.get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let image_binding = args.image.get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let language_framework_binding = args.language_framework.get_output(context);
        let name_binding = args.name.get_output(context);
        let quota_binding = args.quota.get_output(context);
        let server_binding = args.server.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudContainerDeployment:SpringCloudContainerDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonJson".into(),
                    value: addon_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationPerformanceMonitoringIds".into(),
                    value: application_performance_monitoring_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arguments".into(),
                    value: arguments_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commands".into(),
                    value: commands_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: environment_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "image".into(),
                    value: image_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: instance_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageFramework".into(),
                    value: language_framework_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quota".into(),
                    value: quota_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "server".into(),
                    value: server_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: spring_cloud_app_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudContainerDeploymentResult {
            addon_json: o.get_field("addonJson"),
            application_performance_monitoring_ids: o
                .get_field("applicationPerformanceMonitoringIds"),
            arguments: o.get_field("arguments"),
            commands: o.get_field("commands"),
            environment_variables: o.get_field("environmentVariables"),
            image: o.get_field("image"),
            instance_count: o.get_field("instanceCount"),
            language_framework: o.get_field("languageFramework"),
            name: o.get_field("name"),
            quota: o.get_field("quota"),
            server: o.get_field("server"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
