/// Manages a Security Hub finding aggregator. Security Hub needs to be enabled in a region in order for the aggregator to pull through findings.
///
/// ## Example Usage
///
/// ### All Regions Usage
///
/// The following example will enable the aggregator for every region.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleFindingAggregator = finding_aggregator::create(
///         "exampleFindingAggregator",
///         FindingAggregatorArgs::builder().linking_mode("ALL_REGIONS").build_struct(),
///     );
/// }
/// ```
///
/// ### All Regions Except Specified Regions Usage
///
/// The following example will enable the aggregator for every region except those specified in `specified_regions`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleFindingAggregator = finding_aggregator::create(
///         "exampleFindingAggregator",
///         FindingAggregatorArgs::builder()
///             .linking_mode("ALL_REGIONS_EXCEPT_SPECIFIED")
///             .specified_regions(vec!["eu-west-1", "eu-west-2",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specified Regions Usage
///
/// The following example will enable the aggregator for every region specified in `specified_regions`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleFindingAggregator = finding_aggregator::create(
///         "exampleFindingAggregator",
///         FindingAggregatorArgs::builder()
///             .linking_mode("SPECIFIED_REGIONS")
///             .specified_regions(vec!["eu-west-1", "eu-west-2",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub finding aggregator using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/findingAggregator:FindingAggregator example arn:aws:securityhub:eu-west-1:123456789098:finding-aggregator/abcd1234-abcd-1234-1234-abcdef123456
/// ```
pub mod finding_aggregator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FindingAggregatorArgs {
        /// Indicates whether to aggregate findings from all of the available Regions or from a specified list. The options are `ALL_REGIONS`, `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`. When `ALL_REGIONS` or `ALL_REGIONS_EXCEPT_SPECIFIED` are used, Security Hub will automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.
        #[builder(into)]
        pub linking_mode: pulumi_wasm_rust::Output<String>,
        /// List of regions to include or exclude (required if `linking_mode` is set to `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`)
        #[builder(into, default)]
        pub specified_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct FindingAggregatorResult {
        /// Indicates whether to aggregate findings from all of the available Regions or from a specified list. The options are `ALL_REGIONS`, `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`. When `ALL_REGIONS` or `ALL_REGIONS_EXCEPT_SPECIFIED` are used, Security Hub will automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.
        pub linking_mode: pulumi_wasm_rust::Output<String>,
        /// List of regions to include or exclude (required if `linking_mode` is set to `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`)
        pub specified_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FindingAggregatorArgs) -> FindingAggregatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let linking_mode_binding = args.linking_mode.get_inner();
        let specified_regions_binding = args.specified_regions.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/findingAggregator:FindingAggregator".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "linkingMode".into(),
                    value: &linking_mode_binding,
                },
                register_interface::ObjectField {
                    name: "specifiedRegions".into(),
                    value: &specified_regions_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "linkingMode".into(),
                },
                register_interface::ResultField {
                    name: "specifiedRegions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FindingAggregatorResult {
            linking_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkingMode").unwrap(),
            ),
            specified_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specifiedRegions").unwrap(),
            ),
        }
    }
}