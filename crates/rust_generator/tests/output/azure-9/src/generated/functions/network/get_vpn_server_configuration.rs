#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpn_server_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnServerConfigurationArgs {
        /// The Name of the VPN Server Configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the VPN Server Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpnServerConfigurationResult {
        /// A `azure_active_directory_authentication` block as defined below.
        pub azure_active_directory_authentications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationAzureActiveDirectoryAuthentication,
            >,
        >,
        /// One or more `client_revoked_certificate` blocks as defined below.
        pub client_revoked_certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationClientRevokedCertificate,
            >,
        >,
        /// One or more `client_root_certificate` blocks as defined below.
        pub client_root_certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationClientRootCertificate,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The `bgp_settings` block as defined below.
        pub ipsec_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationIpsecPolicy,
            >,
        >,
        /// The Azure Region where the VPN Server Configuration exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name used to uniquely identify this certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `radius` block as defined below.
        pub radii: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVpnServerConfigurationRadius>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the VPN Server Configuration.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of Authentication Types applicable for the VPN Server Configuration.
        pub vpn_authentication_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of VPN Protocols to use for the VPN Server Configuration.
        pub vpn_protocols: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpnServerConfigurationArgs,
    ) -> GetVpnServerConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVpnServerConfiguration:getVpnServerConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpnServerConfigurationResult {
            azure_active_directory_authentications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureActiveDirectoryAuthentications"),
            ),
            client_revoked_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientRevokedCertificates"),
            ),
            client_root_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientRootCertificates"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipsec_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipsecPolicies"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            radii: pulumi_gestalt_rust::__private::into_domain(o.extract_field("radii")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vpn_authentication_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnAuthenticationTypes"),
            ),
            vpn_protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnProtocols"),
            ),
        }
    }
}
