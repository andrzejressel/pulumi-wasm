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
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// One or more `additional_location` blocks as defined below.
        #[builder(into, default)]
        pub additional_locations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceAdditionalLocation>>,
        >,
        /// One or more `certificate` blocks (up to 10) as defined below.
        #[builder(into, default)]
        pub certificates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceCertificate>>,
        >,
        /// Enforce a client certificate to be presented on each request to the gateway? This is only supported when SKU type is `Consumption`.
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `delegation` block as defined below.
        #[builder(into, default)]
        pub delegation: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceDelegation>,
        >,
        /// Disable the gateway in main region? This is only supported when `additional_location` is set.
        #[builder(into, default)]
        pub gateway_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `hostname_configuration` block as defined below.
        #[builder(into, default)]
        pub hostname_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceHostnameConfiguration>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceIdentity>,
        >,
        /// The Azure location where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The version which the control plane API calls to API Management service are limited with version equal to or newer than.
        #[builder(into, default)]
        pub min_api_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Email address from which the notification will be sent.
        #[builder(into, default)]
        pub notification_sender_email: pulumi_wasm_rust::Output<Option<String>>,
        /// A `protocols` block as defined below.
        #[builder(into, default)]
        pub protocols: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceProtocols>,
        >,
        /// ID of a standard SKU IPv4 Public IP.
        ///
        /// > **NOTE:** Custom public IPs are only supported on the `Premium` and `Developer` tiers when deployed in a virtual network.
        #[builder(into, default)]
        pub public_ip_address_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is public access to the service allowed? Defaults to `true`.
        ///
        /// > **NOTE:** This option is applicable only to the Management plane, not the API gateway or Developer portal. It is required to be `true` on the creation.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email of publisher/company.
        #[builder(into)]
        pub publisher_email: pulumi_wasm_rust::Output<String>,
        /// The name of publisher/company.
        #[builder(into)]
        pub publisher_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `security` block as defined below.
        #[builder(into, default)]
        pub security: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceSecurity>,
        >,
        /// A `sign_in` block as defined below.
        #[builder(into, default)]
        pub sign_in: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceSignIn>,
        >,
        /// A `sign_up` block as defined below.
        #[builder(into, default)]
        pub sign_up: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceSignUp>,
        >,
        /// `sku_name` is a string consisting of two parts separated by an underscore(\_). The first part is the `name`, valid values include: `Consumption`, `Developer`, `Basic`, `Standard` and `Premium`. The second part is the `capacity` (e.g. the number of deployed units of the `sku`), which must be a positive `integer` (e.g. `Developer_1`).
        ///
        /// > **NOTE:** Premium SKU's are limited to a default maximum of 12 (i.e. `Premium_12`), this can, however, be increased via support request.
        ///
        /// > **NOTE:** Consumption SKU capacity should be 0 (e.g. `Consumption_0`) as this tier includes automatic scaling.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        #[builder(into, default)]
        pub tenant_access: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceTenantAccess>,
        >,
        /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
        #[builder(into, default)]
        pub virtual_network_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apimanagement::ServiceVirtualNetworkConfiguration,
            >,
        >,
        /// The type of virtual network you want to use, valid values include: `None`, `External`, `Internal`. Defaults to `None`.
        ///
        /// > **NOTE:** Please ensure that in the subnet, inbound port 3443 is open when `virtual_network_type` is `Internal` or `External`. And please ensure other necessary ports are open according to [api management network configuration](https://learn.microsoft.com/azure/api-management/virtual-network-reference).
        #[builder(into, default)]
        pub virtual_network_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this API Management service should be located.
        ///
        /// > **NOTE:** Availability zones are only supported in the Premium tier.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// One or more `additional_location` blocks as defined below.
        pub additional_locations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceAdditionalLocation>>,
        >,
        /// One or more `certificate` blocks (up to 10) as defined below.
        pub certificates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::ServiceCertificate>>,
        >,
        /// Enforce a client certificate to be presented on each request to the gateway? This is only supported when SKU type is `Consumption`.
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `delegation` block as defined below.
        pub delegation: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceDelegation,
        >,
        /// The URL for the Developer Portal associated with this API Management service.
        pub developer_portal_url: pulumi_wasm_rust::Output<String>,
        /// Disable the gateway in main region? This is only supported when `additional_location` is set.
        pub gateway_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The URL of the Regional Gateway for the API Management Service in the specified region.
        pub gateway_regional_url: pulumi_wasm_rust::Output<String>,
        /// The URL of the Gateway for the API Management Service.
        pub gateway_url: pulumi_wasm_rust::Output<String>,
        /// A `hostname_configuration` block as defined below.
        pub hostname_configuration: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceHostnameConfiguration,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ServiceIdentity>,
        >,
        /// The Azure location where the API Management Service exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The URL for the Management API associated with this API Management service.
        pub management_api_url: pulumi_wasm_rust::Output<String>,
        /// The version which the control plane API calls to API Management service are limited with version equal to or newer than.
        pub min_api_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Email address from which the notification will be sent.
        pub notification_sender_email: pulumi_wasm_rust::Output<String>,
        /// The URL for the Publisher Portal associated with this API Management service.
        pub portal_url: pulumi_wasm_rust::Output<String>,
        /// The Private IP addresses of the API Management Service. Available only when the API Manager instance is using Virtual Network mode.
        pub private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `protocols` block as defined below.
        pub protocols: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceProtocols,
        >,
        /// ID of a standard SKU IPv4 Public IP.
        ///
        /// > **NOTE:** Custom public IPs are only supported on the `Premium` and `Developer` tiers when deployed in a virtual network.
        pub public_ip_address_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
        pub public_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Is public access to the service allowed? Defaults to `true`.
        ///
        /// > **NOTE:** This option is applicable only to the Management plane, not the API gateway or Developer portal. It is required to be `true` on the creation.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email of publisher/company.
        pub publisher_email: pulumi_wasm_rust::Output<String>,
        /// The name of publisher/company.
        pub publisher_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The URL for the SCM (Source Code Management) Endpoint associated with this API Management service.
        pub scm_url: pulumi_wasm_rust::Output<String>,
        /// A `security` block as defined below.
        pub security: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceSecurity,
        >,
        /// A `sign_in` block as defined below.
        pub sign_in: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceSignIn,
        >,
        /// A `sign_up` block as defined below.
        pub sign_up: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceSignUp,
        >,
        /// `sku_name` is a string consisting of two parts separated by an underscore(\_). The first part is the `name`, valid values include: `Consumption`, `Developer`, `Basic`, `Standard` and `Premium`. The second part is the `capacity` (e.g. the number of deployed units of the `sku`), which must be a positive `integer` (e.g. `Developer_1`).
        ///
        /// > **NOTE:** Premium SKU's are limited to a default maximum of 12 (i.e. `Premium_12`), this can, however, be increased via support request.
        ///
        /// > **NOTE:** Consumption SKU capacity should be 0 (e.g. `Consumption_0`) as this tier includes automatic scaling.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        pub tenant_access: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ServiceTenantAccess,
        >,
        /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
        pub virtual_network_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apimanagement::ServiceVirtualNetworkConfiguration,
            >,
        >,
        /// The type of virtual network you want to use, valid values include: `None`, `External`, `Internal`. Defaults to `None`.
        ///
        /// > **NOTE:** Please ensure that in the subnet, inbound port 3443 is open when `virtual_network_type` is `Internal` or `External`. And please ensure other necessary ports are open according to [api management network configuration](https://learn.microsoft.com/azure/api-management/virtual-network-reference).
        pub virtual_network_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this API Management service should be located.
        ///
        /// > **NOTE:** Availability zones are only supported in the Premium tier.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_locations_binding = args.additional_locations.get_inner();
        let certificates_binding = args.certificates.get_inner();
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_inner();
        let delegation_binding = args.delegation.get_inner();
        let gateway_disabled_binding = args.gateway_disabled.get_inner();
        let hostname_configuration_binding = args.hostname_configuration.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let min_api_version_binding = args.min_api_version.get_inner();
        let name_binding = args.name.get_inner();
        let notification_sender_email_binding = args
            .notification_sender_email
            .get_inner();
        let protocols_binding = args.protocols.get_inner();
        let public_ip_address_id_binding = args.public_ip_address_id.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let publisher_email_binding = args.publisher_email.get_inner();
        let publisher_name_binding = args.publisher_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let security_binding = args.security.get_inner();
        let sign_in_binding = args.sign_in.get_inner();
        let sign_up_binding = args.sign_up.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let tenant_access_binding = args.tenant_access.get_inner();
        let virtual_network_configuration_binding = args
            .virtual_network_configuration
            .get_inner();
        let virtual_network_type_binding = args.virtual_network_type.get_inner();
        let zones_binding = args.zones.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalLocations".into(),
                },
                register_interface::ResultField {
                    name: "certificates".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateEnabled".into(),
                },
                register_interface::ResultField {
                    name: "delegation".into(),
                },
                register_interface::ResultField {
                    name: "developerPortalUrl".into(),
                },
                register_interface::ResultField {
                    name: "gatewayDisabled".into(),
                },
                register_interface::ResultField {
                    name: "gatewayRegionalUrl".into(),
                },
                register_interface::ResultField {
                    name: "gatewayUrl".into(),
                },
                register_interface::ResultField {
                    name: "hostnameConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementApiUrl".into(),
                },
                register_interface::ResultField {
                    name: "minApiVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationSenderEmail".into(),
                },
                register_interface::ResultField {
                    name: "portalUrl".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddressId".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "publisherEmail".into(),
                },
                register_interface::ResultField {
                    name: "publisherName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scmUrl".into(),
                },
                register_interface::ResultField {
                    name: "security".into(),
                },
                register_interface::ResultField {
                    name: "signIn".into(),
                },
                register_interface::ResultField {
                    name: "signUp".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenantAccess".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkType".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceResult {
            additional_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalLocations").unwrap(),
            ),
            certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificates").unwrap(),
            ),
            client_certificate_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateEnabled").unwrap(),
            ),
            delegation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegation").unwrap(),
            ),
            developer_portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("developerPortalUrl").unwrap(),
            ),
            gateway_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayDisabled").unwrap(),
            ),
            gateway_regional_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayRegionalUrl").unwrap(),
            ),
            gateway_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayUrl").unwrap(),
            ),
            hostname_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnameConfiguration").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_api_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementApiUrl").unwrap(),
            ),
            min_api_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minApiVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_sender_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationSenderEmail").unwrap(),
            ),
            portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portalUrl").unwrap(),
            ),
            private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddresses").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            public_ip_address_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddressId").unwrap(),
            ),
            public_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddresses").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            publisher_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisherEmail").unwrap(),
            ),
            publisher_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisherName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scm_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scmUrl").unwrap(),
            ),
            security: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("security").unwrap(),
            ),
            sign_in: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signIn").unwrap(),
            ),
            sign_up: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signUp").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenant_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantAccess").unwrap(),
            ),
            virtual_network_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkConfiguration").unwrap(),
            ),
            virtual_network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkType").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
