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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_java_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudJavaDeploymentArgs {
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the jvm option of the Spring Cloud Deployment.
        #[builder(into, default)]
        pub jvm_options: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Spring Cloud Deployment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `quota` block as defined below.
        #[builder(into, default)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudJavaDeploymentQuota>,
        >,
        /// Specifies the runtime version of the Spring Cloud Deployment. Possible Values are `Java_8`, `Java_11` and `Java_17`. Defaults to `Java_8`.
        #[builder(into, default)]
        pub runtime_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id of the Spring Cloud Application in which to create the Deployment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudJavaDeploymentResult {
        /// Specifies the environment variables of the Spring Cloud Deployment as a map of key-value pairs.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the required instance count of the Spring Cloud Deployment. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the jvm option of the Spring Cloud Deployment.
        pub jvm_options: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Deployment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `quota` block as defined below.
        pub quota: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudJavaDeploymentQuota,
        >,
        /// Specifies the runtime version of the Spring Cloud Deployment. Possible Values are `Java_8`, `Java_11` and `Java_17`. Defaults to `Java_8`.
        pub runtime_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id of the Spring Cloud Application in which to create the Deployment. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudJavaDeploymentArgs,
    ) -> SpringCloudJavaDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let jvm_options_binding = args.jvm_options.get_output(context);
        let name_binding = args.name.get_output(context);
        let quota_binding = args.quota.get_output(context);
        let runtime_version_binding = args.runtime_version.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudJavaDeployment:SpringCloudJavaDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: environment_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: instance_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jvmOptions".into(),
                    value: jvm_options_binding.get_id(),
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
                    name: "runtimeVersion".into(),
                    value: runtime_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: spring_cloud_app_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudJavaDeploymentResult {
            environment_variables: o.get_field("environmentVariables"),
            instance_count: o.get_field("instanceCount"),
            jvm_options: o.get_field("jvmOptions"),
            name: o.get_field("name"),
            quota: o.get_field("quota"),
            runtime_version: o.get_field("runtimeVersion"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
