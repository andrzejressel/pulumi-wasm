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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataIntegrationArgs {
        /// Specifies the description of the Data Integration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
        #[builder(into)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Integration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
        #[builder(into)]
        pub schedule_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appintegrations::DataIntegrationScheduleConfig,
        >,
        /// Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
        #[builder(into)]
        pub source_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataIntegrationResult {
        /// The Amazon Resource Name (ARN) of the Data Integration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the Data Integration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
        pub kms_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Integration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
        pub schedule_config: pulumi_gestalt_rust::Output<
            super::super::types::appintegrations::DataIntegrationScheduleConfig,
        >,
        /// Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
        pub source_uri: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: DataIntegrationArgs,
    ) -> DataIntegrationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let kms_key_binding = args.kms_key.get_output(context);
        let name_binding = args.name.get_output(context);
        let schedule_config_binding = args.schedule_config.get_output(context);
        let source_uri_binding = args.source_uri.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appintegrations/dataIntegration:DataIntegration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: kms_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleConfig".into(),
                    value: schedule_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceUri".into(),
                    value: source_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataIntegrationResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            kms_key: o.get_field("kmsKey"),
            name: o.get_field("name"),
            schedule_config: o.get_field("scheduleConfig"),
            source_uri: o.get_field("sourceUri"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
