#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// ID of the directory.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags assigned to the directory/connector.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDirectoryResult {
        /// Access URL for the directory/connector, such as http://alias.awsapps.com.
        pub access_url: pulumi_gestalt_rust::Output<String>,
        /// Alias for the directory/connector, such as `d-991708b282.awsapps.com`.
        pub alias: pulumi_gestalt_rust::Output<String>,
        pub connect_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryConnectSetting>,
        >,
        /// Textual description for the directory/connector.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// List of IP addresses of the DNS servers for the directory/connector.
        pub dns_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// (for `MicrosoftAD`) Microsoft AD edition (`Standard` or `Enterprise`).
        pub edition: pulumi_gestalt_rust::Output<String>,
        /// Directory/connector single-sign on status.
        pub enable_sso: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Fully qualified name for the directory/connector.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub radius_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryRadiusSetting>,
        >,
        /// ID of the security group created by the directory/connector.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// Short name of the directory/connector, such as `CORP`.
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// (for `SimpleAD` and `ADConnector`) Size of the directory/connector (`Small` or `Large`).
        pub size: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the directory/connector.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Directory type (`SimpleAD`, `ADConnector` or `MicrosoftAD`).
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub vpc_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryVpcSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDirectoryArgs,
    ) -> GetDirectoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directoryservice/getDirectory:getDirectory".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDirectoryResult {
            access_url: o.get_field("accessUrl"),
            alias: o.get_field("alias"),
            connect_settings: o.get_field("connectSettings"),
            description: o.get_field("description"),
            directory_id: o.get_field("directoryId"),
            dns_ip_addresses: o.get_field("dnsIpAddresses"),
            edition: o.get_field("edition"),
            enable_sso: o.get_field("enableSso"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            radius_settings: o.get_field("radiusSettings"),
            security_group_id: o.get_field("securityGroupId"),
            short_name: o.get_field("shortName"),
            size: o.get_field("size"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            vpc_settings: o.get_field("vpcSettings"),
        }
    }
}
