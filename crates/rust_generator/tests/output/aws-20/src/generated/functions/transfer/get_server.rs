#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerArgs {
        /// ID for an SFTP server.
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServerResult {
        /// ARN of Transfer Server.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of any certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The domain of the storage system that is used for file transfers.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Endpoint of the Transfer Server (e.g., `s-12345678.server.transfer.REGION.amazonaws.com`).
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Type of endpoint that the server is connected to.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice.
        pub identity_provider_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
        pub invocation_role: pulumi_gestalt_rust::Output<String>,
        /// ARN of an IAM role that allows the service to write your SFTP users’ activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
        pub logging_role: pulumi_gestalt_rust::Output<String>,
        /// File transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the security policy that is attached to the server.
        pub security_policy_name: pulumi_gestalt_rust::Output<String>,
        pub server_id: pulumi_gestalt_rust::Output<String>,
        /// A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs.
        pub structured_log_destinations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// URL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServerArgs,
    ) -> GetServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let server_id_binding = args.server_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:transfer/getServer:getServer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServerResult {
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            domain: o.get_field("domain"),
            endpoint: o.get_field("endpoint"),
            endpoint_type: o.get_field("endpointType"),
            id: o.get_field("id"),
            identity_provider_type: o.get_field("identityProviderType"),
            invocation_role: o.get_field("invocationRole"),
            logging_role: o.get_field("loggingRole"),
            protocols: o.get_field("protocols"),
            security_policy_name: o.get_field("securityPolicyName"),
            server_id: o.get_field("serverId"),
            structured_log_destinations: o.get_field("structuredLogDestinations"),
            tags: o.get_field("tags"),
            url: o.get_field("url"),
        }
    }
}
