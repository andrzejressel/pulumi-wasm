/// Provides an Elastic Container Registry Replication Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleReplicationConfiguration:
///     type: aws:ecr:ReplicationConfiguration
///     name: example
///     properties:
///       replicationConfiguration:
///         rules:
///           - destinations:
///               - region: ${example.names[0]}
///                 registryId: ${current.accountId}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:getRegions
///       arguments: {}
/// ```
///
/// ## Multiple Region Usage
///
/// ```yaml
/// resources:
///   exampleReplicationConfiguration:
///     type: aws:ecr:ReplicationConfiguration
///     name: example
///     properties:
///       replicationConfiguration:
///         rules:
///           - destinations:
///               - region: ${example.names[0]}
///                 registryId: ${current.accountId}
///               - region: ${example.names[1]}
///                 registryId: ${current.accountId}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:getRegions
///       arguments: {}
/// ```
///
/// ## Repository Filter Usage
///
/// ```yaml
/// resources:
///   exampleReplicationConfiguration:
///     type: aws:ecr:ReplicationConfiguration
///     name: example
///     properties:
///       replicationConfiguration:
///         rules:
///           - destinations:
///               - region: ${example.names[0]}
///                 registryId: ${current.accountId}
///             repositoryFilters:
///               - filter: prod-microservice
///                 filterType: PREFIX_MATCH
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:getRegions
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Replication Configuration using the `registry_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/replicationConfiguration:ReplicationConfiguration service 012345678912
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigurationArgs {
        /// Replication configuration for a registry. See Replication Configuration.
        #[builder(into, default)]
        pub replication_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ecr::ReplicationConfigurationReplicationConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigurationResult {
        /// The registry ID where the replication configuration was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// Replication configuration for a registry. See Replication Configuration.
        pub replication_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ecr::ReplicationConfigurationReplicationConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ReplicationConfigurationArgs,
    ) -> ReplicationConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let replication_configuration_binding_1 = args
            .replication_configuration
            .get_output(context);
        let replication_configuration_binding = replication_configuration_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/replicationConfiguration:ReplicationConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationConfiguration".into(),
                    value: &replication_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReplicationConfigurationResult {
            registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            replication_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationConfiguration"),
            ),
        }
    }
}
