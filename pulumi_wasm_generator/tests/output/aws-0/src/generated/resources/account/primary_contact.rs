/// Manages the specified primary contact information associated with an AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod primary_contact {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrimaryContactArgs {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The first line of the primary contact address.
        #[builder(into)]
        pub address_line1: pulumi_wasm_rust::Output<String>,
        /// The second line of the primary contact address, if any.
        #[builder(into, default)]
        pub address_line2: pulumi_wasm_rust::Output<Option<String>>,
        /// The third line of the primary contact address, if any.
        #[builder(into, default)]
        pub address_line3: pulumi_wasm_rust::Output<Option<String>>,
        /// The city of the primary contact address.
        #[builder(into)]
        pub city: pulumi_wasm_rust::Output<String>,
        /// The name of the company associated with the primary contact information, if any.
        #[builder(into, default)]
        pub company_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO-3166 two-letter country code for the primary contact address.
        #[builder(into)]
        pub country_code: pulumi_wasm_rust::Output<String>,
        /// The district or county of the primary contact address, if any.
        #[builder(into, default)]
        pub district_or_county: pulumi_wasm_rust::Output<Option<String>>,
        /// The full name of the primary contact address.
        #[builder(into)]
        pub full_name: pulumi_wasm_rust::Output<String>,
        /// The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.
        #[builder(into)]
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The postal code of the primary contact address.
        #[builder(into)]
        pub postal_code: pulumi_wasm_rust::Output<String>,
        /// The state or region of the primary contact address. This field is required in selected countries.
        #[builder(into, default)]
        pub state_or_region: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the website associated with the primary contact information, if any.
        #[builder(into, default)]
        pub website_url: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrimaryContactResult {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The first line of the primary contact address.
        pub address_line1: pulumi_wasm_rust::Output<String>,
        /// The second line of the primary contact address, if any.
        pub address_line2: pulumi_wasm_rust::Output<Option<String>>,
        /// The third line of the primary contact address, if any.
        pub address_line3: pulumi_wasm_rust::Output<Option<String>>,
        /// The city of the primary contact address.
        pub city: pulumi_wasm_rust::Output<String>,
        /// The name of the company associated with the primary contact information, if any.
        pub company_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO-3166 two-letter country code for the primary contact address.
        pub country_code: pulumi_wasm_rust::Output<String>,
        /// The district or county of the primary contact address, if any.
        pub district_or_county: pulumi_wasm_rust::Output<Option<String>>,
        /// The full name of the primary contact address.
        pub full_name: pulumi_wasm_rust::Output<String>,
        /// The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The postal code of the primary contact address.
        pub postal_code: pulumi_wasm_rust::Output<String>,
        /// The state or region of the primary contact address. This field is required in selected countries.
        pub state_or_region: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the website associated with the primary contact information, if any.
        pub website_url: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PrimaryContactArgs) -> PrimaryContactResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let address_line1_binding = args.address_line1.get_inner();
        let address_line2_binding = args.address_line2.get_inner();
        let address_line3_binding = args.address_line3.get_inner();
        let city_binding = args.city.get_inner();
        let company_name_binding = args.company_name.get_inner();
        let country_code_binding = args.country_code.get_inner();
        let district_or_county_binding = args.district_or_county.get_inner();
        let full_name_binding = args.full_name.get_inner();
        let phone_number_binding = args.phone_number.get_inner();
        let postal_code_binding = args.postal_code.get_inner();
        let state_or_region_binding = args.state_or_region.get_inner();
        let website_url_binding = args.website_url.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:account/primaryContact:PrimaryContact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "addressLine1".into(),
                    value: &address_line1_binding,
                },
                register_interface::ObjectField {
                    name: "addressLine2".into(),
                    value: &address_line2_binding,
                },
                register_interface::ObjectField {
                    name: "addressLine3".into(),
                    value: &address_line3_binding,
                },
                register_interface::ObjectField {
                    name: "city".into(),
                    value: &city_binding,
                },
                register_interface::ObjectField {
                    name: "companyName".into(),
                    value: &company_name_binding,
                },
                register_interface::ObjectField {
                    name: "countryCode".into(),
                    value: &country_code_binding,
                },
                register_interface::ObjectField {
                    name: "districtOrCounty".into(),
                    value: &district_or_county_binding,
                },
                register_interface::ObjectField {
                    name: "fullName".into(),
                    value: &full_name_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "postalCode".into(),
                    value: &postal_code_binding,
                },
                register_interface::ObjectField {
                    name: "stateOrRegion".into(),
                    value: &state_or_region_binding,
                },
                register_interface::ObjectField {
                    name: "websiteUrl".into(),
                    value: &website_url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "addressLine1".into(),
                },
                register_interface::ResultField {
                    name: "addressLine2".into(),
                },
                register_interface::ResultField {
                    name: "addressLine3".into(),
                },
                register_interface::ResultField {
                    name: "city".into(),
                },
                register_interface::ResultField {
                    name: "companyName".into(),
                },
                register_interface::ResultField {
                    name: "countryCode".into(),
                },
                register_interface::ResultField {
                    name: "districtOrCounty".into(),
                },
                register_interface::ResultField {
                    name: "fullName".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "postalCode".into(),
                },
                register_interface::ResultField {
                    name: "stateOrRegion".into(),
                },
                register_interface::ResultField {
                    name: "websiteUrl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrimaryContactResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            address_line1: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressLine1").unwrap(),
            ),
            address_line2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressLine2").unwrap(),
            ),
            address_line3: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressLine3").unwrap(),
            ),
            city: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("city").unwrap(),
            ),
            company_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("companyName").unwrap(),
            ),
            country_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("countryCode").unwrap(),
            ),
            district_or_county: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("districtOrCounty").unwrap(),
            ),
            full_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullName").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            postal_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postalCode").unwrap(),
            ),
            state_or_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateOrRegion").unwrap(),
            ),
            website_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websiteUrl").unwrap(),
            ),
        }
    }
}
