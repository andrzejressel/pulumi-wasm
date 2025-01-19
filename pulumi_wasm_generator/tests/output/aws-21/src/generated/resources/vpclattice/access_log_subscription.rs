/// Resource for managing an AWS VPC Lattice Service Network or Service Access log subscription.
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
///     let example = access_log_subscription::create(
///         "example",
///         AccessLogSubscriptionArgs::builder()
///             .destination_arn("${bucket.arn}")
///             .resource_identifier("${exampleAwsVpclatticeServiceNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Access Log Subscription using the access log subscription ID. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/accessLogSubscription:AccessLogSubscription example rft-8012925589
/// ```
pub mod access_log_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessLogSubscriptionArgs {
        /// Amazon Resource Name (ARN) of the log destination.
        #[builder(into)]
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network or service. You must use the ARN if the resources specified in the operation are in different accounts.
        #[builder(into)]
        pub resource_identifier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessLogSubscriptionResult {
        /// Amazon Resource Name (ARN) of the access log subscription.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the log destination.
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the service network or service.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network or service. You must use the ARN if the resources specified in the operation are in different accounts.
        pub resource_identifier: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessLogSubscriptionArgs,
    ) -> AccessLogSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_arn_binding = args.destination_arn.get_inner();
        let resource_identifier_binding = args.resource_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/accessLogSubscription:AccessLogSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessLogSubscriptionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            resource_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceIdentifier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
