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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod container_service_deployment_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerServiceDeploymentVersionArgs {
        /// A set of configuration blocks that describe the settings of the containers that will be launched on the container service. Maximum of 53. Detailed below.
        #[builder(into)]
        pub containers: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::lightsail::ContainerServiceDeploymentVersionContainer,
            >,
        >,
        /// A configuration block that describes the settings of the public endpoint for the container service. Detailed below.
        #[builder(into, default)]
        pub public_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::lightsail::ContainerServiceDeploymentVersionPublicEndpoint,
            >,
        >,
        /// The name for the container service.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ContainerServiceDeploymentVersionResult {
        /// A set of configuration blocks that describe the settings of the containers that will be launched on the container service. Maximum of 53. Detailed below.
        pub containers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::lightsail::ContainerServiceDeploymentVersionContainer,
            >,
        >,
        /// The timestamp when the deployment was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that describes the settings of the public endpoint for the container service. Detailed below.
        pub public_endpoint: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::lightsail::ContainerServiceDeploymentVersionPublicEndpoint,
            >,
        >,
        /// The name for the container service.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The current state of the container service.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The version number of the deployment.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerServiceDeploymentVersionArgs,
    ) -> ContainerServiceDeploymentVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let containers_binding = args.containers.get_output(context);
        let public_endpoint_binding = args.public_endpoint.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/containerServiceDeploymentVersion:ContainerServiceDeploymentVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containers".into(),
                    value: containers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicEndpoint".into(),
                    value: public_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContainerServiceDeploymentVersionResult {
            containers: o.get_field("containers"),
            created_at: o.get_field("createdAt"),
            public_endpoint: o.get_field("publicEndpoint"),
            service_name: o.get_field("serviceName"),
            state: o.get_field("state"),
            version: o.get_field("version"),
        }
    }
}
