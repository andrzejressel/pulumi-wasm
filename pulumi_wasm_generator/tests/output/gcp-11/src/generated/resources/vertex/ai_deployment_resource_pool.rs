/// 'DeploymentResourcePool can be shared by multiple deployed models,
/// whose underlying specification consists of dedicated resources.'
///
///
/// To get more information about DeploymentResourcePool, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.deploymentResourcePools)
///
/// ## Example Usage
///
/// ### Vertex Ai Deployment Resource Pool
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let deploymentResourcePool = ai_deployment_resource_pool::create(
///         "deploymentResourcePool",
///         AiDeploymentResourcePoolArgs::builder()
///             .dedicated_resources(
///                 AiDeploymentResourcePoolDedicatedResources::builder()
///                     .autoscalingMetricSpecs(
///                         vec![
///                             AiDeploymentResourcePoolDedicatedResourcesAutoscalingMetricSpec::builder()
///                             .metricName("aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle")
///                             .target(60).build_struct(),
///                         ],
///                     )
///                     .machineSpec(
///                         AiDeploymentResourcePoolDedicatedResourcesMachineSpec::builder()
///                             .acceleratorCount(1)
///                             .acceleratorType("NVIDIA_TESLA_P4")
///                             .machineType("n1-standard-4")
///                             .build_struct(),
///                     )
///                     .maxReplicaCount(2)
///                     .minReplicaCount(1)
///                     .build_struct(),
///             )
///             .name("example-deployment-resource-pool")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DeploymentResourcePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/deploymentResourcePools/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, DeploymentResourcePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiDeploymentResourcePool:AiDeploymentResourcePool default projects/{{project}}/locations/{{region}}/deploymentResourcePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiDeploymentResourcePool:AiDeploymentResourcePool default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiDeploymentResourcePool:AiDeploymentResourcePool default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiDeploymentResourcePool:AiDeploymentResourcePool default {{name}}
/// ```
///
pub mod ai_deployment_resource_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiDeploymentResourcePoolArgs {
        /// The underlying dedicated resources that the deployment resource pool uses.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dedicated_resources: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiDeploymentResourcePoolDedicatedResources,
            >,
        >,
        /// The resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are `/^a-z?$/`.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of deployment resource pool. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiDeploymentResourcePoolResult {
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The underlying dedicated resources that the deployment resource pool uses.
        /// Structure is documented below.
        pub dedicated_resources: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiDeploymentResourcePoolDedicatedResources,
            >,
        >,
        /// The resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are `/^a-z?$/`.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of deployment resource pool. eg us-central1
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AiDeploymentResourcePoolArgs,
    ) -> AiDeploymentResourcePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dedicated_resources_binding = args.dedicated_resources.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiDeploymentResourcePool:AiDeploymentResourcePool".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dedicatedResources".into(),
                    value: &dedicated_resources_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedResources".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AiDeploymentResourcePoolResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dedicated_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedResources").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
