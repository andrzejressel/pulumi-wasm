/// ## Example Usage
///
/// ### Clouddomains Registration Full
///
///
/// ```yaml
/// resources:
///   myRegistration:
///     type: gcp:clouddomains:Registration
///     name: my_registration
///     properties:
///       domainName: example-domain.com
///       location: global
///       labels:
///         labelkey: labelvalue
///       yearlyPrice:
///         currencyCode: USD
///         units: 12
///       dnsSettings:
///         customDns:
///           nameServers:
///             - ns-cloud-a1.googledomains.com.
///             - ns-cloud-a2.googledomains.com.
///             - ns-cloud-a3.googledomains.com.
///             - ns-cloud-a4.googledomains.com.
///       contactSettings:
///         privacy: REDACTED_CONTACT_DATA
///         registrantContact:
///           phoneNumber: '+12345000000'
///           email: user@example.com
///           postalAddress:
///             regionCode: US
///             postalCode: '95050'
///             administrativeArea: CA
///             locality: Example City
///             addressLines:
///               - 1234 Example street
///             recipients:
///               - example recipient
///         adminContact:
///           phoneNumber: '+12345000000'
///           email: user@example.com
///           postalAddress:
///             regionCode: US
///             postalCode: '95050'
///             administrativeArea: CA
///             locality: Example City
///             addressLines:
///               - 1234 Example street
///             recipients:
///               - example recipient
///         technicalContact:
///           phoneNumber: '+12345000000'
///           email: user@example.com
///           postalAddress:
///             regionCode: US
///             postalCode: '95050'
///             administrativeArea: CA
///             locality: Example City
///             addressLines:
///               - 1234 Example street
///             recipients:
///               - example recipient
/// ```
///
/// ## Import
///
/// Registration can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/registrations/{{domain_name}}`
///
/// * `{{project}}/{{location}}/{{domain_name}}`
///
/// * `{{location}}/{{domain_name}}`
///
/// When using the `pulumi import` command, Registration can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:clouddomains/registration:Registration default projects/{{project}}/locations/{{location}}/registrations/{{domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddomains/registration:Registration default {{project}}/{{location}}/{{domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddomains/registration:Registration default {{location}}/{{domain_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistrationArgs {
        /// The list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT
        #[builder(into, default)]
        pub contact_notices: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Required. Settings for contact information linked to the Registration.
        /// Structure is documented below.
        #[builder(into)]
        pub contact_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::clouddomains::RegistrationContactSettings,
        >,
        /// Settings controlling the DNS configuration of the Registration.
        #[builder(into, default)]
        pub dns_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddomains::RegistrationDnsSettings>,
        >,
        /// Required. The domain name. Unicode domain names must be expressed in Punycode format.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED
        #[builder(into, default)]
        pub domain_notices: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of labels associated with the Registration. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Settings for management of the Registration, including renewal, billing, and transfer
        #[builder(into, default)]
        pub management_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddomains::RegistrationManagementSettings>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Yearly price to register or renew the domain. The value that should be put here can be obtained from
        /// registrations.retrieveRegisterParameters or registrations.searchDomains calls.
        /// Structure is documented below.
        #[builder(into)]
        pub yearly_price: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::clouddomains::RegistrationYearlyPrice,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistrationResult {
        /// The list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT
        pub contact_notices: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Required. Settings for contact information linked to the Registration.
        /// Structure is documented below.
        pub contact_settings: pulumi_gestalt_rust::Output<
            super::super::types::clouddomains::RegistrationContactSettings,
        >,
        /// Output only. Time at which the automation was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Settings controlling the DNS configuration of the Registration.
        pub dns_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddomains::RegistrationDnsSettings>,
        >,
        /// Required. The domain name. Unicode domain names must be expressed in Punycode format.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED
        pub domain_notices: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Time at which the automation was updated.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. The set of issues with the Registration that require attention.
        pub issues: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of labels associated with the Registration. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Settings for management of the Registration, including renewal, billing, and transfer
        pub management_settings: pulumi_gestalt_rust::Output<
            super::super::types::clouddomains::RegistrationManagementSettings,
        >,
        /// Output only. Name of the Registration resource, in the format projects/*/locations/*/registrations/<domain_name>.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state.
        pub register_failure_reason: pulumi_gestalt_rust::Output<String>,
        /// Output only. The current state of the Registration.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Set of options for the contactSettings.privacy field that this Registration supports.
        pub supported_privacies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Required. Yearly price to register or renew the domain. The value that should be put here can be obtained from
        /// registrations.retrieveRegisterParameters or registrations.searchDomains calls.
        /// Structure is documented below.
        pub yearly_price: pulumi_gestalt_rust::Output<
            super::super::types::clouddomains::RegistrationYearlyPrice,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistrationArgs,
    ) -> RegistrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_notices_binding = args.contact_notices.get_output(context);
        let contact_settings_binding = args.contact_settings.get_output(context);
        let dns_settings_binding = args.dns_settings.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let domain_notices_binding = args.domain_notices.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_settings_binding = args.management_settings.get_output(context);
        let project_binding = args.project.get_output(context);
        let yearly_price_binding = args.yearly_price.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:clouddomains/registration:Registration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactNotices".into(),
                    value: contact_notices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactSettings".into(),
                    value: contact_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSettings".into(),
                    value: dns_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNotices".into(),
                    value: domain_notices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementSettings".into(),
                    value: management_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "yearlyPrice".into(),
                    value: yearly_price_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistrationResult {
            contact_notices: o.get_field("contactNotices"),
            contact_settings: o.get_field("contactSettings"),
            create_time: o.get_field("createTime"),
            dns_settings: o.get_field("dnsSettings"),
            domain_name: o.get_field("domainName"),
            domain_notices: o.get_field("domainNotices"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            issues: o.get_field("issues"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            management_settings: o.get_field("managementSettings"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            register_failure_reason: o.get_field("registerFailureReason"),
            state: o.get_field("state"),
            supported_privacies: o.get_field("supportedPrivacies"),
            yearly_price: o.get_field("yearlyPrice"),
        }
    }
}
