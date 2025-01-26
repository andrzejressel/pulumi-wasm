/// Provides a Security Hub custom insight resource. See the [Managing custom insights section](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-custom-insights.html) of the AWS User Guide for more information.
///
/// ## Example Usage
///
/// ### Filter by AWS account ID
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleInsight = insight::create(
///         "exampleInsight",
///         InsightArgs::builder()
///             .filters(
///                 InsightFilters::builder()
///                     .awsAccountIds(
///                         vec![
///                             InsightFiltersAwsAccountId::builder().comparison("EQUALS")
///                             .value("1234567890").build_struct(),
///                             InsightFiltersAwsAccountId::builder().comparison("EQUALS")
///                             .value("09876543210").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .group_by_attribute("AwsAccountId")
///             .name("example-insight")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filter by date range
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleInsight = insight::create(
///         "exampleInsight",
///         InsightArgs::builder()
///             .filters(
///                 InsightFilters::builder()
///                     .createdAts(
///                         vec![
///                             InsightFiltersCreatedAt::builder()
///                             .dateRange(InsightFiltersCreatedAtDateRange::builder()
///                             .unit("DAYS").value(5).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .group_by_attribute("CreatedAt")
///             .name("example-insight")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filter by destination IPv4 address
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleInsight = insight::create(
///         "exampleInsight",
///         InsightArgs::builder()
///             .filters(
///                 InsightFilters::builder()
///                     .networkDestinationIpv4s(
///                         vec![
///                             InsightFiltersNetworkDestinationIpv4::builder()
///                             .cidr("10.0.0.0/16").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .group_by_attribute("NetworkDestinationIpV4")
///             .name("example-insight")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filter by finding's confidence
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleInsight = insight::create(
///         "exampleInsight",
///         InsightArgs::builder()
///             .filters(
///                 InsightFilters::builder()
///                     .confidences(
///                         vec![
///                             InsightFiltersConfidence::builder().gte("80").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .group_by_attribute("Confidence")
///             .name("example-insight")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filter by resource tags
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleInsight = insight::create(
///         "exampleInsight",
///         InsightArgs::builder()
///             .filters(
///                 InsightFilters::builder()
///                     .resourceTags(
///                         vec![
///                             InsightFiltersResourceTag::builder().comparison("EQUALS")
///                             .key("Environment").value("Production").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .group_by_attribute("ResourceTags")
///             .name("example-insight")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub insights using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/insight:Insight example arn:aws:securityhub:us-west-2:1234567890:insight/1234567890/custom/91299ed7-abd0-4e44-a858-d0b15e37141a
/// ```
pub mod insight {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InsightArgs {
        /// A configuration block including one or more (up to 10 distinct) attributes used to filter the findings included in the insight. The insight only includes findings that match criteria defined in the filters. See filters below for more details.
        #[builder(into)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            super::super::types::securityhub::InsightFilters,
        >,
        /// The attribute used to group the findings for the insight e.g., if an insight is grouped by `ResourceId`, then the insight produces a list of resource identifiers.
        #[builder(into)]
        pub group_by_attribute: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the custom insight.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InsightResult {
        /// ARN of the insight.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A configuration block including one or more (up to 10 distinct) attributes used to filter the findings included in the insight. The insight only includes findings that match criteria defined in the filters. See filters below for more details.
        pub filters: pulumi_wasm_rust::Output<
            super::super::types::securityhub::InsightFilters,
        >,
        /// The attribute used to group the findings for the insight e.g., if an insight is grouped by `ResourceId`, then the insight produces a list of resource identifiers.
        pub group_by_attribute: pulumi_wasm_rust::Output<String>,
        /// The name of the custom insight.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InsightArgs,
    ) -> InsightResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let group_by_attribute_binding = args
            .group_by_attribute
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/insight:Insight".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "groupByAttribute".into(),
                    value: &group_by_attribute_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "groupByAttribute".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InsightResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            group_by_attribute: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupByAttribute").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
