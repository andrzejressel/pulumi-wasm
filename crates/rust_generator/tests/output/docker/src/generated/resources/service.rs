/// <!-- Bug: Type and Name are switched -->
/// This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
/// With the Converge Config Name of the service
/// - `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `service` as follows
///
/// ```shell
/// docker service create --name foo -p 8080:80 nginx
/// ```
///
/// prints this ID
///
/// ```text
/// 4pcphbxkfn2rffhbhe6czytgi
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = service::create(
///         "foo",
///         ServiceArgs::builder()
///             .endpoint_spec(
///                 ServiceEndpointSpec::builder()
///                     .ports(
///                         vec![
///                             ServiceEndpointSpecPort::builder().publishedPort(8080)
///                             .targetPort(80).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .task_spec(
///                 ServiceTaskSpec::builder()
///                     .containerSpec(
///                         ServiceTaskSpecContainerSpec::builder()
///                             .image("nginx")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Configuration for the authentication for pulling the images of the service
        #[builder(into, default)]
        pub auth: pulumi_gestalt_rust::InputOrOutput<Option<super::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        #[builder(into, default)]
        pub converge_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ServiceConvergeConfig>,
        >,
        /// Properties that can be configured to access and load balance a service
        #[builder(into, default)]
        pub endpoint_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ServiceEndpointSpec>,
        >,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ServiceLabel>>,
        >,
        /// Scheduling mode for the service
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<super::types::ServiceMode>>,
        /// Name of the service
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specification for the rollback strategy of the service
        #[builder(into, default)]
        pub rollback_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ServiceRollbackConfig>,
        >,
        /// User modifiable task configuration
        #[builder(into)]
        pub task_spec: pulumi_gestalt_rust::InputOrOutput<super::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        #[builder(into, default)]
        pub update_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ServiceUpdateConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Configuration for the authentication for pulling the images of the service
        pub auth: pulumi_gestalt_rust::Output<Option<super::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        pub converge_config: pulumi_gestalt_rust::Output<
            Option<super::types::ServiceConvergeConfig>,
        >,
        /// Properties that can be configured to access and load balance a service
        pub endpoint_spec: pulumi_gestalt_rust::Output<
            super::types::ServiceEndpointSpec,
        >,
        /// User-defined key/value metadata
        pub labels: pulumi_gestalt_rust::Output<Vec<super::types::ServiceLabel>>,
        /// Scheduling mode for the service
        pub mode: pulumi_gestalt_rust::Output<super::types::ServiceMode>,
        /// Name of the service
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specification for the rollback strategy of the service
        pub rollback_config: pulumi_gestalt_rust::Output<
            Option<super::types::ServiceRollbackConfig>,
        >,
        /// User modifiable task configuration
        pub task_spec: pulumi_gestalt_rust::Output<super::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        pub update_config: pulumi_gestalt_rust::Output<
            Option<super::types::ServiceUpdateConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_binding = args.auth.get_output(context);
        let converge_config_binding = args.converge_config.get_output(context);
        let endpoint_spec_binding = args.endpoint_spec.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let rollback_config_binding = args.rollback_config.get_output(context);
        let task_spec_binding = args.task_spec.get_output(context);
        let update_config_binding = args.update_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auth".into(),
                    value: auth_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "convergeConfig".into(),
                    value: converge_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointSpec".into(),
                    value: endpoint_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rollbackConfig".into(),
                    value: rollback_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskSpec".into(),
                    value: task_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateConfig".into(),
                    value: update_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            auth: o.get_field("auth"),
            converge_config: o.get_field("convergeConfig"),
            endpoint_spec: o.get_field("endpointSpec"),
            labels: o.get_field("labels"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            rollback_config: o.get_field("rollbackConfig"),
            task_spec: o.get_field("taskSpec"),
            update_config: o.get_field("updateConfig"),
        }
    }
}
