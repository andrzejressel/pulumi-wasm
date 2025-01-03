/// Provides a resource to manage a deployment version for your Amazon Lightsail container service.
///
/// > **NOTE:** The Amazon Lightsail container service must be enabled to create a deployment.
///
/// > **NOTE:** This resource allows you to manage an Amazon Lightsail container service deployment version but the provider cannot destroy it. Removing this resource from your configuration will remove it from your statefile.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lightsail:ContainerServiceDeploymentVersion
///     properties:
///       containers:
///         - containerName: hello-world
///           image: amazon/amazon-lightsail:hello-world
///           commands: []
///           environment:
///             MY_ENVIRONMENT_VARIABLE: my_value
///           ports:
///             '80': HTTP
///       publicEndpoint:
///         containerName: hello-world
///         containerPort: 80
///         healthCheck:
///           healthyThreshold: 2
///           unhealthyThreshold: 2
///           timeoutSeconds: 2
///           intervalSeconds: 5
///           path: /
///           successCodes: 200-499
///       serviceName: ${exampleAwsLightsailContainerService.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Container Service Deployment Version using the `service_name` and `version` separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/containerServiceDeploymentVersion:ContainerServiceDeploymentVersion example container-service-1/1
/// ```
pub mod container_service_deployment_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerServiceDeploymentVersionArgs {
        /// A set of configuration blocks that describe the settings of the containers that will be launched on the container service. Maximum of 53. Detailed below.
        #[builder(into)]
        pub containers: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::lightsail::ContainerServiceDeploymentVersionContainer,
            >,
        >,
        /// A configuration block that describes the settings of the public endpoint for the container service. Detailed below.
        #[builder(into, default)]
        pub public_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lightsail::ContainerServiceDeploymentVersionPublicEndpoint,
            >,
        >,
        /// The name for the container service.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContainerServiceDeploymentVersionResult {
        /// A set of configuration blocks that describe the settings of the containers that will be launched on the container service. Maximum of 53. Detailed below.
        pub containers: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::lightsail::ContainerServiceDeploymentVersionContainer,
            >,
        >,
        /// The timestamp when the deployment was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A configuration block that describes the settings of the public endpoint for the container service. Detailed below.
        pub public_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lightsail::ContainerServiceDeploymentVersionPublicEndpoint,
            >,
        >,
        /// The name for the container service.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The current state of the container service.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The version number of the deployment.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ContainerServiceDeploymentVersionArgs,
    ) -> ContainerServiceDeploymentVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let containers_binding = args.containers.get_inner();
        let public_endpoint_binding = args.public_endpoint.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/containerServiceDeploymentVersion:ContainerServiceDeploymentVersion"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containers".into(),
                    value: &containers_binding,
                },
                register_interface::ObjectField {
                    name: "publicEndpoint".into(),
                    value: &public_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containers".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "publicEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerServiceDeploymentVersionResult {
            containers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containers").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            public_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicEndpoint").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
