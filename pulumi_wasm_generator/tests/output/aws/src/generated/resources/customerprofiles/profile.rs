/// Resource for managing an Amazon Customer Profiles Profile.
/// See the [Create Profile](https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_CreateProfile.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder().domain_name("example").build_struct(),
///     );
///     let exampleProfile = profile::create(
///         "exampleProfile",
///         ProfileArgs::builder().domain_name("${example.domainName}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Customer Profiles Profile using the resource `id`. For example:
///
/// ```sh
/// $ pulumi import aws:customerprofiles/profile:Profile example domain-name/5f2f473dfbe841eb8d05cfc2a4c926df
/// ```
pub mod profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// A unique account number that you have given to the customer.
        #[builder(into, default)]
        pub account_number: pulumi_wasm_rust::Output<Option<String>>,
        /// Any additional information relevant to the customer’s profile.
        #[builder(into, default)]
        pub additional_information: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies a generic address associated with the customer that is not mailing, shipping, or billing. Documented below.
        #[builder(into, default)]
        pub address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileAddress>,
        >,
        /// A key value pair of attributes of a customer profile.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies the customer’s billing address. Documented below.
        #[builder(into, default)]
        pub billing_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileBillingAddress>,
        >,
        /// The customer’s birth date.
        #[builder(into, default)]
        pub birth_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s business email address.
        #[builder(into, default)]
        pub business_email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the customer’s business.
        #[builder(into, default)]
        pub business_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s business phone number.
        #[builder(into, default)]
        pub business_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of your Customer Profile domain. It must be unique for your AWS account.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The customer’s email address, which has not been specified as a personal or business address.
        #[builder(into, default)]
        pub email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s first name.
        #[builder(into, default)]
        pub first_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The gender with which the customer identifies.
        #[builder(into, default)]
        pub gender_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s home phone number.
        #[builder(into, default)]
        pub home_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s last name.
        #[builder(into, default)]
        pub last_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies the customer’s mailing address. Documented below.
        #[builder(into, default)]
        pub mailing_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileMailingAddress>,
        >,
        /// The customer’s middle name.
        #[builder(into, default)]
        pub middle_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s mobile phone number.
        #[builder(into, default)]
        pub mobile_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of profile used to describe the customer.
        #[builder(into, default)]
        pub party_type_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s personal email address.
        #[builder(into, default)]
        pub personal_email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s phone number, which has not been specified as a mobile, home, or business number.
        #[builder(into, default)]
        pub phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies the customer’s shipping address. Documented below.
        #[builder(into, default)]
        pub shipping_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileShippingAddress>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// A unique account number that you have given to the customer.
        pub account_number: pulumi_wasm_rust::Output<Option<String>>,
        /// Any additional information relevant to the customer’s profile.
        pub additional_information: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies a generic address associated with the customer that is not mailing, shipping, or billing. Documented below.
        pub address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileAddress>,
        >,
        /// A key value pair of attributes of a customer profile.
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies the customer’s billing address. Documented below.
        pub billing_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileBillingAddress>,
        >,
        /// The customer’s birth date.
        pub birth_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s business email address.
        pub business_email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the customer’s business.
        pub business_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s business phone number.
        pub business_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of your Customer Profile domain. It must be unique for your AWS account.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The customer’s email address, which has not been specified as a personal or business address.
        pub email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s first name.
        pub first_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The gender with which the customer identifies.
        pub gender_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s home phone number.
        pub home_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s last name.
        pub last_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies the customer’s mailing address. Documented below.
        pub mailing_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileMailingAddress>,
        >,
        /// The customer’s middle name.
        pub middle_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s mobile phone number.
        pub mobile_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of profile used to describe the customer.
        pub party_type_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s personal email address.
        pub personal_email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer’s phone number, which has not been specified as a mobile, home, or business number.
        pub phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that specifies the customer’s shipping address. Documented below.
        pub shipping_address: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::ProfileShippingAddress>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProfileArgs) -> ProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_number_binding = args.account_number.get_inner();
        let additional_information_binding = args.additional_information.get_inner();
        let address_binding = args.address.get_inner();
        let attributes_binding = args.attributes.get_inner();
        let billing_address_binding = args.billing_address.get_inner();
        let birth_date_binding = args.birth_date.get_inner();
        let business_email_address_binding = args.business_email_address.get_inner();
        let business_name_binding = args.business_name.get_inner();
        let business_phone_number_binding = args.business_phone_number.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let email_address_binding = args.email_address.get_inner();
        let first_name_binding = args.first_name.get_inner();
        let gender_string_binding = args.gender_string.get_inner();
        let home_phone_number_binding = args.home_phone_number.get_inner();
        let last_name_binding = args.last_name.get_inner();
        let mailing_address_binding = args.mailing_address.get_inner();
        let middle_name_binding = args.middle_name.get_inner();
        let mobile_phone_number_binding = args.mobile_phone_number.get_inner();
        let party_type_string_binding = args.party_type_string.get_inner();
        let personal_email_address_binding = args.personal_email_address.get_inner();
        let phone_number_binding = args.phone_number.get_inner();
        let shipping_address_binding = args.shipping_address.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:customerprofiles/profile:Profile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountNumber".into(),
                    value: &account_number_binding,
                },
                register_interface::ObjectField {
                    name: "additionalInformation".into(),
                    value: &additional_information_binding,
                },
                register_interface::ObjectField {
                    name: "address".into(),
                    value: &address_binding,
                },
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "billingAddress".into(),
                    value: &billing_address_binding,
                },
                register_interface::ObjectField {
                    name: "birthDate".into(),
                    value: &birth_date_binding,
                },
                register_interface::ObjectField {
                    name: "businessEmailAddress".into(),
                    value: &business_email_address_binding,
                },
                register_interface::ObjectField {
                    name: "businessName".into(),
                    value: &business_name_binding,
                },
                register_interface::ObjectField {
                    name: "businessPhoneNumber".into(),
                    value: &business_phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding,
                },
                register_interface::ObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding,
                },
                register_interface::ObjectField {
                    name: "genderString".into(),
                    value: &gender_string_binding,
                },
                register_interface::ObjectField {
                    name: "homePhoneNumber".into(),
                    value: &home_phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding,
                },
                register_interface::ObjectField {
                    name: "mailingAddress".into(),
                    value: &mailing_address_binding,
                },
                register_interface::ObjectField {
                    name: "middleName".into(),
                    value: &middle_name_binding,
                },
                register_interface::ObjectField {
                    name: "mobilePhoneNumber".into(),
                    value: &mobile_phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "partyTypeString".into(),
                    value: &party_type_string_binding,
                },
                register_interface::ObjectField {
                    name: "personalEmailAddress".into(),
                    value: &personal_email_address_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "shippingAddress".into(),
                    value: &shipping_address_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountNumber".into(),
                },
                register_interface::ResultField {
                    name: "additionalInformation".into(),
                },
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "billingAddress".into(),
                },
                register_interface::ResultField {
                    name: "birthDate".into(),
                },
                register_interface::ResultField {
                    name: "businessEmailAddress".into(),
                },
                register_interface::ResultField {
                    name: "businessName".into(),
                },
                register_interface::ResultField {
                    name: "businessPhoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "emailAddress".into(),
                },
                register_interface::ResultField {
                    name: "firstName".into(),
                },
                register_interface::ResultField {
                    name: "genderString".into(),
                },
                register_interface::ResultField {
                    name: "homePhoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "lastName".into(),
                },
                register_interface::ResultField {
                    name: "mailingAddress".into(),
                },
                register_interface::ResultField {
                    name: "middleName".into(),
                },
                register_interface::ResultField {
                    name: "mobilePhoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "partyTypeString".into(),
                },
                register_interface::ResultField {
                    name: "personalEmailAddress".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "shippingAddress".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProfileResult {
            account_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountNumber").unwrap(),
            ),
            additional_information: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalInformation").unwrap(),
            ),
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            billing_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAddress").unwrap(),
            ),
            birth_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("birthDate").unwrap(),
            ),
            business_email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("businessEmailAddress").unwrap(),
            ),
            business_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("businessName").unwrap(),
            ),
            business_phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("businessPhoneNumber").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddress").unwrap(),
            ),
            first_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firstName").unwrap(),
            ),
            gender_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("genderString").unwrap(),
            ),
            home_phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homePhoneNumber").unwrap(),
            ),
            last_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastName").unwrap(),
            ),
            mailing_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mailingAddress").unwrap(),
            ),
            middle_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("middleName").unwrap(),
            ),
            mobile_phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobilePhoneNumber").unwrap(),
            ),
            party_type_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partyTypeString").unwrap(),
            ),
            personal_email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("personalEmailAddress").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            shipping_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shippingAddress").unwrap(),
            ),
        }
    }
}