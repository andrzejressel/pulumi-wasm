pub mod get_router_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouterConfigurationArgs {
        /// ID of the Router Type. For example: `CiscoSystemsInc-2900SeriesRouters-IOS124`
        ///
        /// There is currently no AWS API to retrieve the full list of `router_type_identifier` values. Here is a list of known `RouterType` objects that can be used:
        ///
        /// ```json
        /// {
        /// "routerTypes": [
        /// {"platform":"2900 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-2900SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"3700 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-3700SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"7200 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-7200SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"Nexus 7000 Series Switches","routerTypeIdentifier":"CiscoSystemsInc-Nexus7000SeriesSwitches-NXOS51","software":"NX-OS 5.1+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-switch-cisco-nexus-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"Nexus 9K+ Series Switches","routerTypeIdentifier":"CiscoSystemsInc-Nexus9KSeriesSwitches-NXOS93","software":"NX-OS 9.3+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-switch-cisco-nexus-generic.xslt","xsltTemplateNameForMacSec":"customer-switch-cisco-nexus-generic-macsec.xslt"},
        /// {"platform":"M/MX Series Routers","routerTypeIdentifier":"JuniperNetworksInc-MMXSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":"customer-router-juniper-generic-macsec.xslt"},
        /// {"platform":"SRX Series Routers","routerTypeIdentifier":"JuniperNetworksInc-SRXSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"T Series Routers","routerTypeIdentifier":"JuniperNetworksInc-TSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":""},
        /// {"platform":"PA-3000+ and 5000+ series","routerTypeIdentifier":"PaloAltoNetworks-PA3000and5000series-PANOS803","software":"PAN-OS 8.0.3+","vendor":"Palo Alto Networks","xsltTemplateName":"customer-router-palo-alto-generic.xslt","xsltTemplateNameForMacSec":""}]
        /// }
        /// ```
        #[builder(into)]
        pub router_type_identifier: pulumi_wasm_rust::Output<String>,
        /// ID of the Direct Connect Virtual Interface
        #[builder(into)]
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouterConfigurationResult {
        /// Instructions for configuring your router
        pub customer_router_config: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Router type identifier
        pub router_type_identifier: pulumi_wasm_rust::Output<String>,
        /// Block of the router type details
        pub routers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::directconnect::GetRouterConfigurationRouter>,
        >,
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
        pub virtual_interface_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRouterConfigurationArgs) -> GetRouterConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let router_type_identifier_binding = args.router_type_identifier.get_inner();
        let virtual_interface_id_binding = args.virtual_interface_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:directconnect/getRouterConfiguration:getRouterConfiguration"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "routerTypeIdentifier".into(),
                    value: &router_type_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "virtualInterfaceId".into(),
                    value: &virtual_interface_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customerRouterConfig".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "routerTypeIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "routers".into(),
                },
                register_interface::ResultField {
                    name: "virtualInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "virtualInterfaceName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRouterConfigurationResult {
            customer_router_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerRouterConfig").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            router_type_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routerTypeIdentifier").unwrap(),
            ),
            routers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routers").unwrap(),
            ),
            virtual_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualInterfaceId").unwrap(),
            ),
            virtual_interface_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualInterfaceName").unwrap(),
            ),
        }
    }
}
