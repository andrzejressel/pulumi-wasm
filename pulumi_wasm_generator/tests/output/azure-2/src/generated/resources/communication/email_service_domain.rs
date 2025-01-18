/// Manages an Email Communication Service Domain.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleEmailService = email_service::create(
///         "exampleEmailService",
///         EmailServiceArgs::builder()
///             .data_location("United States")
///             .name("example-emailcommunicationservice")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEmailServiceDomain = email_service_domain::create(
///         "exampleEmailServiceDomain",
///         EmailServiceDomainArgs::builder()
///             .domain_management("AzureManaged")
///             .email_service_id("${exampleEmailService.id}")
///             .name("AzureManagedDomain")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Communication Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:communication/emailServiceDomain:EmailServiceDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/emailServices/emailCommunicationService1/domains/domain1
/// ```
///
pub mod email_service_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailServiceDomainArgs {
        /// Describes how a Domains resource is being managed. Possible values are `AzureManaged`, `CustomerManaged`, `CustomerManagedInExchangeOnline`. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub domain_management: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Email Communication Service where the Domain belongs to. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub email_service_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes user engagement tracking is enabled or disabled. Defaults to `false`.
        #[builder(into, default)]
        pub user_engagement_tracking_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EmailServiceDomainResult {
        /// Describes how a Domains resource is being managed. Possible values are `AzureManaged`, `CustomerManaged`, `CustomerManagedInExchangeOnline`. Changing this forces a new Email Communication Service to be created.
        pub domain_management: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Email Communication Service where the Domain belongs to. Changing this forces a new Email Communication Service to be created.
        pub email_service_id: pulumi_wasm_rust::Output<String>,
        /// P2 sender domain that is displayed to the email recipients [RFC 5322].
        pub from_sender_domain: pulumi_wasm_rust::Output<String>,
        /// P1 sender domain that is present on the email envelope [RFC 5321].
        pub mail_from_sender_domain: pulumi_wasm_rust::Output<String>,
        /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes user engagement tracking is enabled or disabled. Defaults to `false`.
        pub user_engagement_tracking_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// (Optional) An `verification_records` block as defined below.
        pub verification_records: pulumi_wasm_rust::Output<
            Vec<super::super::types::communication::EmailServiceDomainVerificationRecord>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailServiceDomainArgs) -> EmailServiceDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_management_binding = args.domain_management.get_inner();
        let email_service_id_binding = args.email_service_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_engagement_tracking_enabled_binding = args
            .user_engagement_tracking_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:communication/emailServiceDomain:EmailServiceDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainManagement".into(),
                    value: &domain_management_binding,
                },
                register_interface::ObjectField {
                    name: "emailServiceId".into(),
                    value: &email_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userEngagementTrackingEnabled".into(),
                    value: &user_engagement_tracking_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainManagement".into(),
                },
                register_interface::ResultField {
                    name: "emailServiceId".into(),
                },
                register_interface::ResultField {
                    name: "fromSenderDomain".into(),
                },
                register_interface::ResultField {
                    name: "mailFromSenderDomain".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userEngagementTrackingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "verificationRecords".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailServiceDomainResult {
            domain_management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainManagement").unwrap(),
            ),
            email_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailServiceId").unwrap(),
            ),
            from_sender_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromSenderDomain").unwrap(),
            ),
            mail_from_sender_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mailFromSenderDomain").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_engagement_tracking_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEngagementTrackingEnabled").unwrap(),
            ),
            verification_records: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verificationRecords").unwrap(),
            ),
        }
    }
}
