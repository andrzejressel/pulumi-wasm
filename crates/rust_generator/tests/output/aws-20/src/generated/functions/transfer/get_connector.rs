#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectorArgs,
    ) -> GetConnectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:transfer/getConnector:getConnector".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectorResult {
            access_role: o.get_field("accessRole"),
            arn: o.get_field("arn"),
            as2_configs: o.get_field("as2Configs"),
            id: o.get_field("id"),
            logging_role: o.get_field("loggingRole"),
            security_policy_name: o.get_field("securityPolicyName"),
            service_managed_egress_ip_addresses: o
                .get_field("serviceManagedEgressIpAddresses"),
            sftp_configs: o.get_field("sftpConfigs"),
            tags: o.get_field("tags"),
            url: o.get_field("url"),
        }
    }
}
