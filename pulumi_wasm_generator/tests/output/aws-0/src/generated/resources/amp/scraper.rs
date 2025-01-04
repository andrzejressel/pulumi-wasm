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
pub mod scraper {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScraperArgs {
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        #[builder(into, default)]
        pub destination: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        #[builder(into)]
        pub scrape_configuration: pulumi_wasm_rust::Output<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub source: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperSource>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScraperResult {
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the new scraper.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        pub destination: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the scraper to discover, collect, and produce metrics
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        pub scrape_configuration: pulumi_wasm_rust::Output<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        pub source: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperSource>,
        >,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScraperArgs) -> ScraperResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let destination_binding = args.destination.get_inner();
        let scrape_configuration_binding = args.scrape_configuration.get_inner();
        let source_binding = args.source.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amp/scraper:Scraper".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "scrapeConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScraperResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            scrape_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scrapeConfiguration").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
