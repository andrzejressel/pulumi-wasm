/// Resource for managing an AWS QuickSight Ingestion.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ingestion::create(
///         "example",
///         IngestionArgs::builder()
///             .data_set_id("${exampleAwsQuicksightDataSet.dataSetId}")
///             .ingestion_id("example-id")
///             .ingestion_type("FULL_REFRESH")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Ingestion using the AWS account ID, data set ID, and ingestion ID separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/ingestion:Ingestion example 123456789012,example-dataset-id,example-ingestion-id
/// ```
pub mod ingestion {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IngestionArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the dataset used in the ingestion.
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// ID for the ingestion.
        #[builder(into)]
        pub ingestion_id: pulumi_wasm_rust::Output<String>,
        /// Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub ingestion_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IngestionResult {
        /// ARN of the Ingestion.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// ID of the dataset used in the ingestion.
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// ID for the ingestion.
        pub ingestion_id: pulumi_wasm_rust::Output<String>,
        /// Ingestion status.
        pub ingestion_status: pulumi_wasm_rust::Output<String>,
        /// Type of ingestion to be created. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
        ///
        /// The following arguments are optional:
        pub ingestion_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IngestionArgs) -> IngestionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let data_set_id_binding = args.data_set_id.get_inner();
        let ingestion_id_binding = args.ingestion_id.get_inner();
        let ingestion_type_binding = args.ingestion_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/ingestion:Ingestion".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "ingestionId".into(),
                    value: &ingestion_id_binding,
                },
                register_interface::ObjectField {
                    name: "ingestionType".into(),
                    value: &ingestion_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "dataSetId".into(),
                },
                register_interface::ResultField {
                    name: "ingestionId".into(),
                },
                register_interface::ResultField {
                    name: "ingestionStatus".into(),
                },
                register_interface::ResultField {
                    name: "ingestionType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IngestionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            data_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetId").unwrap(),
            ),
            ingestion_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingestionId").unwrap(),
            ),
            ingestion_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingestionStatus").unwrap(),
            ),
            ingestion_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingestionType").unwrap(),
            ),
        }
    }
}
