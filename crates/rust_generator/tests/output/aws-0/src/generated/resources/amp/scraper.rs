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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScraperArgs,
    ) -> ScraperResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_output(context).get_inner();
        let destination_binding = args.destination.get_output(context).get_inner();
        let scrape_configuration_binding = args
            .scrape_configuration
            .get_output(context)
            .get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amp/scraper:Scraper".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "scrapeConfiguration".into(),
                    value: &scrape_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScraperResult {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            scrape_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scrapeConfiguration"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
