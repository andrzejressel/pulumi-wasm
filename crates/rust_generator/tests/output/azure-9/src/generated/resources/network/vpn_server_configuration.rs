/// Manages a VPN Server Configuration.
///
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
///     let test = vpn_server_configuration::create(
///         "test",
///         VpnServerConfigurationArgs::builder()
///             .client_root_certificates(
///                 vec![
///                     VpnServerConfigurationClientRootCertificate::builder()
///                     .name("DigiCert-Federated-ID-Root-CA")
///                     .publicCertData("MIIDuzCCAqOgAwIBAgIQCHTZWCM+IlfFIRXIvyKSrjANBgkqhkiG9w0BAQsFADBn\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSYwJAYDVQQDEx1EaWdpQ2VydCBGZWRlcmF0ZWQgSUQg\nUm9vdCBDQTAeFw0xMzAxMTUxMjAwMDBaFw0zMzAxMTUxMjAwMDBaMGcxCzAJBgNV\nBAYTAlVTMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdp\nY2VydC5jb20xJjAkBgNVBAMTHURpZ2lDZXJ0IEZlZGVyYXRlZCBJRCBSb290IENB\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvAEB4pcCqnNNOWE6Ur5j\nQPUH+1y1F9KdHTRSza6k5iDlXq1kGS1qAkuKtw9JsiNRrjltmFnzMZRBbX8Tlfl8\nzAhBmb6dDduDGED01kBsTkgywYPxXVTKec0WxYEEF0oMn4wSYNl0lt2eJAKHXjNf\nGTwiibdP8CUR2ghSM2sUTI8Nt1Omfc4SMHhGhYD64uJMbX98THQ/4LMGuYegou+d\nGTiahfHtjn7AboSEknwAMJHCh5RlYZZ6B1O4QbKJ+34Q0eKgnI3X6Vc9u0zf6DH8\nDk+4zQDYRRTqTnVO3VT8jzqDlCRuNtq6YvryOWN74/dq8LQhUnXHvFyrsdMaE1X2\nDwIDAQABo2MwYTAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV\nHQ4EFgQUGRdkFnbGt1EWjKwbUne+5OaZvRYwHwYDVR0jBBgwFoAUGRdkFnbGt1EW\njKwbUne+5OaZvRYwDQYJKoZIhvcNAQELBQADggEBAHcqsHkrjpESqfuVTRiptJfP\n9JbdtWqRTmOf6uJi2c8YVqI6XlKXsD8C1dUUaaHKLUJzvKiazibVuBwMIT84AyqR\nQELn3e0BtgEymEygMU569b01ZPxoFSnNXc7qDZBDef8WfqAV/sxkTi8L9BkmFYfL\nuGLOhRJOFprPdoDIUBB+tmCl3oDcBy3vnUeOEioz8zAkprcb3GHwHAK+vHmmfgcn\nWsfMLH4JCLa/tRYL+Rw/N3ybCkDp00s0WUZ+AoDywSl0Q/ZEnNY0MsFiw6LyIdbq\nM/s/1JRtO3bDSzD9TazRVzn2oBqzSa8VgIo5C1nOnoAKJTlsClJKvIhnRlaLQqk=")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-config")
///             .resource_group_name("${example.name}")
///             .vpn_authentication_types(vec!["Certificate",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Server Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vpnServerConfiguration:VpnServerConfiguration config1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/vpnServerConfigurations/config1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_server_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnServerConfigurationArgs {
        #[builder(into, default)]
        pub azure_active_directory_authentications: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationAzureActiveDirectoryAuthentication,
                >,
            >,
        >,
        #[builder(into, default)]
        pub client_revoked_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationClientRevokedCertificate,
                >,
            >,
        >,
        #[builder(into, default)]
        pub client_root_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationClientRootCertificate,
                >,
            >,
        >,
        /// A `ipsec_policy` block as defined below.
        #[builder(into, default)]
        pub ipsec_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VpnServerConfigurationIpsecPolicy>,
        >,
        /// The Azure location where this VPN Server Configuration should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name which should be used for this VPN Server Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub radius: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VpnServerConfigurationRadius>,
        >,
        /// The Name of the Resource Group in which this VPN Server Configuration should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Authentication Types applicable for this VPN Server Configuration. Possible values are `AAD` (Azure Active Directory), `Certificate` and `Radius`.
        #[builder(into)]
        pub vpn_authentication_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A list of VPN Protocols to use for this Server Configuration. Possible values are `IkeV2` and `OpenVPN`.
        #[builder(into, default)]
        pub vpn_protocols: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct VpnServerConfigurationResult {
        pub azure_active_directory_authentications: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationAzureActiveDirectoryAuthentication,
                >,
            >,
        >,
        pub client_revoked_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationClientRevokedCertificate,
                >,
            >,
        >,
        pub client_root_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::VpnServerConfigurationClientRootCertificate,
                >,
            >,
        >,
        /// A `ipsec_policy` block as defined below.
        pub ipsec_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::VpnServerConfigurationIpsecPolicy>,
        >,
        /// The Azure location where this VPN Server Configuration should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this VPN Server Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub radius: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::VpnServerConfigurationRadius>,
        >,
        /// The Name of the Resource Group in which this VPN Server Configuration should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Authentication Types applicable for this VPN Server Configuration. Possible values are `AAD` (Azure Active Directory), `Certificate` and `Radius`.
        pub vpn_authentication_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of VPN Protocols to use for this Server Configuration. Possible values are `IkeV2` and `OpenVPN`.
        pub vpn_protocols: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnServerConfigurationArgs,
    ) -> VpnServerConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let azure_active_directory_authentications_binding = args
            .azure_active_directory_authentications
            .get_output(context);
        let client_revoked_certificates_binding = args
            .client_revoked_certificates
            .get_output(context);
        let client_root_certificates_binding = args
            .client_root_certificates
            .get_output(context);
        let ipsec_policy_binding = args.ipsec_policy.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let radius_binding = args.radius.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpn_authentication_types_binding = args
            .vpn_authentication_types
            .get_output(context);
        let vpn_protocols_binding = args.vpn_protocols.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/vpnServerConfiguration:VpnServerConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureActiveDirectoryAuthentications".into(),
                    value: azure_active_directory_authentications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientRevokedCertificates".into(),
                    value: client_revoked_certificates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientRootCertificates".into(),
                    value: client_root_certificates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipsecPolicy".into(),
                    value: ipsec_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "radius".into(),
                    value: radius_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnAuthenticationTypes".into(),
                    value: vpn_authentication_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnProtocols".into(),
                    value: vpn_protocols_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnServerConfigurationResult {
            azure_active_directory_authentications: o
                .get_field("azureActiveDirectoryAuthentications"),
            client_revoked_certificates: o.get_field("clientRevokedCertificates"),
            client_root_certificates: o.get_field("clientRootCertificates"),
            ipsec_policy: o.get_field("ipsecPolicy"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            radius: o.get_field("radius"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            vpn_authentication_types: o.get_field("vpnAuthenticationTypes"),
            vpn_protocols: o.get_field("vpnProtocols"),
        }
    }
}
