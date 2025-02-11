/// Manages a Spring Cloud Build Deployment.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
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
///   exampleSpringCloudBuildDeployment:
///     type: azure:appplatform:SpringCloudBuildDeployment
///     name: example
///     properties:
///       name: example
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       buildResultId: <default>
///       instanceCount: 2
///       environmentVariables:
///         Foo: Bar
///         Env: Staging
///       quota:
///         cpu: '2'
///         memory: 4Gi
/// ```
///
/// ## Import
///
/// Spring Cloud Build Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudBuildDeployment:SpringCloudBuildDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.AppPlatform/spring/spring1/apps/app1/deployments/deploy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_build_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudBuildDeploymentArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Build Deployment.
        #[builder(into, default)]
        pub addon_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        #[builder(into, default)]
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the Spring Cloud Build Result.
        #[builder(into)]
        pub build_result_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Spring Cloud Build Deployment. Changing this forces a new Spring Cloud Build Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudBuildDeploymentQuota>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Build Deployment to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudBuildDeploymentResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Build Deployment.
        pub addon_json: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        pub application_performance_monitoring_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the Spring Cloud Build Result.
        pub build_result_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name which should be used for this Spring Cloud Build Deployment. Changing this forces a new Spring Cloud Build Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudBuildDeploymentQuota,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Build Deployment to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudBuildDeploymentArgs,
    ) -> SpringCloudBuildDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addon_json_binding = args.addon_json.get_output(context);
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_output(context);
        let build_result_id_binding = args.build_result_id.get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let name_binding = args.name.get_output(context);
        let quota_binding = args.quota.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudBuildDeployment:SpringCloudBuildDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonJson".into(),
                    value: &addon_json_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationPerformanceMonitoringIds".into(),
                    value: &application_performance_monitoring_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildResultId".into(),
                    value: &build_result_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quota".into(),
                    value: &quota_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudBuildDeploymentResult {
            addon_json: o.get_field("addonJson"),
            application_performance_monitoring_ids: o
                .get_field("applicationPerformanceMonitoringIds"),
            build_result_id: o.get_field("buildResultId"),
            environment_variables: o.get_field("environmentVariables"),
            instance_count: o.get_field("instanceCount"),
            name: o.get_field("name"),
            quota: o.get_field("quota"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
