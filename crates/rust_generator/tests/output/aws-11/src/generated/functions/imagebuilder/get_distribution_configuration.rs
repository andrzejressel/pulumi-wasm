#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_distribution_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDistributionConfigurationArgs {
        /// ARN of the distribution configuration.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the distribution configuration.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDistributionConfigurationResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the distribution configuration was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Date the distribution configuration was updated.
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description of the container distribution configuration.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Set of distributions.
        pub distributions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetDistributionConfigurationDistribution,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the distribution configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the distribution configuration.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDistributionConfigurationArgs,
    ) -> GetDistributionConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getDistributionConfiguration:getDistributionConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDistributionConfigurationResult {
            arn: o.get_field("arn"),
            date_created: o.get_field("dateCreated"),
            date_updated: o.get_field("dateUpdated"),
            description: o.get_field("description"),
            distributions: o.get_field("distributions"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
