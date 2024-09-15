//! <!-- Bug: Type and Name are switched -->
//! This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
//!  With the Converge Config Name of the service
//! - `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)
//!
//! ## Import
//!
//! ### Example
//!
//! Assuming you created a `service` as follows
//!
//! #!/bin/bash
//!
//! docker service create --name foo -p 8080:80 nginx
//!
//! prints th ID
//!
//! 4pcphbxkfn2rffhbhe6czytgi
//!
//! you provide the definition for the resource as follows
//!
//! terraform
//!
//! resource "docker_service" "foo" {
//!
//!   name = "foo"
//!
//!   task_spec {
//!
//!     container_spec {
//!
//!       image = "nginx"
//!
//!     }
//!
//!   }
//!
//!   endpoint_spec {
//!
//!     ports {
//!
//!       target_port    = "80"
//!
//!       published_port = "8080"
//!
//!     }
//!
//!   }
//!
//! }
//!
//! then the import command is as follows
//!
//! #!/bin/bash
//!
//! ```sh
//! $ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ServiceArgs {
    /// Configuration for the authentication for pulling the images of the service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
    /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
    /// Properties that can be configured to access and load balance a service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub endpoint_spec: pulumi_wasm_rust::Output<Option<crate::types::ServiceEndpointSpec>>,
    /// User-defined key/value metadata
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ServiceLabel>>>,
    /// Scheduling mode for the service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub mode: pulumi_wasm_rust::Output<Option<crate::types::ServiceMode>>,
    /// Name of the service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Specification for the rollback strategy of the service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
    /// User modifiable task configuration
    #[builder(into)]
    pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
    /// Specification for the update strategy of the service
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
}

pub struct ServiceResult {
    /// Configuration for the authentication for pulling the images of the service
    pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
    /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
    pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
    /// Properties that can be configured to access and load balance a service
    pub endpoint_spec: pulumi_wasm_rust::Output<crate::types::ServiceEndpointSpec>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ServiceLabel>>,
    /// Scheduling mode for the service
    pub mode: pulumi_wasm_rust::Output<crate::types::ServiceMode>,
    /// Name of the service
    pub name: pulumi_wasm_rust::Output<String>,
    /// Specification for the rollback strategy of the service
    pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
    /// User modifiable task configuration
    pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
    /// Specification for the update strategy of the service
    pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
    let result = crate::bindings::pulumi::docker::service::invoke(
        name,
        &crate::bindings::pulumi::docker::service::Args {
            auth: &args.auth.get_inner(),
            converge_config: &args.converge_config.get_inner(),
            endpoint_spec: &args.endpoint_spec.get_inner(),
            labels: &args.labels.get_inner(),
            mode: &args.mode.get_inner(),
            name: &args.name.get_inner(),
            rollback_config: &args.rollback_config.get_inner(),
            task_spec: &args.task_spec.get_inner(),
            update_config: &args.update_config.get_inner(),
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
