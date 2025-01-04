/// Manages an Azure Spring Cloud Deployment with a Java runtime.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with basic and standard tier.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///       identity:
///         type: SystemAssigned
///   exampleSpringCloudJavaDeployment:
///     type: azure:appplatform:SpringCloudJavaDeployment
///     name: example
///     properties:
///       name: deploy1
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       instanceCount: 2
///       jvmOptions: -XX:+PrintGC
///       quota:
///         cpu: '2'
///         memory: 4Gi
///       runtimeVersion: Java_11
///       environmentVariables:
///         Foo: Bar
///         Env: Staging
/// ```
///
/// ## Import
///
/// Spring Cloud Deployment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudJavaDeployment:SpringCloudJavaDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/service1/apps/app1/deployments/deploy1
/// ```
///
pub mod spring_cloud_java_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudJavaDeploymentArgs {
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the jvm option of the Spring Cloud Deployment.
        #[builder(into, default)]
        pub jvm_options: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Deployment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudJavaDeploymentQuota>,
        >,
        /// Specifies the runtime version of the Spring Cloud Deployment. Possible Values are `Java_8`, `Java_11` and `Java_17`. Defaults to `Java_8`.
        #[builder(into, default)]
        pub runtime_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the id of the Spring Cloud Application in which to create the Deployment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudJavaDeploymentResult {
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the jvm option of the Spring Cloud Deployment.
        pub jvm_options: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Deployment. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudJavaDeploymentQuota,
        >,
        /// Specifies the runtime version of the Spring Cloud Deployment. Possible Values are `Java_8`, `Java_11` and `Java_17`. Defaults to `Java_8`.
        pub runtime_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the id of the Spring Cloud Application in which to create the Deployment. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudJavaDeploymentArgs,
    ) -> SpringCloudJavaDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let environment_variables_binding = args.environment_variables.get_inner();
        let instance_count_binding = args.instance_count.get_inner();
        let jvm_options_binding = args.jvm_options.get_inner();
        let name_binding = args.name.get_inner();
        let quota_binding = args.quota.get_inner();
        let runtime_version_binding = args.runtime_version.get_inner();
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudJavaDeployment:SpringCloudJavaDeployment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "jvmOptions".into(),
                    value: &jvm_options_binding,
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
                    name: "runtimeVersion".into(),
                    value: &runtime_version_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "jvmOptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "quota".into(),
                },
                register_interface::ResultField {
                    name: "runtimeVersion".into(),
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
        SpringCloudJavaDeploymentResult {
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            jvm_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jvmOptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quota").unwrap(),
            ),
            runtime_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeVersion").unwrap(),
            ),
            spring_cloud_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudAppId").unwrap(),
            ),
        }
    }
}
