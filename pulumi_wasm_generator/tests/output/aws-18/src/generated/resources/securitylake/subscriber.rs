/// Resource for managing an AWS Security Lake Subscriber.
///
/// > **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.Subscriber`. Use a `depends_on` statement.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscriber::create(
///         "example",
///         SubscriberArgs::builder()
///             .access_type("S3")
///             .source(
///                 SubscriberSource::builder()
///                     .awsLogSourceResource(
///                         SubscriberSourceAwsLogSourceResource::builder()
///                             .sourceName("ROUTE53")
///                             .sourceVersion("1.0")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .subscriber_identity(
///                 SubscriberSubscriberIdentity::builder()
///                     .externalId("example")
///                     .principal("1234567890")
///                     .build_struct(),
///             )
///             .subscriber_name("example-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Lake subscriber using the subscriber ID. For example:
///
/// ```sh
/// $ pulumi import aws:securitylake/subscriber:Subscriber example 9f3bfe79-d543-474d-a93c-f3846805d208
/// ```
pub mod subscriber {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriberArgs {
        /// The Amazon S3 or Lake Formation access type.
        #[builder(into, default)]
        pub access_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
        #[builder(into, default)]
        pub source: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberSource>,
        >,
        /// The description for your subscriber account in Security Lake.
        #[builder(into, default)]
        pub subscriber_description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The AWS identity used to access your data. See `subscriber_identity` Block below.
        #[builder(into, default)]
        pub subscriber_identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberSubscriberIdentity>,
        >,
        /// The name of your Security Lake subscriber account.
        #[builder(into, default)]
        pub subscriber_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securitylake::SubscriberTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct SubscriberResult {
        /// The Amazon S3 or Lake Formation access type.
        pub access_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the Data Lake.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) which uniquely defines the AWS RAM resource share. Before accepting the RAM resource share invitation, you can view details related to the RAM resource share.
        pub resource_share_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the resource share.
        pub resource_share_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role to be used by the entity putting logs into your custom source partition.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN for the Amazon Security Lake Amazon S3 bucket.
        pub s3_bucket_arn: pulumi_wasm_rust::Output<String>,
        /// The supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
        pub source: pulumi_wasm_rust::Output<
            Option<super::super::types::securitylake::SubscriberSource>,
        >,
        /// The description for your subscriber account in Security Lake.
        pub subscriber_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The subscriber endpoint to which exception messages are posted.
        pub subscriber_endpoint: pulumi_wasm_rust::Output<String>,
        /// The AWS identity used to access your data. See `subscriber_identity` Block below.
        pub subscriber_identity: pulumi_wasm_rust::Output<
            Option<super::super::types::securitylake::SubscriberSubscriberIdentity>,
        >,
        /// The name of your Security Lake subscriber account.
        pub subscriber_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The subscriber status of the Amazon Security Lake subscriber account.
        pub subscriber_status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::securitylake::SubscriberTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubscriberArgs,
    ) -> SubscriberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_type_binding = args.access_type.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let subscriber_description_binding = args
            .subscriber_description
            .get_output(context)
            .get_inner();
        let subscriber_identity_binding = args
            .subscriber_identity
            .get_output(context)
            .get_inner();
        let subscriber_name_binding = args
            .subscriber_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securitylake/subscriber:Subscriber".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessType".into(),
                    value: &access_type_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "subscriberDescription".into(),
                    value: &subscriber_description_binding,
                },
                register_interface::ObjectField {
                    name: "subscriberIdentity".into(),
                    value: &subscriber_identity_binding,
                },
                register_interface::ObjectField {
                    name: "subscriberName".into(),
                    value: &subscriber_name_binding,
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
                    name: "accessType".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "resourceShareArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceShareName".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "s3BucketArn".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "subscriberDescription".into(),
                },
                register_interface::ResultField {
                    name: "subscriberEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "subscriberIdentity".into(),
                },
                register_interface::ResultField {
                    name: "subscriberName".into(),
                },
                register_interface::ResultField {
                    name: "subscriberStatus".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriberResult {
            access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            resource_share_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceShareArn").unwrap(),
            ),
            resource_share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceShareName").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            s3_bucket_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3BucketArn").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            subscriber_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberDescription").unwrap(),
            ),
            subscriber_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberEndpoint").unwrap(),
            ),
            subscriber_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberIdentity").unwrap(),
            ),
            subscriber_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberName").unwrap(),
            ),
            subscriber_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberStatus").unwrap(),
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
