/// Provides an ECS cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = cluster::create(
///         "foo",
///         ClusterArgs::builder()
///             .name("white-hart")
///             .settings(
///                 vec![
///                     ClusterSetting::builder().name("containerInsights").value("enabled")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Execute Command Configuration with Override Logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("example")
///             .build_struct(),
///     );
///     let exampleLogGroup = log_group::create(
///         "exampleLogGroup",
///         LogGroupArgs::builder().name("example").build_struct(),
///     );
///     let test = cluster::create(
///         "test",
///         ClusterArgs::builder()
///             .configuration(
///                 ClusterConfiguration::builder()
///                     .executeCommandConfiguration(
///                         ClusterConfigurationExecuteCommandConfiguration::builder()
///                             .kmsKeyId("${example.arn}")
///                             .logConfiguration(
///                                 ClusterConfigurationExecuteCommandConfigurationLogConfiguration::builder()
///                                     .cloudWatchEncryptionEnabled(true)
///                                     .cloudWatchLogGroupName("${exampleLogGroup.name}")
///                                     .build_struct(),
///                             )
///                             .logging("OVERRIDE")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Fargate Ephemeral Storage Encryption with Customer-Managed KMS Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: example
///       deletionWindowInDays: 7
///   exampleKeyPolicy:
///     type: aws:kms:KeyPolicy
///     name: example
///     properties:
///       keyId: ${example.id}
///       policy:
///         fn::toJSON:
///           Id: ECSClusterFargatePolicy
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: '*'
///               Action: kms:*
///               Resource: '*'
///             - Sid: Allow generate data key access for Fargate tasks.
///               Effect: Allow
///               Principal:
///                 Service: fargate.amazonaws.com
///               Action:
///                 - kms:GenerateDataKeyWithoutPlaintext
///               Condition:
///                 StringEquals:
///                   kms:EncryptionContext:aws:ecs:clusterAccount:
///                     - ${current.accountId}
///                   kms:EncryptionContext:aws:ecs:clusterName:
///                     - example
///               Resource: '*'
///             - Sid: Allow grant creation permission for Fargate tasks.
///               Effect: Allow
///               Principal:
///                 Service: fargate.amazonaws.com
///               Action:
///                 - kms:CreateGrant
///               Condition:
///                 StringEquals:
///                   kms:EncryptionContext:aws:ecs:clusterAccount:
///                     - ${current.accountId}
///                   kms:EncryptionContext:aws:ecs:clusterName:
///                     - example
///                 ForAllValues:StringEquals:
///                   kms:GrantOperations:
///                     - Decrypt
///               Resource: '*'
///           Version: 2012-10-17
///   test:
///     type: aws:ecs:Cluster
///     properties:
///       name: example
///       configuration:
///         managedStorageConfiguration:
///           fargateEphemeralStorageKmsKeyId: ${example.id}
///     options:
///       dependson:
///         - ${exampleKeyPolicy}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS clusters using the cluster name. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/cluster:Cluster stateless stateless-app
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Execute command configuration for the cluster. See `configuration` Block for details.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ClusterConfiguration>,
        >,
        /// Name of the cluster (up to 255 letters, numbers, hyphens, and underscores)
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Default Service Connect namespace. See `service_connect_defaults` Block for details.
        #[builder(into, default)]
        pub service_connect_defaults: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ClusterServiceConnectDefaults>,
        >,
        /// Configuration block(s) with cluster settings. For example, this can be used to enable CloudWatch Container Insights for a cluster. See `setting` Block for details.
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ClusterSetting>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// ARN that identifies the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Execute command configuration for the cluster. See `configuration` Block for details.
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ClusterConfiguration>,
        >,
        /// Name of the cluster (up to 255 letters, numbers, hyphens, and underscores)
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Default Service Connect namespace. See `service_connect_defaults` Block for details.
        pub service_connect_defaults: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ClusterServiceConnectDefaults>,
        >,
        /// Configuration block(s) with cluster settings. For example, this can be used to enable CloudWatch Container Insights for a cluster. See `setting` Block for details.
        pub settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::ecs::ClusterSetting>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let name_binding = args.name.get_inner();
        let service_connect_defaults_binding = args.service_connect_defaults.get_inner();
        let settings_binding = args.settings.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceConnectDefaults".into(),
                    value: &service_connect_defaults_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serviceConnectDefaults".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            service_connect_defaults: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceConnectDefaults").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}