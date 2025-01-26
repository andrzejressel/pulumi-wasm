/// Provides an Amazon AppIntegrations Data Integration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appintegrations:DataIntegration
///     properties:
///       name: example
///       description: example
///       kmsKey: ${test.arn}
///       sourceUri: Salesforce://AppFlow/example
///       scheduleConfig:
///         firstExecutionFrom: '1439788442681'
///         object: Account
///         scheduleExpression: rate(1 hour)
///       tags:
///         Key1: Value1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon AppIntegrations Data Integrations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:appintegrations/dataIntegration:DataIntegration example 12345678-1234-1234-1234-123456789123
/// ```
pub mod data_integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataIntegrationArgs {
        /// Specifies the description of the Data Integration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
        #[builder(into)]
        pub kms_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Integration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
        #[builder(into)]
        pub schedule_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::appintegrations::DataIntegrationScheduleConfig,
        >,
        /// Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
        #[builder(into)]
        pub source_uri: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataIntegrationResult {
        /// The Amazon Resource Name (ARN) of the Data Integration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the Data Integration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
        pub kms_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Integration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
        pub schedule_config: pulumi_wasm_rust::Output<
            super::super::types::appintegrations::DataIntegrationScheduleConfig,
        >,
        /// Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
        pub source_uri: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: DataIntegrationArgs,
    ) -> DataIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let kms_key_binding = args.kms_key.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let schedule_config_binding = args
            .schedule_config
            .get_output(context)
            .get_inner();
        let source_uri_binding = args.source_uri.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appintegrations/dataIntegration:DataIntegration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleConfig".into(),
                    value: &schedule_config_binding,
                },
                register_interface::ObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataIntegrationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(o.extract_field("kmsKey")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            schedule_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleConfig"),
            ),
            source_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceUri"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
