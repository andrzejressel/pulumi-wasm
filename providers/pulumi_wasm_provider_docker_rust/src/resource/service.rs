pub struct ServiceArgs {
    pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
    pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
    pub endpoint_spec: pulumi_wasm_rust::Output<Option<crate::types::ServiceEndpointSpec>>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ServiceLabel>>>,
    pub mode: pulumi_wasm_rust::Output<Option<crate::types::ServiceMode>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
    pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
    pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
}

pub struct ServiceResult {
    pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
    pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
    pub endpoint_spec: pulumi_wasm_rust::Output<crate::types::ServiceEndpointSpec>,
    pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ServiceLabel>>,
    pub mode: pulumi_wasm_rust::Output<crate::types::ServiceMode>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
    pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
    pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
}

pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
    let result = crate::bindings::pulumi::docker::service::invoke(
        name,
        &crate::bindings::pulumi::docker::service::Args {
            auth: args.auth.get_inner(),
            converge_config: args.converge_config.get_inner(),
            endpoint_spec: args.endpoint_spec.get_inner(),
            labels: args.labels.get_inner(),
            mode: args.mode.get_inner(),
            name: args.name.get_inner(),
            rollback_config: args.rollback_config.get_inner(),
            task_spec: args.task_spec.get_inner(),
            update_config: args.update_config.get_inner(),
        },
    );

    ServiceResult {
        auth: crate::into_domain(result.auth),
        converge_config: crate::into_domain(result.converge_config),
        endpoint_spec: crate::into_domain(result.endpoint_spec),
        labels: crate::into_domain(result.labels),
        mode: crate::into_domain(result.mode),
        name: crate::into_domain(result.name),
        rollback_config: crate::into_domain(result.rollback_config),
        task_spec: crate::into_domain(result.task_spec),
        update_config: crate::into_domain(result.update_config),
    }
}
