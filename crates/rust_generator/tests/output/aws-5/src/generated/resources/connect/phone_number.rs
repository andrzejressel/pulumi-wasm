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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod phone_number {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PhoneNumberArgs {
        /// The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
        #[builder(into)]
        pub country_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the phone number.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
        #[builder(into)]
        pub target_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PhoneNumberResult {
        /// The ARN of the phone number.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
        pub country_code: pulumi_gestalt_rust::Output<String>,
        /// The description of the phone number.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The phone number. Phone numbers are formatted `[+] [country code] [subscriber number including area code]`.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of the phone number. Valid Values: `CLAIMED` | `IN_PROGRESS` | `FAILED`.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::connect::PhoneNumberStatus>,
        >,
        /// Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
        /// The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PhoneNumberArgs,
    ) -> PhoneNumberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let country_code_binding = args.country_code.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let prefix_binding = args.prefix.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_arn_binding = args.target_arn.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/phoneNumber:PhoneNumber".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PhoneNumberResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            country_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("countryCode"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            phone_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("phoneNumber"),
            ),
            prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefix"),
            ),
            statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetArn"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
