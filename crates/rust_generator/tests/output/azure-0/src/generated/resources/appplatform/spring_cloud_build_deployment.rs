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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpringCloudBuildDeploymentArgs,
    ) -> SpringCloudBuildDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let addon_json_binding = args.addon_json.get_output(context).get_inner();
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_output(context)
            .get_inner();
        let build_result_id_binding = args
            .build_result_id
            .get_output(context)
            .get_inner();
        let environment_variables_binding = args
            .environment_variables
            .get_output(context)
            .get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let quota_binding = args.quota.get_output(context).get_inner();
        let spring_cloud_app_id_binding = args
            .spring_cloud_app_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudBuildDeployment:SpringCloudBuildDeployment"
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
                    name: "buildResultId".into(),
                    value: &build_result_id_binding,
                },
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
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
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudBuildDeploymentResult {
            addon_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addonJson"),
            ),
            application_performance_monitoring_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationPerformanceMonitoringIds"),
            ),
            build_result_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buildResultId"),
            ),
            environment_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentVariables"),
            ),
            instance_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            quota: pulumi_gestalt_rust::__private::into_domain(o.extract_field("quota")),
            spring_cloud_app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("springCloudAppId"),
            ),
        }
    }
}
