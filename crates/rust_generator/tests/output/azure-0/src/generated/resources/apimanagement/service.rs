/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// One or more `additional_location` blocks as defined below.
        #[builder(into, default)]
        pub additional_locations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::ServiceAdditionalLocation>>,
        >,
        /// One or more `certificate` blocks (up to 10) as defined below.
        #[builder(into, default)]
        pub certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::ServiceCertificate>>,
        >,
        /// Enforce a client certificate to be presented on each request to the gateway? This is only supported when SKU type is `Consumption`.
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `delegation` block as defined below.
        #[builder(into, default)]
        pub delegation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceDelegation>,
        >,
        /// Disable the gateway in main region? This is only supported when `additional_location` is set.
        #[builder(into, default)]
        pub gateway_disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `hostname_configuration` block as defined below.
        #[builder(into, default)]
        pub hostname_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceHostnameConfiguration>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceIdentity>,
        >,
        /// The Azure location where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version which the control plane API calls to API Management service are limited with version equal to or newer than.
        #[builder(into, default)]
        pub min_api_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Email address from which the notification will be sent.
        #[builder(into, default)]
        pub notification_sender_email: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `protocols` block as defined below.
        #[builder(into, default)]
        pub protocols: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceProtocols>,
        >,
        /// ID of a standard SKU IPv4 Public IP.
        ///
        /// > **NOTE:** Custom public IPs are only supported on the `Premium` and `Developer` tiers when deployed in a virtual network.
        #[builder(into, default)]
        pub public_ip_address_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is public access to the service allowed? Defaults to `true`.
        ///
        /// > **NOTE:** This option is applicable only to the Management plane, not the API gateway or Developer portal. It is required to be `true` on the creation.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The email of publisher/company.
        #[builder(into)]
        pub publisher_email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of publisher/company.
        #[builder(into)]
        pub publisher_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `security` block as defined below.
        #[builder(into, default)]
        pub security: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceSecurity>,
        >,
        /// A `sign_in` block as defined below.
        #[builder(into, default)]
        pub sign_in: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceSignIn>,
        >,
        /// A `sign_up` block as defined below.
        #[builder(into, default)]
        pub sign_up: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceSignUp>,
        >,
        /// `sku_name` is a string consisting of two parts separated by an underscore(\_). The first part is the `name`, valid values include: `Consumption`, `Developer`, `Basic`, `Standard` and `Premium`. The second part is the `capacity` (e.g. the number of deployed units of the `sku`), which must be a positive `integer` (e.g. `Developer_1`).
        ///
        /// > **NOTE:** Premium SKU's are limited to a default maximum of 12 (i.e. `Premium_12`), this can, however, be increased via support request.
        ///
        /// > **NOTE:** Consumption SKU capacity should be 0 (e.g. `Consumption_0`) as this tier includes automatic scaling.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        #[builder(into, default)]
        pub tenant_access: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ServiceTenantAccess>,
        >,
        /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
        #[builder(into, default)]
        pub virtual_network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::apimanagement::ServiceVirtualNetworkConfiguration,
            >,
        >,
        /// The type of virtual network you want to use, valid values include: `None`, `External`, `Internal`. Defaults to `None`.
        ///
        /// > **NOTE:** Please ensure that in the subnet, inbound port 3443 is open when `virtual_network_type` is `Internal` or `External`. And please ensure other necessary ports are open according to [api management network configuration](https://learn.microsoft.com/azure/api-management/virtual-network-reference).
        #[builder(into, default)]
        pub virtual_network_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Availability Zones in which this API Management service should be located.
        ///
        /// > **NOTE:** Availability zones are only supported in the Premium tier.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// One or more `additional_location` blocks as defined below.
        pub additional_locations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceAdditionalLocation>>,
        >,
        /// One or more `certificate` blocks (up to 10) as defined below.
        pub certificates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceCertificate>>,
        >,
        /// Enforce a client certificate to be presented on each request to the gateway? This is only supported when SKU type is `Consumption`.
        pub client_certificate_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `delegation` block as defined below.
        pub delegation: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceDelegation,
        >,
        /// The URL for the Developer Portal associated with this API Management service.
        pub developer_portal_url: pulumi_gestalt_rust::Output<String>,
        /// Disable the gateway in main region? This is only supported when `additional_location` is set.
        pub gateway_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The URL of the Regional Gateway for the API Management Service in the specified region.
        pub gateway_regional_url: pulumi_gestalt_rust::Output<String>,
        /// The URL of the Gateway for the API Management Service.
        pub gateway_url: pulumi_gestalt_rust::Output<String>,
        /// A `hostname_configuration` block as defined below.
        pub hostname_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceHostnameConfiguration,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ServiceIdentity>,
        >,
        /// The Azure location where the API Management Service exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The URL for the Management API associated with this API Management service.
        pub management_api_url: pulumi_gestalt_rust::Output<String>,
        /// The version which the control plane API calls to API Management service are limited with version equal to or newer than.
        pub min_api_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Email address from which the notification will be sent.
        pub notification_sender_email: pulumi_gestalt_rust::Output<String>,
        /// The URL for the Publisher Portal associated with this API Management service.
        pub portal_url: pulumi_gestalt_rust::Output<String>,
        /// The Private IP addresses of the API Management Service. Available only when the API Manager instance is using Virtual Network mode.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `protocols` block as defined below.
        pub protocols: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceProtocols,
        >,
        /// ID of a standard SKU IPv4 Public IP.
        ///
        /// > **NOTE:** Custom public IPs are only supported on the `Premium` and `Developer` tiers when deployed in a virtual network.
        pub public_ip_address_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
        pub public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Is public access to the service allowed? Defaults to `true`.
        ///
        /// > **NOTE:** This option is applicable only to the Management plane, not the API gateway or Developer portal. It is required to be `true` on the creation.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The email of publisher/company.
        pub publisher_email: pulumi_gestalt_rust::Output<String>,
        /// The name of publisher/company.
        pub publisher_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The URL for the SCM (Source Code Management) Endpoint associated with this API Management service.
        pub scm_url: pulumi_gestalt_rust::Output<String>,
        /// A `security` block as defined below.
        pub security: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceSecurity,
        >,
        /// A `sign_in` block as defined below.
        pub sign_in: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceSignIn,
        >,
        /// A `sign_up` block as defined below.
        pub sign_up: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceSignUp,
        >,
        /// `sku_name` is a string consisting of two parts separated by an underscore(\_). The first part is the `name`, valid values include: `Consumption`, `Developer`, `Basic`, `Standard` and `Premium`. The second part is the `capacity` (e.g. the number of deployed units of the `sku`), which must be a positive `integer` (e.g. `Developer_1`).
        ///
        /// > **NOTE:** Premium SKU's are limited to a default maximum of 12 (i.e. `Premium_12`), this can, however, be increased via support request.
        ///
        /// > **NOTE:** Consumption SKU capacity should be 0 (e.g. `Consumption_0`) as this tier includes automatic scaling.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        pub tenant_access: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ServiceTenantAccess,
        >,
        /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
        pub virtual_network_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::apimanagement::ServiceVirtualNetworkConfiguration,
            >,
        >,
        /// The type of virtual network you want to use, valid values include: `None`, `External`, `Internal`. Defaults to `None`.
        ///
        /// > **NOTE:** Please ensure that in the subnet, inbound port 3443 is open when `virtual_network_type` is `Internal` or `External`. And please ensure other necessary ports are open according to [api management network configuration](https://learn.microsoft.com/azure/api-management/virtual-network-reference).
        pub virtual_network_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this API Management service should be located.
        ///
        /// > **NOTE:** Availability zones are only supported in the Premium tier.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_locations_binding_1 = args
            .additional_locations
            .get_output(context);
        let additional_locations_binding = additional_locations_binding_1.get_inner();
        let certificates_binding_1 = args.certificates.get_output(context);
        let certificates_binding = certificates_binding_1.get_inner();
        let client_certificate_enabled_binding_1 = args
            .client_certificate_enabled
            .get_output(context);
        let client_certificate_enabled_binding = client_certificate_enabled_binding_1
            .get_inner();
        let delegation_binding_1 = args.delegation.get_output(context);
        let delegation_binding = delegation_binding_1.get_inner();
        let gateway_disabled_binding_1 = args.gateway_disabled.get_output(context);
        let gateway_disabled_binding = gateway_disabled_binding_1.get_inner();
        let hostname_configuration_binding_1 = args
            .hostname_configuration
            .get_output(context);
        let hostname_configuration_binding = hostname_configuration_binding_1
            .get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let min_api_version_binding_1 = args.min_api_version.get_output(context);
        let min_api_version_binding = min_api_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let notification_sender_email_binding_1 = args
            .notification_sender_email
            .get_output(context);
        let notification_sender_email_binding = notification_sender_email_binding_1
            .get_inner();
        let protocols_binding_1 = args.protocols.get_output(context);
        let protocols_binding = protocols_binding_1.get_inner();
        let public_ip_address_id_binding_1 = args
            .public_ip_address_id
            .get_output(context);
        let public_ip_address_id_binding = public_ip_address_id_binding_1.get_inner();
        let public_network_access_enabled_binding_1 = args
            .public_network_access_enabled
            .get_output(context);
        let public_network_access_enabled_binding = public_network_access_enabled_binding_1
            .get_inner();
        let publisher_email_binding_1 = args.publisher_email.get_output(context);
        let publisher_email_binding = publisher_email_binding_1.get_inner();
        let publisher_name_binding_1 = args.publisher_name.get_output(context);
        let publisher_name_binding = publisher_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let security_binding_1 = args.security.get_output(context);
        let security_binding = security_binding_1.get_inner();
        let sign_in_binding_1 = args.sign_in.get_output(context);
        let sign_in_binding = sign_in_binding_1.get_inner();
        let sign_up_binding_1 = args.sign_up.get_output(context);
        let sign_up_binding = sign_up_binding_1.get_inner();
        let sku_name_binding_1 = args.sku_name.get_output(context);
        let sku_name_binding = sku_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let tenant_access_binding_1 = args.tenant_access.get_output(context);
        let tenant_access_binding = tenant_access_binding_1.get_inner();
        let virtual_network_configuration_binding_1 = args
            .virtual_network_configuration
            .get_output(context);
        let virtual_network_configuration_binding = virtual_network_configuration_binding_1
            .get_inner();
        let virtual_network_type_binding_1 = args
            .virtual_network_type
            .get_output(context);
        let virtual_network_type_binding = virtual_network_type_binding_1.get_inner();
        let zones_binding_1 = args.zones.get_output(context);
        let zones_binding = zones_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalLocations".into(),
                    value: &additional_locations_binding,
                },
                register_interface::ObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateEnabled".into(),
                    value: &client_certificate_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "delegation".into(),
                    value: &delegation_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayDisabled".into(),
                    value: &gateway_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "hostnameConfiguration".into(),
                    value: &hostname_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minApiVersion".into(),
                    value: &min_api_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationSenderEmail".into(),
                    value: &notification_sender_email_binding,
                },
                register_interface::ObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "publisherEmail".into(),
                    value: &publisher_email_binding,
                },
                register_interface::ObjectField {
                    name: "publisherName".into(),
                    value: &publisher_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "security".into(),
                    value: &security_binding,
                },
                register_interface::ObjectField {
                    name: "signIn".into(),
                    value: &sign_in_binding,
                },
                register_interface::ObjectField {
                    name: "signUp".into(),
                    value: &sign_up_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenantAccess".into(),
                    value: &tenant_access_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkConfiguration".into(),
                    value: &virtual_network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkType".into(),
                    value: &virtual_network_type_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            additional_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalLocations"),
            ),
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            client_certificate_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertificateEnabled"),
            ),
            delegation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delegation"),
            ),
            developer_portal_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("developerPortalUrl"),
            ),
            gateway_disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayDisabled"),
            ),
            gateway_regional_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayRegionalUrl"),
            ),
            gateway_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayUrl"),
            ),
            hostname_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostnameConfiguration"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_api_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementApiUrl"),
            ),
            min_api_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minApiVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            notification_sender_email: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationSenderEmail"),
            ),
            portal_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portalUrl"),
            ),
            private_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            public_ip_address_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddressId"),
            ),
            public_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddresses"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            publisher_email: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publisherEmail"),
            ),
            publisher_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publisherName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scm_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scmUrl"),
            ),
            security: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("security"),
            ),
            sign_in: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signIn"),
            ),
            sign_up: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signUp"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantAccess"),
            ),
            virtual_network_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkConfiguration"),
            ),
            virtual_network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkType"),
            ),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
