/// Resource for managing an AWS Redshift Data Share Consumer Association.
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
///     let example = data_share_consumer_association::create(
///         "example",
///         DataShareConsumerAssociationArgs::builder()
///             .associate_entire_account(true)
///             .data_share_arn(
///                 "arn:aws:redshift:us-west-2:123456789012:datashare:b3bfde75-73fd-408b-9086-d6fccfd6d588/example",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Consumer Region
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_share_consumer_association::create(
///         "example",
///         DataShareConsumerAssociationArgs::builder()
///             .consumer_region("us-west-2")
///             .data_share_arn(
///                 "arn:aws:redshift:us-west-2:123456789012:datashare:b3bfde75-73fd-408b-9086-d6fccfd6d588/example",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Data Share Consumer Association using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/dataShareConsumerAssociation:DataShareConsumerAssociation example arn:aws:redshift:us-west-2:123456789012:datashare:b3bfde75-73fd-408b-9086-d6fccfd6d588/example,,,us-west-2
/// ```
pub mod data_share_consumer_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataShareConsumerAssociationArgs {
        /// Whether to allow write operations for a datashare.
        #[builder(into, default)]
        pub allow_writes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the datashare is associated with the entire account. Conflicts with `consumer_arn` and `consumer_region`.
        #[builder(into, default)]
        pub associate_entire_account: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the consumer that is associated with the datashare. Conflicts with `associate_entire_account` and `consumer_region`.
        #[builder(into, default)]
        pub consumer_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// From a datashare consumer account, associates a datashare with all existing and future namespaces in the specified AWS Region. Conflicts with `associate_entire_account` and `consumer_arn`.
        #[builder(into, default)]
        pub consumer_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the datashare that the consumer is to use with the account or the namespace.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_share_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataShareConsumerAssociationResult {
        /// Whether to allow write operations for a datashare.
        pub allow_writes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the datashare is associated with the entire account. Conflicts with `consumer_arn` and `consumer_region`.
        pub associate_entire_account: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the consumer that is associated with the datashare. Conflicts with `associate_entire_account` and `consumer_region`.
        pub consumer_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// From a datashare consumer account, associates a datashare with all existing and future namespaces in the specified AWS Region. Conflicts with `associate_entire_account` and `consumer_arn`.
        pub consumer_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the datashare that the consumer is to use with the account or the namespace.
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
        args: DataShareConsumerAssociationArgs,
    ) -> DataShareConsumerAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_writes_binding = args.allow_writes.get_inner();
        let associate_entire_account_binding = args.associate_entire_account.get_inner();
        let consumer_arn_binding = args.consumer_arn.get_inner();
        let consumer_region_binding = args.consumer_region.get_inner();
        let data_share_arn_binding = args.data_share_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/dataShareConsumerAssociation:DataShareConsumerAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowWrites".into(),
                    value: &allow_writes_binding,
                },
                register_interface::ObjectField {
                    name: "associateEntireAccount".into(),
                    value: &associate_entire_account_binding,
                },
                register_interface::ObjectField {
                    name: "consumerArn".into(),
                    value: &consumer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "consumerRegion".into(),
                    value: &consumer_region_binding,
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
                    name: "associateEntireAccount".into(),
                },
                register_interface::ResultField {
                    name: "consumerArn".into(),
                },
                register_interface::ResultField {
                    name: "consumerRegion".into(),
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
        DataShareConsumerAssociationResult {
            allow_writes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowWrites").unwrap(),
            ),
            associate_entire_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associateEntireAccount").unwrap(),
            ),
            consumer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerArn").unwrap(),
            ),
            consumer_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerRegion").unwrap(),
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