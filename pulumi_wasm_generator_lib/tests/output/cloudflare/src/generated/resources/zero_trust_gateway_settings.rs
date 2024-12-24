#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    #[builder(into, default)]
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    #[builder(into, default)]
    pub antivirus: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
    >,
    /// Configuration for a custom block page.
    #[builder(into, default)]
    pub block_page: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
    >,
    /// Configuration for body scanning.
    #[builder(into, default)]
    pub body_scanning: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
    >,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    #[builder(into, default)]
    pub certificate: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsCertificate>,
    >,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    #[builder(into, default)]
    pub custom_certificate: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
    >,
    /// Configuration for extended e-mail matching.
    #[builder(into, default)]
    pub extended_email_matching: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsExtendedEmailMatching>,
    >,
    /// Configure compliance with Federal Information Processing Standards.
    #[builder(into, default)]
    pub fips: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsFips>,
    >,
    #[builder(into, default)]
    pub logging: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsLogging>,
    >,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    #[builder(into, default)]
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    #[builder(into, default)]
    pub payload_log: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
    >,
    /// Indicator that protocol detection is enabled.
    #[builder(into, default)]
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    #[builder(into, default)]
    pub proxy: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsProxy>,
    >,
    /// Configuration for SSH Session Logging.
    #[builder(into, default)]
    pub ssh_session_log: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
    >,
    /// Indicator that decryption of TLS traffic is enabled.
    #[builder(into, default)]
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    #[builder(into, default)]
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}
pub struct ZeroTrustGatewaySettingsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    pub antivirus: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
    >,
    /// Configuration for a custom block page.
    pub block_page: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
    >,
    /// Configuration for body scanning.
    pub body_scanning: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
    >,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    pub certificate: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsCertificate>,
    >,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    pub custom_certificate: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
    >,
    /// Configuration for extended e-mail matching.
    pub extended_email_matching: pulumi_wasm_rust::Output<
        super::types::ZeroTrustGatewaySettingsExtendedEmailMatching,
    >,
    /// Configure compliance with Federal Information Processing Standards.
    pub fips: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsFips>,
    >,
    pub logging: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsLogging>,
    >,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    pub payload_log: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
    >,
    /// Indicator that protocol detection is enabled.
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    pub proxy: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsProxy>,
    >,
    /// Configuration for SSH Session Logging.
    pub ssh_session_log: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
    >,
    /// Indicator that decryption of TLS traffic is enabled.
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustGatewaySettingsArgs,
) -> ZeroTrustGatewaySettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let activity_log_enabled_binding = args.activity_log_enabled.get_inner();
    let antivirus_binding = args.antivirus.get_inner();
    let block_page_binding = args.block_page.get_inner();
    let body_scanning_binding = args.body_scanning.get_inner();
    let certificate_binding = args.certificate.get_inner();
    let custom_certificate_binding = args.custom_certificate.get_inner();
    let extended_email_matching_binding = args.extended_email_matching.get_inner();
    let fips_binding = args.fips.get_inner();
    let logging_binding = args.logging.get_inner();
    let non_identity_browser_isolation_enabled_binding = args
        .non_identity_browser_isolation_enabled
        .get_inner();
    let payload_log_binding = args.payload_log.get_inner();
    let protocol_detection_enabled_binding = args.protocol_detection_enabled.get_inner();
    let proxy_binding = args.proxy.get_inner();
    let ssh_session_log_binding = args.ssh_session_log.get_inner();
    let tls_decrypt_enabled_binding = args.tls_decrypt_enabled.get_inner();
    let url_browser_isolation_enabled_binding = args
        .url_browser_isolation_enabled
        .get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "activityLogEnabled".into(),
                value: &activity_log_enabled_binding,
            },
            register_interface::ObjectField {
                name: "antivirus".into(),
                value: &antivirus_binding,
            },
            register_interface::ObjectField {
                name: "blockPage".into(),
                value: &block_page_binding,
            },
            register_interface::ObjectField {
                name: "bodyScanning".into(),
                value: &body_scanning_binding,
            },
            register_interface::ObjectField {
                name: "certificate".into(),
                value: &certificate_binding,
            },
            register_interface::ObjectField {
                name: "customCertificate".into(),
                value: &custom_certificate_binding,
            },
            register_interface::ObjectField {
                name: "extendedEmailMatching".into(),
                value: &extended_email_matching_binding,
            },
            register_interface::ObjectField {
                name: "fips".into(),
                value: &fips_binding,
            },
            register_interface::ObjectField {
                name: "logging".into(),
                value: &logging_binding,
            },
            register_interface::ObjectField {
                name: "nonIdentityBrowserIsolationEnabled".into(),
                value: &non_identity_browser_isolation_enabled_binding,
            },
            register_interface::ObjectField {
                name: "payloadLog".into(),
                value: &payload_log_binding,
            },
            register_interface::ObjectField {
                name: "protocolDetectionEnabled".into(),
                value: &protocol_detection_enabled_binding,
            },
            register_interface::ObjectField {
                name: "proxy".into(),
                value: &proxy_binding,
            },
            register_interface::ObjectField {
                name: "sshSessionLog".into(),
                value: &ssh_session_log_binding,
            },
            register_interface::ObjectField {
                name: "tlsDecryptEnabled".into(),
                value: &tls_decrypt_enabled_binding,
            },
            register_interface::ObjectField {
                name: "urlBrowserIsolationEnabled".into(),
                value: &url_browser_isolation_enabled_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "activityLogEnabled".into() },
            register_interface::ResultField { name : "antivirus".into() },
            register_interface::ResultField { name : "blockPage".into() },
            register_interface::ResultField { name : "bodyScanning".into() },
            register_interface::ResultField { name : "certificate".into() },
            register_interface::ResultField { name : "customCertificate".into() },
            register_interface::ResultField { name : "extendedEmailMatching".into() },
            register_interface::ResultField { name : "fips".into() },
            register_interface::ResultField { name : "logging".into() },
            register_interface::ResultField { name : "nonIdentityBrowserIsolationEnabled"
            .into() }, register_interface::ResultField { name : "payloadLog".into() },
            register_interface::ResultField { name : "protocolDetectionEnabled".into() },
            register_interface::ResultField { name : "proxy".into() },
            register_interface::ResultField { name : "sshSessionLog".into() },
            register_interface::ResultField { name : "tlsDecryptEnabled".into() },
            register_interface::ResultField { name : "urlBrowserIsolationEnabled".into()
            },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ZeroTrustGatewaySettingsResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        activity_log_enabled: into_domain(hashmap.remove("activityLogEnabled").unwrap()),
        antivirus: into_domain(hashmap.remove("antivirus").unwrap()),
        block_page: into_domain(hashmap.remove("blockPage").unwrap()),
        body_scanning: into_domain(hashmap.remove("bodyScanning").unwrap()),
        certificate: into_domain(hashmap.remove("certificate").unwrap()),
        custom_certificate: into_domain(hashmap.remove("customCertificate").unwrap()),
        extended_email_matching: into_domain(
            hashmap.remove("extendedEmailMatching").unwrap(),
        ),
        fips: into_domain(hashmap.remove("fips").unwrap()),
        logging: into_domain(hashmap.remove("logging").unwrap()),
        non_identity_browser_isolation_enabled: into_domain(
            hashmap.remove("nonIdentityBrowserIsolationEnabled").unwrap(),
        ),
        payload_log: into_domain(hashmap.remove("payloadLog").unwrap()),
        protocol_detection_enabled: into_domain(
            hashmap.remove("protocolDetectionEnabled").unwrap(),
        ),
        proxy: into_domain(hashmap.remove("proxy").unwrap()),
        ssh_session_log: into_domain(hashmap.remove("sshSessionLog").unwrap()),
        tls_decrypt_enabled: into_domain(hashmap.remove("tlsDecryptEnabled").unwrap()),
        url_browser_isolation_enabled: into_domain(
            hashmap.remove("urlBrowserIsolationEnabled").unwrap(),
        ),
    }
}
