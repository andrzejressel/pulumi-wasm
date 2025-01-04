/// Provides an Amazon Connect Phone Number resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:PhoneNumber
///     properties:
///       targetArn: ${exampleAwsConnectInstance.arn}
///       countryCode: US
///       type: DID
///       tags:
///         hello: world
/// ```
///
/// ### Description
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = phone_number::create(
///         "example",
///         PhoneNumberArgs::builder()
///             .country_code("US")
///             .description("example description")
///             .target_arn("${exampleAwsConnectInstance.arn}")
///             .type_("DID")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Prefix to filter phone numbers
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = phone_number::create(
///         "example",
///         PhoneNumberArgs::builder()
///             .country_code("US")
///             .prefix("+18005")
///             .target_arn("${exampleAwsConnectInstance.arn}")
///             .type_("DID")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Phone Numbers using its `id`. For example:
///
/// ```sh
/// $ pulumi import aws:connect/phoneNumber:PhoneNumber example 12345678-abcd-1234-efgh-9876543210ab
/// ```
pub mod phone_number {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PhoneNumberArgs {
        /// The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
        #[builder(into)]
        pub country_code: pulumi_wasm_rust::Output<String>,
        /// The description of the phone number.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
        #[builder(into, default)]
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
        #[builder(into)]
        pub target_arn: pulumi_wasm_rust::Output<String>,
        /// The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PhoneNumberResult {
        /// The ARN of the phone number.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
        pub country_code: pulumi_wasm_rust::Output<String>,
        /// The description of the phone number.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The phone number. Phone numbers are formatted `[+] [country code] [subscriber number including area code]`.
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the phone number. Valid Values: `CLAIMED` | `IN_PROGRESS` | `FAILED`.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::connect::PhoneNumberStatus>,
        >,
        /// Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
        pub target_arn: pulumi_wasm_rust::Output<String>,
        /// The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PhoneNumberArgs) -> PhoneNumberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let country_code_binding = args.country_code.get_inner();
        let description_binding = args.description.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_arn_binding = args.target_arn.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/phoneNumber:PhoneNumber".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "countryCode".into(),
                    value: &country_code_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "countryCode".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
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
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PhoneNumberResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            country_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("countryCode").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
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
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
