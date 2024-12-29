/// Provides an Elastic Container Registry Replication Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = get_regions::invoke(GetRegionsArgs::builder().build_struct());
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .replication_configuration(
///                 ReplicationConfigurationReplicationConfiguration::builder()
///                     .rules(
///                         vec![
///                             ReplicationConfigurationReplicationConfigurationRule::builder()
///                             .destinations(vec![ReplicationConfigurationReplicationConfigurationRuleDestination::builder()
///                             .region("${example.names[0]}")
///                             .registryId("${current.accountId}").build_struct(),])
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Multiple Region Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = get_regions::invoke(GetRegionsArgs::builder().build_struct());
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .replication_configuration(
///                 ReplicationConfigurationReplicationConfiguration::builder()
///                     .rules(
///                         vec![
///                             ReplicationConfigurationReplicationConfigurationRule::builder()
///                             .destinations(vec![ReplicationConfigurationReplicationConfigurationRuleDestination::builder()
///                             .region("${example.names[0]}")
///                             .registryId("${current.accountId}").build_struct(),
///                             ReplicationConfigurationReplicationConfigurationRuleDestination::builder()
///                             .region("${example.names[1]}")
///                             .registryId("${current.accountId}").build_struct(),])
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Repository Filter Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = get_regions::invoke(GetRegionsArgs::builder().build_struct());
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .replication_configuration(
///                 ReplicationConfigurationReplicationConfiguration::builder()
///                     .rules(
///                         vec![
///                             ReplicationConfigurationReplicationConfigurationRule::builder()
///                             .destinations(vec![ReplicationConfigurationReplicationConfigurationRuleDestination::builder()
///                             .region("${example.names[0]}")
///                             .registryId("${current.accountId}").build_struct(),])
///                             .repositoryFilters(vec![ReplicationConfigurationReplicationConfigurationRuleRepositoryFilter::builder()
///                             .filter("prod-microservice").filterType("PREFIX_MATCH")
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Replication Configuration using the `registry_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/replicationConfiguration:ReplicationConfiguration service 012345678912
/// ```
pub mod replication_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigurationArgs {
        /// Replication configuration for a registry. See Replication Configuration.
        #[builder(into, default)]
        pub replication_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ecr::ReplicationConfigurationReplicationConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigurationResult {
        /// The registry ID where the replication configuration was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Replication configuration for a registry. See Replication Configuration.
        pub replication_configuration: pulumi_wasm_rust::Output<
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
        name: &str,
        args: ReplicationConfigurationArgs,
    ) -> ReplicationConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_configuration_binding = args
            .replication_configuration
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/replicationConfiguration:ReplicationConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationConfiguration".into(),
                    value: &replication_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "replicationConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationConfigurationResult {
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            replication_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationConfiguration").unwrap(),
            ),
        }
    }
}
