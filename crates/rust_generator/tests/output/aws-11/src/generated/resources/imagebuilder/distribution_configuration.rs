/// Manages an Image Builder Distribution Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:imagebuilder:DistributionConfiguration
///     properties:
///       name: example
///       distributions:
///         - amiDistributionConfiguration:
///             amiTags:
///               CostCenter: IT
///             name: example-{{ imagebuilder:buildDate }}
///             launchPermission:
///               userIds:
///                 - '123456789012'
///           launchTemplateConfigurations:
///             - launchTemplateId: lt-0aaa1bcde2ff3456
///           region: us-east-1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_distribution_configurations` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/distributionConfiguration:DistributionConfiguration example arn:aws:imagebuilder:us-east-1:123456789012:distribution-configuration/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod distribution_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DistributionConfigurationArgs {
        /// Description of the distribution configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more configuration blocks with distribution settings. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub distributions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::imagebuilder::DistributionConfigurationDistribution>,
        >,
        /// Name of the distribution configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the distribution configuration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DistributionConfigurationResult {
        /// (Required) Amazon Resource Name (ARN) of the distribution configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the distribution configuration was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Date the distribution configuration was updated.
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description of the distribution configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more configuration blocks with distribution settings. Detailed below.
        ///
        /// The following arguments are optional:
        pub distributions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::DistributionConfigurationDistribution>,
        >,
        /// Name of the distribution configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the distribution configuration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: DistributionConfigurationArgs,
    ) -> DistributionConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let distributions_binding = args.distributions.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/distributionConfiguration:DistributionConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distributions".into(),
                    value: distributions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DistributionConfigurationResult {
            arn: o.get_field("arn"),
            date_created: o.get_field("dateCreated"),
            date_updated: o.get_field("dateUpdated"),
            description: o.get_field("description"),
            distributions: o.get_field("distributions"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
