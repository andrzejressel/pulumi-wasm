#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_router_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub router_type_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Direct Connect Virtual Interface
        #[builder(into)]
        pub virtual_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouterConfigurationResult {
        /// Instructions for configuring your router
        pub customer_router_config: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Router type identifier
        pub router_type_identifier: pulumi_gestalt_rust::Output<String>,
        /// Block of the router type details
        pub routers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::directconnect::GetRouterConfigurationRouter>,
        >,
        pub virtual_interface_id: pulumi_gestalt_rust::Output<String>,
        pub virtual_interface_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRouterConfigurationArgs,
    ) -> GetRouterConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let router_type_identifier_binding = args
            .router_type_identifier
            .get_output(context);
        let virtual_interface_id_binding = args.virtual_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directconnect/getRouterConfiguration:getRouterConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routerTypeIdentifier".into(),
                    value: router_type_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualInterfaceId".into(),
                    value: virtual_interface_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRouterConfigurationResult {
            customer_router_config: o.get_field("customerRouterConfig"),
            id: o.get_field("id"),
            router_type_identifier: o.get_field("routerTypeIdentifier"),
            routers: o.get_field("routers"),
            virtual_interface_id: o.get_field("virtualInterfaceId"),
            virtual_interface_name: o.get_field("virtualInterfaceName"),
        }
    }
}
