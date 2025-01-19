/// Provides an MQ Configuration Resource.
///
/// For more information on Amazon MQ, see [Amazon MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/welcome.html).
///
/// ## Example Usage
///
/// ### ActiveMQ
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration::create(
///         "example",
///         ConfigurationArgs::builder()
///             .data(
///                 "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<broker xmlns=\"http://activemq.apache.org/schema/core\">\n  <plugins>\n    <forcePersistencyModeBrokerPlugin persistenceFlag=\"true\"/>\n    <statisticsBrokerPlugin/>\n    <timeStampingBrokerPlugin ttlCeiling=\"86400000\" zeroExpirationOverride=\"86400000\"/>\n  </plugins>\n</broker>",
///             )
///             .description("Example Configuration")
///             .engine_type("ActiveMQ")
///             .engine_version("5.17.6")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RabbitMQ
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration::create(
///         "example",
///         ConfigurationArgs::builder()
///             .data(
///                 "# Default RabbitMQ delivery acknowledgement timeout is 30 minutes in milliseconds\nconsumer_timeout = 1800000",
///             )
///             .description("Example Configuration")
///             .engine_type("RabbitMQ")
///             .engine_version("3.11.20")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MQ Configurations using the configuration ID. For example:
///
/// ```sh
/// $ pulumi import aws:mq/configuration:Configuration example c-0187d1eb-88c8-475a-9b79-16ef5a10c94f
/// ```
pub mod configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// Authentication strategy associated with the configuration. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
        #[builder(into, default)]
        pub authentication_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Broker configuration in XML format for `ActiveMQ` or [Cuttlefish](https://github.com/Kyorai/cuttlefish) format for `RabbitMQ`. See [official docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-broker-configuration-parameters.html) for supported parameters and format of the XML.
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
        #[builder(into)]
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// Version of the broker engine.
        #[builder(into)]
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Name of the configuration.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// ARN of the configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Authentication strategy associated with the configuration. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
        pub authentication_strategy: pulumi_wasm_rust::Output<String>,
        /// Broker configuration in XML format for `ActiveMQ` or [Cuttlefish](https://github.com/Kyorai/cuttlefish) format for `RabbitMQ`. See [official docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/amazon-mq-broker-configuration-parameters.html) for supported parameters and format of the XML.
        pub data: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// Version of the broker engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Latest revision of the configuration.
        pub latest_revision: pulumi_wasm_rust::Output<i32>,
        /// Name of the configuration.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationArgs) -> ConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_strategy_binding = args.authentication_strategy.get_inner();
        let data_binding = args.data.get_inner();
        let description_binding = args.description.get_inner();
        let engine_type_binding = args.engine_type.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mq/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationStrategy".into(),
                    value: &authentication_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "authenticationStrategy".into(),
                },
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "latestRevision".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        ConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationStrategy").unwrap(),
            ),
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            latest_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestRevision").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
