/// Provides a CloudWatch Logs destination resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testDestination = log_destination::create(
///         "testDestination",
///         LogDestinationArgs::builder()
///             .name("test_destination")
///             .role_arn("${iamForCloudwatch.arn}")
///             .target_arn("${kinesisForCloudwatch.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Logs destinations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logDestination:LogDestination test_destination test_destination
/// ```
pub mod log_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogDestinationArgs {
        /// A name for the log destination.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to put data into the target.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the target Amazon Kinesis stream resource for the destination.
        #[builder(into)]
        pub target_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LogDestinationResult {
        /// The Amazon Resource Name (ARN) specifying the log destination.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A name for the log destination.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to put data into the target.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the target Amazon Kinesis stream resource for the destination.
        pub target_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogDestinationArgs) -> LogDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_arn_binding = args.target_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logDestination:LogDestination".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogDestinationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetArn").unwrap(),
            ),
        }
    }
}