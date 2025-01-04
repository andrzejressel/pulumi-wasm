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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataIntegrationArgs {
        /// Specifies the description of the Data Integration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
        #[builder(into)]
        pub kms_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Integration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
        #[builder(into)]
        pub schedule_config: pulumi_wasm_rust::Output<
            super::super::types::appintegrations::DataIntegrationScheduleConfig,
        >,
        /// Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
        #[builder(into)]
        pub source_uri: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: DataIntegrationArgs) -> DataIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let kms_key_binding = args.kms_key.get_inner();
        let name_binding = args.name.get_inner();
        let schedule_config_binding = args.schedule_config.get_inner();
        let source_uri_binding = args.source_uri.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appintegrations/dataIntegration:DataIntegration".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scheduleConfig".into(),
                },
                register_interface::ResultField {
                    name: "sourceUri".into(),
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
        DataIntegrationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            schedule_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduleConfig").unwrap(),
            ),
            source_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceUri").unwrap(),
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
