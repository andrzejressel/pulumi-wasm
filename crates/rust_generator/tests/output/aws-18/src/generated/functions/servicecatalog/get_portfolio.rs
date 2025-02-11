#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_portfolio {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPortfolioArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags applied to the portfolio.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPortfolioResult {
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Portfolio ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time the portfolio was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the portfolio
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Portfolio name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the person or organization who owns the portfolio.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// Tags applied to the portfolio.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPortfolioArgs,
    ) -> GetPortfolioResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicecatalog/getPortfolio:getPortfolio".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPortfolioResult {
            accept_language: o.get_field("acceptLanguage"),
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            provider_name: o.get_field("providerName"),
            tags: o.get_field("tags"),
        }
    }
}
