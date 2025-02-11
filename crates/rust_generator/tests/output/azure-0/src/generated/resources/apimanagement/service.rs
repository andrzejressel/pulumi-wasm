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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_locations_binding = args.additional_locations.get_output(context);
        let certificates_binding = args.certificates.get_output(context);
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_output(context);
        let delegation_binding = args.delegation.get_output(context);
        let gateway_disabled_binding = args.gateway_disabled.get_output(context);
        let hostname_configuration_binding = args
            .hostname_configuration
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let min_api_version_binding = args.min_api_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_sender_email_binding = args
            .notification_sender_email
            .get_output(context);
        let protocols_binding = args.protocols.get_output(context);
        let public_ip_address_id_binding = args.public_ip_address_id.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let publisher_email_binding = args.publisher_email.get_output(context);
        let publisher_name_binding = args.publisher_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let security_binding = args.security.get_output(context);
        let sign_in_binding = args.sign_in.get_output(context);
        let sign_up_binding = args.sign_up.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenant_access_binding = args.tenant_access.get_output(context);
        let virtual_network_configuration_binding = args
            .virtual_network_configuration
            .get_output(context);
        let virtual_network_type_binding = args.virtual_network_type.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalLocations".into(),
                    value: &additional_locations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateEnabled".into(),
                    value: &client_certificate_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegation".into(),
                    value: &delegation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayDisabled".into(),
                    value: &gateway_disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostnameConfiguration".into(),
                    value: &hostname_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minApiVersion".into(),
                    value: &min_api_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationSenderEmail".into(),
                    value: &notification_sender_email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisherEmail".into(),
                    value: &publisher_email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisherName".into(),
                    value: &publisher_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "security".into(),
                    value: &security_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signIn".into(),
                    value: &sign_in_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signUp".into(),
                    value: &sign_up_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantAccess".into(),
                    value: &tenant_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkConfiguration".into(),
                    value: &virtual_network_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkType".into(),
                    value: &virtual_network_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            additional_locations: o.get_field("additionalLocations"),
            certificates: o.get_field("certificates"),
            client_certificate_enabled: o.get_field("clientCertificateEnabled"),
            delegation: o.get_field("delegation"),
            developer_portal_url: o.get_field("developerPortalUrl"),
            gateway_disabled: o.get_field("gatewayDisabled"),
            gateway_regional_url: o.get_field("gatewayRegionalUrl"),
            gateway_url: o.get_field("gatewayUrl"),
            hostname_configuration: o.get_field("hostnameConfiguration"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            management_api_url: o.get_field("managementApiUrl"),
            min_api_version: o.get_field("minApiVersion"),
            name: o.get_field("name"),
            notification_sender_email: o.get_field("notificationSenderEmail"),
            portal_url: o.get_field("portalUrl"),
            private_ip_addresses: o.get_field("privateIpAddresses"),
            protocols: o.get_field("protocols"),
            public_ip_address_id: o.get_field("publicIpAddressId"),
            public_ip_addresses: o.get_field("publicIpAddresses"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            publisher_email: o.get_field("publisherEmail"),
            publisher_name: o.get_field("publisherName"),
            resource_group_name: o.get_field("resourceGroupName"),
            scm_url: o.get_field("scmUrl"),
            security: o.get_field("security"),
            sign_in: o.get_field("signIn"),
            sign_up: o.get_field("signUp"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            tenant_access: o.get_field("tenantAccess"),
            virtual_network_configuration: o.get_field("virtualNetworkConfiguration"),
            virtual_network_type: o.get_field("virtualNetworkType"),
            zones: o.get_field("zones"),
        }
    }
}
