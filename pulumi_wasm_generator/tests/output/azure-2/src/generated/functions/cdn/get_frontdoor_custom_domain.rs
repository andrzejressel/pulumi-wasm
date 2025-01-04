pub mod get_frontdoor_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorCustomDomainArgs {
        /// The name of the Front Door Custom Domain.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Front Door Profile which the Front Door Custom Domain is bound to.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorCustomDomainResult {
        /// The ID of the Front Door Profile which the Front Door Custom Domain is bound to.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        pub dns_zone_id: pulumi_wasm_rust::Output<String>,
        /// The date time that the token expires.
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The host name of the domain.
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub profile_name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `tls` block as defined below.
        pub tls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorCustomDomainTl>,
        >,
        /// The challenge used for DNS TXT record or file based validation.
        pub validation_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFrontdoorCustomDomainArgs) -> GetFrontdoorCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let profile_name_binding = args.profile_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorCustomDomain:getFrontdoorCustomDomain".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "profileName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tls".into(),
                },
                register_interface::ResultField {
                    name: "validationToken".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFrontdoorCustomDomainResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
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
