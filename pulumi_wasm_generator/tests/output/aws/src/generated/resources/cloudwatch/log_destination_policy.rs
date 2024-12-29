/// Provides a CloudWatch Logs destination policy resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testDestinationPolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["logs:PutSubscriptionFilter",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["123456789012",]). type ("AWS").build_struct(),])
///                     .resources(vec!["${testDestination.arn}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testDestination = log_destination::create(
///         "testDestination",
///         LogDestinationArgs::builder()
///             .name("test_destination")
///             .role_arn("${iamForCloudwatch.arn}")
///             .target_arn("${kinesisForCloudwatch.arn}")
///             .build_struct(),
///     );
///     let testDestinationPolicyLogDestinationPolicy = log_destination_policy::create(
///         "testDestinationPolicyLogDestinationPolicy",
///         LogDestinationPolicyArgs::builder()
///             .access_policy("${testDestinationPolicy.json}")
///             .destination_name("${testDestination.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Logs destination policies using the `destination_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logDestinationPolicy:LogDestinationPolicy test_destination_policy test_destination
/// ```
pub mod log_destination_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogDestinationPolicyArgs {
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub access_policy: pulumi_wasm_rust::Output<String>,
        /// A name for the subscription filter
        #[builder(into)]
        pub destination_name: pulumi_wasm_rust::Output<String>,
        /// Specify true if you are updating an existing destination policy to grant permission to an organization ID instead of granting permission to individual AWS accounts.
        #[builder(into, default)]
        pub force_update: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LogDestinationPolicyResult {
        /// The policy document. This is a JSON formatted string.
        pub access_policy: pulumi_wasm_rust::Output<String>,
        /// A name for the subscription filter
        pub destination_name: pulumi_wasm_rust::Output<String>,
        /// Specify true if you are updating an existing destination policy to grant permission to an organization ID instead of granting permission to individual AWS accounts.
        pub force_update: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LogDestinationPolicyArgs,
    ) -> LogDestinationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policy_binding = args.access_policy.get_inner();
        let destination_name_binding = args.destination_name.get_inner();
        let force_update_binding = args.force_update.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logDestinationPolicy:LogDestinationPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicy".into(),
                    value: &access_policy_binding,
                },
                register_interface::ObjectField {
                    name: "destinationName".into(),
                    value: &destination_name_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdate".into(),
                    value: &force_update_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "destinationName".into(),
                },
                register_interface::ResultField {
                    name: "forceUpdate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogDestinationPolicyResult {
            access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicy").unwrap(),
            ),
            destination_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationName").unwrap(),
            ),
            force_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceUpdate").unwrap(),
            ),
        }
    }
}
