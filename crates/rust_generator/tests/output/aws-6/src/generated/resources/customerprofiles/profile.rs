/// Resource for managing an Amazon Customer Profiles Profile.
/// See the [Create Profile](https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_CreateProfile.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// A unique account number that you have given to the customer.
        #[builder(into, default)]
        pub account_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Any additional information relevant to the customer’s profile.
        #[builder(into, default)]
        pub additional_information: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that specifies a generic address associated with the customer that is not mailing, shipping, or billing. Documented below.
        #[builder(into, default)]
        pub address: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::ProfileAddress>,
        >,
        /// A key value pair of attributes of a customer profile.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies the customer’s billing address. Documented below.
        #[builder(into, default)]
        pub billing_address: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::ProfileBillingAddress>,
        >,
        /// The customer’s birth date.
        #[builder(into, default)]
        pub birth_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s business email address.
        #[builder(into, default)]
        pub business_email_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the customer’s business.
        #[builder(into, default)]
        pub business_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s business phone number.
        #[builder(into, default)]
        pub business_phone_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of your Customer Profile domain. It must be unique for your AWS account.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The customer’s email address, which has not been specified as a personal or business address.
        #[builder(into, default)]
        pub email_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s first name.
        #[builder(into, default)]
        pub first_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The gender with which the customer identifies.
        #[builder(into, default)]
        pub gender_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s home phone number.
        #[builder(into, default)]
        pub home_phone_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s last name.
        #[builder(into, default)]
        pub last_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that specifies the customer’s mailing address. Documented below.
        #[builder(into, default)]
        pub mailing_address: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::ProfileMailingAddress>,
        >,
        /// The customer’s middle name.
        #[builder(into, default)]
        pub middle_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s mobile phone number.
        #[builder(into, default)]
        pub mobile_phone_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of profile used to describe the customer.
        #[builder(into, default)]
        pub party_type_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s personal email address.
        #[builder(into, default)]
        pub personal_email_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer’s phone number, which has not been specified as a mobile, home, or business number.
        #[builder(into, default)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that specifies the customer’s shipping address. Documented below.
        #[builder(into, default)]
        pub shipping_address: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::ProfileShippingAddress>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// A unique account number that you have given to the customer.
        pub account_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// Any additional information relevant to the customer’s profile.
        pub additional_information: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that specifies a generic address associated with the customer that is not mailing, shipping, or billing. Documented below.
        pub address: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::ProfileAddress>,
        >,
        /// A key value pair of attributes of a customer profile.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies the customer’s billing address. Documented below.
        pub billing_address: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::ProfileBillingAddress>,
        >,
        /// The customer’s birth date.
        pub birth_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s business email address.
        pub business_email_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the customer’s business.
        pub business_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s business phone number.
        pub business_phone_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of your Customer Profile domain. It must be unique for your AWS account.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The customer’s email address, which has not been specified as a personal or business address.
        pub email_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s first name.
        pub first_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The gender with which the customer identifies.
        pub gender_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s home phone number.
        pub home_phone_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s last name.
        pub last_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that specifies the customer’s mailing address. Documented below.
        pub mailing_address: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::ProfileMailingAddress>,
        >,
        /// The customer’s middle name.
        pub middle_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s mobile phone number.
        pub mobile_phone_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of profile used to describe the customer.
        pub party_type_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s personal email address.
        pub personal_email_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The customer’s phone number, which has not been specified as a mobile, home, or business number.
        pub phone_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that specifies the customer’s shipping address. Documented below.
        pub shipping_address: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::ProfileShippingAddress>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProfileArgs,
    ) -> ProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_number_binding = args.account_number.get_output(context);
        let additional_information_binding = args
            .additional_information
            .get_output(context);
        let address_binding = args.address.get_output(context);
        let attributes_binding = args.attributes.get_output(context);
        let billing_address_binding = args.billing_address.get_output(context);
        let birth_date_binding = args.birth_date.get_output(context);
        let business_email_address_binding = args
            .business_email_address
            .get_output(context);
        let business_name_binding = args.business_name.get_output(context);
        let business_phone_number_binding = args
            .business_phone_number
            .get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let email_address_binding = args.email_address.get_output(context);
        let first_name_binding = args.first_name.get_output(context);
        let gender_string_binding = args.gender_string.get_output(context);
        let home_phone_number_binding = args.home_phone_number.get_output(context);
        let last_name_binding = args.last_name.get_output(context);
        let mailing_address_binding = args.mailing_address.get_output(context);
        let middle_name_binding = args.middle_name.get_output(context);
        let mobile_phone_number_binding = args.mobile_phone_number.get_output(context);
        let party_type_string_binding = args.party_type_string.get_output(context);
        let personal_email_address_binding = args
            .personal_email_address
            .get_output(context);
        let phone_number_binding = args.phone_number.get_output(context);
        let shipping_address_binding = args.shipping_address.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:customerprofiles/profile:Profile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountNumber".into(),
                    value: &account_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalInformation".into(),
                    value: &additional_information_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "address".into(),
                    value: &address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAddress".into(),
                    value: &billing_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "birthDate".into(),
                    value: &birth_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "businessEmailAddress".into(),
                    value: &business_email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "businessName".into(),
                    value: &business_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "businessPhoneNumber".into(),
                    value: &business_phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "genderString".into(),
                    value: &gender_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homePhoneNumber".into(),
                    value: &home_phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mailingAddress".into(),
                    value: &mailing_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "middleName".into(),
                    value: &middle_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobilePhoneNumber".into(),
                    value: &mobile_phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partyTypeString".into(),
                    value: &party_type_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "personalEmailAddress".into(),
                    value: &personal_email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shippingAddress".into(),
                    value: &shipping_address_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProfileResult {
            account_number: o.get_field("accountNumber"),
            additional_information: o.get_field("additionalInformation"),
            address: o.get_field("address"),
            attributes: o.get_field("attributes"),
            billing_address: o.get_field("billingAddress"),
            birth_date: o.get_field("birthDate"),
            business_email_address: o.get_field("businessEmailAddress"),
            business_name: o.get_field("businessName"),
            business_phone_number: o.get_field("businessPhoneNumber"),
            domain_name: o.get_field("domainName"),
            email_address: o.get_field("emailAddress"),
            first_name: o.get_field("firstName"),
            gender_string: o.get_field("genderString"),
            home_phone_number: o.get_field("homePhoneNumber"),
            last_name: o.get_field("lastName"),
            mailing_address: o.get_field("mailingAddress"),
            middle_name: o.get_field("middleName"),
            mobile_phone_number: o.get_field("mobilePhoneNumber"),
            party_type_string: o.get_field("partyTypeString"),
            personal_email_address: o.get_field("personalEmailAddress"),
            phone_number: o.get_field("phoneNumber"),
            shipping_address: o.get_field("shippingAddress"),
        }
    }
}
