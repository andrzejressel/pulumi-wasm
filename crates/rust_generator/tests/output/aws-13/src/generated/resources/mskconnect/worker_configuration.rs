/// Provides an Amazon MSK Connect Worker Configuration Resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod worker_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerConfigurationArgs {
        /// A summary description of the worker configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the worker configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub properties_file_content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkerConfigurationResult {
        /// the Amazon Resource Name (ARN) of the worker configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A summary description of the worker configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// an ID of the latest successfully created revision of the worker configuration.
        pub latest_revision: pulumi_gestalt_rust::Output<i32>,
        /// The name of the worker configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Contents of connect-distributed.properties file. The value can be either base64 encoded or in raw format.
        ///
        /// The following arguments are optional:
        pub properties_file_content: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkerConfigurationArgs,
    ) -> WorkerConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let properties_file_content_binding = args
            .properties_file_content
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mskconnect/workerConfiguration:WorkerConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "propertiesFileContent".into(),
                    value: properties_file_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkerConfigurationResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            latest_revision: o.get_field("latestRevision"),
            name: o.get_field("name"),
            properties_file_content: o.get_field("propertiesFileContent"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
