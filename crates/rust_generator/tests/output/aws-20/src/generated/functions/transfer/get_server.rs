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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServerArgs,
    ) -> GetServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let server_id_binding_1 = args.server_id.get_output(context);
        let server_id_binding = server_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:transfer/getServer:getServer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domain"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointType"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_provider_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityProviderType"),
            ),
            invocation_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invocationRole"),
            ),
            logging_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingRole"),
            ),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            security_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicyName"),
            ),
            server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
            structured_log_destinations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("structuredLogDestinations"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
