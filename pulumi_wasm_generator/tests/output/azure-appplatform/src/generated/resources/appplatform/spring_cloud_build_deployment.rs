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
pub mod spring_cloud_build_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudBuildDeploymentArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Build Deployment.
        #[builder(into, default)]
        pub addon_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        #[builder(into, default)]
        pub application_performance_monitoring_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the Spring Cloud Build Result.
        #[builder(into)]
        pub build_result_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name which should be used for this Spring Cloud Build Deployment. Changing this forces a new Spring Cloud Build Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudBuildDeploymentQuota>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Build Deployment to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudBuildDeploymentResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Build Deployment.
        pub addon_json: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of Spring Cloud Application Performance Monitoring IDs.
        pub application_performance_monitoring_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the Spring Cloud Build Result.
        pub build_result_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name which should be used for this Spring Cloud Build Deployment. Changing this forces a new Spring Cloud Build Deployment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudBuildDeploymentQuota,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Build Deployment to be created.
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudBuildDeploymentArgs,
    ) -> SpringCloudBuildDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addon_json_binding = args.addon_json.get_inner();
        let application_performance_monitoring_ids_binding = args
            .application_performance_monitoring_ids
            .get_inner();
        let build_result_id_binding = args.build_result_id.get_inner();
        let environment_variables_binding = args.environment_variables.get_inner();
        let instance_count_binding = args.instance_count.get_inner();
        let name_binding = args.name.get_inner();
        let quota_binding = args.quota.get_inner();
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudBuildDeployment:SpringCloudBuildDeployment"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "addonJson".into(),
                },
                register_interface::ResultField {
                    name: "applicationPerformanceMonitoringIds".into(),
                },
                register_interface::ResultField {
                    name: "buildResultId".into(),
                },
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "quota".into(),
                },
                register_interface::ResultField {
                    name: "springCloudAppId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudBuildDeploymentResult {
            addon_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonJson").unwrap(),
            ),
            application_performance_monitoring_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationPerformanceMonitoringIds").unwrap(),
            ),
            build_result_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildResultId").unwrap(),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quota").unwrap(),
            ),
            spring_cloud_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudAppId").unwrap(),
            ),
        }
    }
}
