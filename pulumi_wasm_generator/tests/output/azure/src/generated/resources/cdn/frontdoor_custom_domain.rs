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
///             .name("example-cdn-frontdoor")
///             .build_struct(),
///     );
///     let exampleFrontdoorCustomDomain = frontdoor_custom_domain::create(
///         "exampleFrontdoorCustomDomain",
///         FrontdoorCustomDomainArgs::builder()
///             .cdn_frontdoor_profile_id("${exampleFrontdoorProfile.id}")
///             .dns_zone_id("${exampleZone.id}")
///             .host_name("contoso.fabrikam.com")
///             .name("example-customDomain")
///             .tls(
///                 FrontdoorCustomDomainTls::builder()
///                     .certificateType("ManagedCertificate")
///                     .minimumTlsVersion("TLS12")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleFrontdoorProfile = frontdoor_profile::create(
///         "exampleFrontdoorProfile",
///         FrontdoorProfileArgs::builder()
///             .name("example-profile")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_AzureFrontDoor")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("sub-domain.domain.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example DNS Auth TXT Record Usage
///
/// The name of your DNS TXT record should be in the format of `_dnsauth.<your_subdomain>`. So, for example, if we use the `host_name` in the example usage above you would create a DNS TXT record with the name of `_dnsauth.contoso` which contains the value of the Front Door Custom Domains `validation_token` field. See the [product documentation](https://learn.microsoft.com/azure/frontdoor/standard-premium/how-to-add-custom-domain) for more information.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:dns:TxtRecord
///     properties:
///       name:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - _dnsauth
///               - contoso
///           return: result
///       zoneName: ${exampleAzurermDnsZone.name}
///       resourceGroupName: ${exampleAzurermResourceGroup.name}
///       ttl: 3600
///       records:
///         - value: ${exampleAzurermCdnFrontdoorCustomDomain.validationToken}
/// ```
///
/// ## Example CNAME Record Usage
///
/// !>**IMPORTANT:** You **must** include the `depends_on` meta-argument which references both the `azure.cdn.FrontdoorRoute` and the `azure.cdn.FrontdoorSecurityPolicy` that are associated with your Custom Domain. The reason for these `depends_on` meta-arguments is because all of the resources for the Custom Domain need to be associated within Front Door before the CNAME record can be written to the domains DNS, else the CNAME validation will fail and Front Door will not enable traffic to the Domain.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = c_name_record::create(
///         "example",
///         CNameRecordArgs::builder()
///             .name("contoso")
///             .record("${exampleAzurermCdnFrontdoorEndpoint.hostName}")
///             .resource_group_name("${exampleAzurermResourceGroup.name}")
///             .ttl(3600)
///             .zone_name("${exampleAzurermDnsZone.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Front Door Custom Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorCustomDomain:FrontdoorCustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/customDomains/customDomain1
/// ```
///
pub mod frontdoor_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorCustomDomainArgs {
        /// The ID of the Front Door Profile. Changing this forces a new Front Door Custom Domain to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure DNS Zone which should be used for this Front Door Custom Domain. If you are using Azure to host your [DNS domains](https://learn.microsoft.com/azure/dns/dns-overview), you must delegate the domain provider's domain name system (DNS) to an Azure DNS Zone. For more information, see [Delegate a domain to Azure DNS](https://learn.microsoft.com/azure/dns/dns-delegate-domain-azure-dns). Otherwise, if you're using your own domain provider to handle your DNS, you must validate the Front Door Custom Domain by creating the DNS TXT records manually.
        ///
        /// <!-- * `pre_validated_cdn_frontdoor_custom_domain_id` - (Optional) The resource ID of the pre-validated Front Door Custom Domain. This domain type is used when you wish to onboard a validated Azure service domain, and then configure the Azure service behind an Azure Front Door.
        ///
        /// ->**NOTE:** Currently `pre_validated_cdn_frontdoor_custom_domain_id` only supports domains validated by Static Web App. -->
        #[builder(into, default)]
        pub dns_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The host name of the domain. The `host_name` field must be the FQDN of your domain(e.g. `contoso.fabrikam.com`). Changing this forces a new Front Door Custom Domain to be created.
        #[builder(into)]
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Front Door Custom Domain. Possible values must be between 2 and 260 characters in length, must begin with a letter or number, end with a letter or number and contain only letters, numbers and hyphens. Changing this forces a new Front Door Custom Domain to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `tls` block as defined below.
        #[builder(into)]
        pub tls: pulumi_wasm_rust::Output<
            super::super::types::cdn::FrontdoorCustomDomainTls,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorCustomDomainResult {
        /// The ID of the Front Door Profile. Changing this forces a new Front Door Custom Domain to be created.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure DNS Zone which should be used for this Front Door Custom Domain. If you are using Azure to host your [DNS domains](https://learn.microsoft.com/azure/dns/dns-overview), you must delegate the domain provider's domain name system (DNS) to an Azure DNS Zone. For more information, see [Delegate a domain to Azure DNS](https://learn.microsoft.com/azure/dns/dns-delegate-domain-azure-dns). Otherwise, if you're using your own domain provider to handle your DNS, you must validate the Front Door Custom Domain by creating the DNS TXT records manually.
        ///
        /// <!-- * `pre_validated_cdn_frontdoor_custom_domain_id` - (Optional) The resource ID of the pre-validated Front Door Custom Domain. This domain type is used when you wish to onboard a validated Azure service domain, and then configure the Azure service behind an Azure Front Door.
        ///
        /// ->**NOTE:** Currently `pre_validated_cdn_frontdoor_custom_domain_id` only supports domains validated by Static Web App. -->
        pub dns_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The date time that the token expires.
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The host name of the domain. The `host_name` field must be the FQDN of your domain(e.g. `contoso.fabrikam.com`). Changing this forces a new Front Door Custom Domain to be created.
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Front Door Custom Domain. Possible values must be between 2 and 260 characters in length, must begin with a letter or number, end with a letter or number and contain only letters, numbers and hyphens. Changing this forces a new Front Door Custom Domain to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `tls` block as defined below.
        pub tls: pulumi_wasm_rust::Output<
            super::super::types::cdn::FrontdoorCustomDomainTls,
        >,
        /// Challenge used for DNS TXT record or file based validation.
        pub validation_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FrontdoorCustomDomainArgs,
    ) -> FrontdoorCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_profile_id_binding = args.cdn_frontdoor_profile_id.get_inner();
        let dns_zone_id_binding = args.dns_zone_id.get_inner();
        let host_name_binding = args.host_name.get_inner();
        let name_binding = args.name.get_inner();
        let tls_binding = args.tls.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorCustomDomain:FrontdoorCustomDomain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: &cdn_frontdoor_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsZoneId".into(),
                    value: &dns_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostName".into(),
                    value: &host_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tls".into(),
                    value: &tls_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdnFrontdoorProfileId".into(),
                },
                register_interface::ResultField {
                    name: "dnsZoneId".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "hostName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tls".into(),
                },
                register_interface::ResultField {
                    name: "validationToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrontdoorCustomDomainResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorProfileId").unwrap(),
            ),
            dns_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsZoneId").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tls").unwrap(),
            ),
            validation_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationToken").unwrap(),
            ),
        }
    }
}