pub struct TeamsAccountArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
    pub extended_email_matching:
        pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct TeamsAccountResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
    pub extended_email_matching:
        pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

pub fn create(name: &str, args: TeamsAccountArgs) -> TeamsAccountResult {
    let result = crate::bindings::pulumi::cloudflare::teams_account::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::teams_account::Args {
            account_id: args.account_id.get_inner(),
            activity_log_enabled: args.activity_log_enabled.get_inner(),
            antivirus: args.antivirus.get_inner(),
            block_page: args.block_page.get_inner(),
            body_scanning: args.body_scanning.get_inner(),
            extended_email_matching: args.extended_email_matching.get_inner(),
            fips: args.fips.get_inner(),
            logging: args.logging.get_inner(),
            non_identity_browser_isolation_enabled: args
                .non_identity_browser_isolation_enabled
                .get_inner(),
            payload_log: args.payload_log.get_inner(),
            protocol_detection_enabled: args.protocol_detection_enabled.get_inner(),
            proxy: args.proxy.get_inner(),
            ssh_session_log: args.ssh_session_log.get_inner(),
            tls_decrypt_enabled: args.tls_decrypt_enabled.get_inner(),
            url_browser_isolation_enabled: args.url_browser_isolation_enabled.get_inner(),
        },
    );

    TeamsAccountResult {
        account_id: crate::into_domain(result.account_id),
        activity_log_enabled: crate::into_domain(result.activity_log_enabled),
        antivirus: crate::into_domain(result.antivirus),
        block_page: crate::into_domain(result.block_page),
        body_scanning: crate::into_domain(result.body_scanning),
        extended_email_matching: crate::into_domain(result.extended_email_matching),
        fips: crate::into_domain(result.fips),
        logging: crate::into_domain(result.logging),
        non_identity_browser_isolation_enabled: crate::into_domain(
            result.non_identity_browser_isolation_enabled,
        ),
        payload_log: crate::into_domain(result.payload_log),
        protocol_detection_enabled: crate::into_domain(result.protocol_detection_enabled),
        proxy: crate::into_domain(result.proxy),
        ssh_session_log: crate::into_domain(result.ssh_session_log),
        tls_decrypt_enabled: crate::into_domain(result.tls_decrypt_enabled),
        url_browser_isolation_enabled: crate::into_domain(result.url_browser_isolation_enabled),
    }
}
