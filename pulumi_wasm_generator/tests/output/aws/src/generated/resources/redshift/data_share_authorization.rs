/// Resource for managing an AWS Redshift Data Share Authorization.
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
///     let example = data_share_authorization::create(
///         "example",
///         DataShareAuthorizationArgs::builder()
///             .consumer_identifier("123456789012")
///             .data_share_arn(
///                 "arn:aws:redshift:us-west-2:123456789012:datashare:3072dae5-022b-4d45-9cd3-01f010aae4b2/example_share",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Data Share Authorization using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/dataShareAuthorization:DataShareAuthorization example arn:aws:redshift:us-west-2:123456789012:datashare:3072dae5-022b-4d45-9cd3-01f010aae4b2/example_share,123456789012
/// ```
pub mod data_share_authorization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataShareAuthorizationArgs {
        /// Whether to allow write operations for a datashare.
        #[builder(into, default)]
        pub allow_writes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of the data consumer that is authorized to access the datashare. This identifier is an AWS account ID or a keyword, such as `ADX`.
        #[builder(into)]
        pub consumer_identifier: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the datashare that producers are to authorize sharing for.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_share_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataShareAuthorizationResult {
        /// Whether to allow write operations for a datashare.
        pub allow_writes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of the data consumer that is authorized to access the datashare. This identifier is an AWS account ID or a keyword, such as `ADX`.
        pub consumer_identifier: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the datashare that producers are to authorize sharing for.
        ///
        /// The following arguments are optional:
        pub data_share_arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of a datashare to show its managing entity.
        pub managed_by: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the producer.
        pub producer_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataShareAuthorizationArgs,
    ) -> DataShareAuthorizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_writes_binding = args.allow_writes.get_inner();
        let consumer_identifier_binding = args.consumer_identifier.get_inner();
        let data_share_arn_binding = args.data_share_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/dataShareAuthorization:DataShareAuthorization".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowWrites".into(),
                    value: &allow_writes_binding,
                },
                register_interface::ObjectField {
                    name: "consumerIdentifier".into(),
                    value: &consumer_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dataShareArn".into(),
                    value: &data_share_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowWrites".into(),
                },
                register_interface::ResultField {
                    name: "consumerIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dataShareArn".into(),
                },
                register_interface::ResultField {
                    name: "managedBy".into(),
                },
                register_interface::ResultField {
                    name: "producerArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataShareAuthorizationResult {
            allow_writes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowWrites").unwrap(),
            ),
            consumer_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerIdentifier").unwrap(),
            ),
            data_share_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataShareArn").unwrap(),
            ),
            managed_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedBy").unwrap(),
            ),
            producer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("producerArn").unwrap(),
            ),
        }
    }
}