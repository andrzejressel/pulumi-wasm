/// Provides an Amazon MSK Connect Worker Configuration Resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = worker_configuration::create(
///         "example",
///         WorkerConfigurationArgs::builder()
///             .name("example")
///             .properties_file_content(
///                 "key.converter=org.apache.kafka.connect.storage.StringConverter\nvalue.converter=org.apache.kafka.connect.storage.StringConverter",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK Connect Worker Configuration using the plugin's `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:mskconnect/workerConfiguration:WorkerConfiguration example 'arn:aws:kafkaconnect:eu-central-1:123456789012:worker-configuration/example/8848493b-7fcc-478c-a646-4a52634e3378-4'
/// ```
pub mod worker_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerConfigurationArgs {
        /// A summary description of the worker configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the worker configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub properties_file_content: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkerConfigurationResult {
        /// the Amazon Resource Name (ARN) of the worker configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A summary description of the worker configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// an ID of the latest successfully created revision of the worker configuration.
        pub latest_revision: pulumi_wasm_rust::Output<i32>,
        /// The name of the worker configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.
        ///
        /// The following arguments are optional:
        pub properties_file_content: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkerConfigurationArgs,
    ) -> WorkerConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let properties_file_content_binding = args
            .properties_file_content
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mskconnect/workerConfiguration:WorkerConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "propertiesFileContent".into(),
                    value: &properties_file_content_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkerConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            latest_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestRevision"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            properties_file_content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("propertiesFileContent"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
