/// Manages an Amazon Managed Streaming for Kafka configuration. More information can be found on the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration::create(
///         "example",
///         ConfigurationArgs::builder()
///             .kafka_versions(vec!["2.1.0",])
///             .name("example")
///             .server_properties(
///                 "auto.create.topics.enable = true\ndelete.topic.enable = true",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK configurations using the configuration ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/configuration:Configuration example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
pub mod configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// Description of the configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of Apache Kafka versions which can use this configuration.
        #[builder(into, default)]
        pub kafka_versions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
        #[builder(into)]
        pub server_properties: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// Amazon Resource Name (ARN) of the configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of Apache Kafka versions which can use this configuration.
        pub kafka_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Latest revision of the configuration.
        pub latest_revision: pulumi_wasm_rust::Output<i32>,
        /// Name of the configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
        pub server_properties: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConfigurationArgs,
    ) -> ConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let kafka_versions_binding = args.kafka_versions.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let server_properties_binding = args
            .server_properties
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaVersions".into(),
                    value: &kafka_versions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverProperties".into(),
                    value: &server_properties_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kafka_versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kafkaVersions"),
            ),
            latest_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestRevision"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            server_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverProperties"),
            ),
        }
    }
}
