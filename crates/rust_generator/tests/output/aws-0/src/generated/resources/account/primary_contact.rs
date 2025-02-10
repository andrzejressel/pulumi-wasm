/// Manages the specified primary contact information associated with an AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = primary_contact::create(
///         "test",
///         PrimaryContactArgs::builder()
///             .address_line_1("123 Any Street")
///             .city("Seattle")
///             .company_name("Example Corp, Inc.")
///             .country_code("US")
///             .district_or_county("King")
///             .full_name("My Name")
///             .phone_number("+64211111111")
///             .postal_code("98101")
///             .state_or_region("WA")
///             .website_url("https://www.examplecorp.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the Primary Contact using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:account/primaryContact:PrimaryContact test 1234567890
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod primary_contact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrimaryContactArgs {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The first line of the primary contact address.
        #[builder(into)]
        pub address_line1: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The second line of the primary contact address, if any.
        #[builder(into, default)]
        pub address_line2: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The third line of the primary contact address, if any.
        #[builder(into, default)]
        pub address_line3: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The city of the primary contact address.
        #[builder(into)]
        pub city: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the company associated with the primary contact information, if any.
        #[builder(into, default)]
        pub company_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ISO-3166 two-letter country code for the primary contact address.
        #[builder(into)]
        pub country_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The district or county of the primary contact address, if any.
        #[builder(into, default)]
        pub district_or_county: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full name of the primary contact address.
        #[builder(into)]
        pub full_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The postal code of the primary contact address.
        #[builder(into)]
        pub postal_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The state or region of the primary contact address. This field is required in selected countries.
        #[builder(into, default)]
        pub state_or_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the website associated with the primary contact information, if any.
        #[builder(into, default)]
        pub website_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrimaryContactResult {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The first line of the primary contact address.
        pub address_line1: pulumi_gestalt_rust::Output<String>,
        /// The second line of the primary contact address, if any.
        pub address_line2: pulumi_gestalt_rust::Output<Option<String>>,
        /// The third line of the primary contact address, if any.
        pub address_line3: pulumi_gestalt_rust::Output<Option<String>>,
        /// The city of the primary contact address.
        pub city: pulumi_gestalt_rust::Output<String>,
        /// The name of the company associated with the primary contact information, if any.
        pub company_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ISO-3166 two-letter country code for the primary contact address.
        pub country_code: pulumi_gestalt_rust::Output<String>,
        /// The district or county of the primary contact address, if any.
        pub district_or_county: pulumi_gestalt_rust::Output<Option<String>>,
        /// The full name of the primary contact address.
        pub full_name: pulumi_gestalt_rust::Output<String>,
        /// The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// The postal code of the primary contact address.
        pub postal_code: pulumi_gestalt_rust::Output<String>,
        /// The state or region of the primary contact address. This field is required in selected countries.
        pub state_or_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the website associated with the primary contact information, if any.
        pub website_url: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrimaryContactArgs,
    ) -> PrimaryContactResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let address_line1_binding = args.address_line1.get_output(context);
        let address_line2_binding = args.address_line2.get_output(context);
        let address_line3_binding = args.address_line3.get_output(context);
        let city_binding = args.city.get_output(context);
        let company_name_binding = args.company_name.get_output(context);
        let country_code_binding = args.country_code.get_output(context);
        let district_or_county_binding = args.district_or_county.get_output(context);
        let full_name_binding = args.full_name.get_output(context);
        let phone_number_binding = args.phone_number.get_output(context);
        let postal_code_binding = args.postal_code.get_output(context);
        let state_or_region_binding = args.state_or_region.get_output(context);
        let website_url_binding = args.website_url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:account/primaryContact:PrimaryContact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressLine1".into(),
                    value: address_line1_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressLine2".into(),
                    value: address_line2_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressLine3".into(),
                    value: address_line3_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "city".into(),
                    value: city_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "companyName".into(),
                    value: company_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "countryCode".into(),
                    value: country_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "districtOrCounty".into(),
                    value: district_or_county_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fullName".into(),
                    value: full_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: phone_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postalCode".into(),
                    value: postal_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stateOrRegion".into(),
                    value: state_or_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "websiteUrl".into(),
                    value: website_url_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrimaryContactResult {
            account_id: o.get_field("accountId"),
            address_line1: o.get_field("addressLine1"),
            address_line2: o.get_field("addressLine2"),
            address_line3: o.get_field("addressLine3"),
            city: o.get_field("city"),
            company_name: o.get_field("companyName"),
            country_code: o.get_field("countryCode"),
            district_or_county: o.get_field("districtOrCounty"),
            full_name: o.get_field("fullName"),
            phone_number: o.get_field("phoneNumber"),
            postal_code: o.get_field("postalCode"),
            state_or_region: o.get_field("stateOrRegion"),
            website_url: o.get_field("websiteUrl"),
        }
    }
}
