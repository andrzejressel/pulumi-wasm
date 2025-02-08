#[allow(clippy::doc_lazy_continuation)]
pub mod get_connector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectorArgs {
        /// Unique identifier for connector
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConnectorResult {
        /// ARN of the AWS Identity and Access Management role.
        pub access_role: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Connector.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Structure containing the parameters for an AS2 connector object. Contains the following attributes:
        pub as2_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::transfer::GetConnectorAs2Config>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that allows a connector to turn on CLoudwatch logging for Amazon S3 events.
        pub logging_role: pulumi_gestalt_rust::Output<String>,
        /// Name of security policy.
        pub security_policy_name: pulumi_gestalt_rust::Output<String>,
        /// List of egress Ip addresses.
        pub service_managed_egress_ip_addresses: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Object containing the following attributes:
        pub sftp_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::transfer::GetConnectorSftpConfig>,
        >,
        /// Object containing the following attributes:
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// URL of the partner's AS2 or SFTP endpoint.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetConnectorArgs,
    ) -> GetConnectorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:transfer/getConnector:getConnector".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConnectorResult {
            access_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessRole"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            as2_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("as2Configs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            logging_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingRole"),
            ),
            security_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicyName"),
            ),
            service_managed_egress_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceManagedEgressIpAddresses"),
            ),
            sftp_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sftpConfigs"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
