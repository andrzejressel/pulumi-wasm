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
        context: &pulumi_gestalt_rust::Context,
        args: GetVpnServerConfigurationArgs,
    ) -> GetVpnServerConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVpnServerConfiguration:getVpnServerConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpnServerConfigurationResult {
            azure_active_directory_authentications: o
                .get_field("azureActiveDirectoryAuthentications"),
            client_revoked_certificates: o.get_field("clientRevokedCertificates"),
            client_root_certificates: o.get_field("clientRootCertificates"),
            id: o.get_field("id"),
            ipsec_policies: o.get_field("ipsecPolicies"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            radii: o.get_field("radii"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            vpn_authentication_types: o.get_field("vpnAuthenticationTypes"),
            vpn_protocols: o.get_field("vpnProtocols"),
        }
    }
}
