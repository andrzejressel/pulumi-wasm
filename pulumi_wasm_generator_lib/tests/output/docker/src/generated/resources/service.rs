#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceArgs {
    /// Configuration for the authentication for pulling the images of the service
    #[builder(into, default)]
    pub auth: pulumi_wasm_rust::Output<Option<super::types::ServiceAuth>>,
    /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
    #[builder(into, default)]
    pub converge_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceConvergeConfig>,
    >,
    /// Properties that can be configured to access and load balance a service
    #[builder(into, default)]
    pub endpoint_spec: pulumi_wasm_rust::Output<
        Option<super::types::ServiceEndpointSpec>,
    >,
    /// User-defined key/value metadata
    #[builder(into, default)]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::ServiceLabel>>>,
    /// Scheduling mode for the service
    #[builder(into, default)]
    pub mode: pulumi_wasm_rust::Output<Option<super::types::ServiceMode>>,
    /// Name of the service
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Specification for the rollback strategy of the service
    #[builder(into, default)]
    pub rollback_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceRollbackConfig>,
    >,
    /// User modifiable task configuration
    #[builder(into)]
    pub task_spec: pulumi_wasm_rust::Output<super::types::ServiceTaskSpec>,
    /// Specification for the update strategy of the service
    #[builder(into, default)]
    pub update_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceUpdateConfig>,
    >,
}
pub struct ServiceResult {
    /// Configuration for the authentication for pulling the images of the service
    pub auth: pulumi_wasm_rust::Output<Option<super::types::ServiceAuth>>,
    /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
    pub converge_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceConvergeConfig>,
    >,
    /// Properties that can be configured to access and load balance a service
    pub endpoint_spec: pulumi_wasm_rust::Output<super::types::ServiceEndpointSpec>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Vec<super::types::ServiceLabel>>,
    /// Scheduling mode for the service
    pub mode: pulumi_wasm_rust::Output<super::types::ServiceMode>,
    /// Name of the service
    pub name: pulumi_wasm_rust::Output<String>,
    /// Specification for the rollback strategy of the service
    pub rollback_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceRollbackConfig>,
    >,
    /// User modifiable task configuration
    pub task_spec: pulumi_wasm_rust::Output<super::types::ServiceTaskSpec>,
    /// Specification for the update strategy of the service
    pub update_config: pulumi_wasm_rust::Output<
        Option<super::types::ServiceUpdateConfig>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let auth_binding = args.auth.get_inner();
    let converge_config_binding = args.converge_config.get_inner();
    let endpoint_spec_binding = args.endpoint_spec.get_inner();
    let labels_binding = args.labels.get_inner();
    let mode_binding = args.mode.get_inner();
    let name_binding = args.name.get_inner();
    let rollback_config_binding = args.rollback_config.get_inner();
    let task_spec_binding = args.task_spec.get_inner();
    let update_config_binding = args.update_config.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/service:Service".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "auth".into(),
                value: &auth_binding,
            },
            register_interface::ObjectField {
                name: "convergeConfig".into(),
                value: &converge_config_binding,
            },
            register_interface::ObjectField {
                name: "endpointSpec".into(),
                value: &endpoint_spec_binding,
            },
            register_interface::ObjectField {
                name: "labels".into(),
                value: &labels_binding,
            },
            register_interface::ObjectField {
                name: "mode".into(),
                value: &mode_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "rollbackConfig".into(),
                value: &rollback_config_binding,
            },
            register_interface::ObjectField {
                name: "taskSpec".into(),
                value: &task_spec_binding,
            },
            register_interface::ObjectField {
                name: "updateConfig".into(),
                value: &update_config_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "auth".into() },
            register_interface::ResultField { name : "convergeConfig".into() },
            register_interface::ResultField { name : "endpointSpec".into() },
            register_interface::ResultField { name : "labels".into() },
            register_interface::ResultField { name : "mode".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "rollbackConfig".into() },
            register_interface::ResultField { name : "taskSpec".into() },
            register_interface::ResultField { name : "updateConfig".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ServiceResult {
        auth: into_domain(hashmap.remove("auth").unwrap()),
        converge_config: into_domain(hashmap.remove("convergeConfig").unwrap()),
        endpoint_spec: into_domain(hashmap.remove("endpointSpec").unwrap()),
        labels: into_domain(hashmap.remove("labels").unwrap()),
        mode: into_domain(hashmap.remove("mode").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        rollback_config: into_domain(hashmap.remove("rollbackConfig").unwrap()),
        task_spec: into_domain(hashmap.remove("taskSpec").unwrap()),
        update_config: into_domain(hashmap.remove("updateConfig").unwrap()),
    }
}
