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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDirectoryArgs,
    ) -> GetDirectoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:directoryservice/getDirectory:getDirectory".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectoryResult {
            access_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessUrl"),
            ),
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            connect_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectSettings"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            directory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            dns_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsIpAddresses"),
            ),
            edition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edition"),
            ),
            enable_sso: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableSso"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            radius_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("radiusSettings"),
            ),
            security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupId"),
            ),
            short_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vpc_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSettings"),
            ),
        }
    }
}
