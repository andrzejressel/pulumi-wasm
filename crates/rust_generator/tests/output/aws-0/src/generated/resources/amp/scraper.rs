///
///
/// ## Import
///
/// Using `pulumi import`, import the Managed Scraper using its identifier.
/// For example:
///
/// ```sh
/// $ pulumi import aws:amp/scraper:Scraper example s-0123abc-0000-0123-a000-000000000000
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scraper {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScraperArgs {
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        #[builder(into, default)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        #[builder(into)]
        pub scrape_configuration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::amp::ScraperSource>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScraperResult {
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the new scraper.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        pub destination: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the scraper to discover, collect, and produce metrics
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        pub scrape_configuration: pulumi_gestalt_rust::Output<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        pub source: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperSource>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScraperArgs,
    ) -> ScraperResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let scrape_configuration_binding = args.scrape_configuration.get_output(context);
        let source_binding = args.source.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amp/scraper:Scraper".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scrapeConfiguration".into(),
                    value: scrape_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScraperResult {
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            destination: o.get_field("destination"),
            role_arn: o.get_field("roleArn"),
            scrape_configuration: o.get_field("scrapeConfiguration"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
