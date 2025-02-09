/// Provides a SageMaker Hub resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hub::create(
///         "example",
///         HubArgs::builder().hub_description("example").hub_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Hubs using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/hub:Hub test_hub my-code-repo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HubArgs {
        /// A description of the hub.
        #[builder(into)]
        pub hub_description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name of the hub.
        #[builder(into, default)]
        pub hub_display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the hub.
        #[builder(into)]
        pub hub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The searchable keywords for the hub.
        #[builder(into, default)]
        pub hub_search_keywords: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Amazon S3 storage configuration for the hub. See S3 Storage Config details below.
        #[builder(into, default)]
        pub s3_storage_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::HubS3StorageConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HubResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Hub.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the hub.
        pub hub_description: pulumi_gestalt_rust::Output<String>,
        /// The display name of the hub.
        pub hub_display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the hub.
        pub hub_name: pulumi_gestalt_rust::Output<String>,
        /// The searchable keywords for the hub.
        pub hub_search_keywords: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Amazon S3 storage configuration for the hub. See S3 Storage Config details below.
        pub s3_storage_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::HubS3StorageConfig>,
        >,
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
        args: HubArgs,
    ) -> HubResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hub_description_binding = args.hub_description.get_output(context);
        let hub_display_name_binding = args.hub_display_name.get_output(context);
        let hub_name_binding = args.hub_name.get_output(context);
        let hub_search_keywords_binding = args.hub_search_keywords.get_output(context);
        let s3_storage_config_binding = args.s3_storage_config.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/hub:Hub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hubDescription".into(),
                    value: hub_description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hubDisplayName".into(),
                    value: hub_display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hubName".into(),
                    value: hub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hubSearchKeywords".into(),
                    value: hub_search_keywords_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3StorageConfig".into(),
                    value: s3_storage_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HubResult {
            arn: o.get_field("arn"),
            hub_description: o.get_field("hubDescription"),
            hub_display_name: o.get_field("hubDisplayName"),
            hub_name: o.get_field("hubName"),
            hub_search_keywords: o.get_field("hubSearchKeywords"),
            s3_storage_config: o.get_field("s3StorageConfig"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
