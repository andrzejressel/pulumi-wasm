/// Manages an Active Azure Spring Cloud Deployment.
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
///       runtimeVersion: Java_11
///       quota:
///         cpu: '2'
///         memory: 4Gi
///       environmentVariables:
///         Env: Staging
///   exampleSpringCloudActiveDeployment:
///     type: azure:appplatform:SpringCloudActiveDeployment
///     name: example
///     properties:
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       deploymentName: ${exampleSpringCloudJavaDeployment.name}
/// ```
///
/// ## Import
///
/// Spring Cloud Active Deployment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudActiveDeployment:SpringCloudActiveDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/service1/apps/app1
/// ```
///
pub mod spring_cloud_active_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentArgs {
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        #[builder(into)]
        pub deployment_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentResult {
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        pub deployment_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudActiveDeploymentArgs,
    ) -> SpringCloudActiveDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deployment_name_binding = args.deployment_name.get_inner();
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudActiveDeployment:SpringCloudActiveDeployment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deploymentName".into(),
                    value: &deployment_name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deploymentName".into(),
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
        SpringCloudActiveDeploymentResult {
            deployment_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentName").unwrap(),
            ),
            spring_cloud_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudAppId").unwrap(),
            ),
        }
    }
}