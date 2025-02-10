#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_device_posture_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDevicePostureRulesArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Device Posture Rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDevicePostureRulesResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Device Posture Rule.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of matching Device Posture Rules.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetDevicePostureRulesRule>,
        >,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDevicePostureRulesArgs,
    ) -> GetDevicePostureRulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getDevicePostureRules:getDevicePostureRules".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDevicePostureRulesResult {
            account_id: o.get_field("accountId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            rules: o.get_field("rules"),
            type_: o.get_field("type"),
        }
    }
}
