pub mod get_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerArgs {
        /// ID for an SFTP server.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServerResult {
        /// ARN of Transfer Server.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of any certificate.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The domain of the storage system that is used for file transfers.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Endpoint of the Transfer Server (e.g., `s-12345678.server.transfer.REGION.amazonaws.com`).
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Type of endpoint that the server is connected to.
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice.
        pub identity_provider_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
        pub invocation_role: pulumi_wasm_rust::Output<String>,
        /// ARN of an IAM role that allows the service to write your SFTP users’ activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
        pub logging_role: pulumi_wasm_rust::Output<String>,
        /// File transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint.
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the security policy that is attached to the server.
        pub security_policy_name: pulumi_wasm_rust::Output<String>,
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs.
        pub structured_log_destinations: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// URL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServerArgs) -> GetServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let server_id_binding = args.server_id.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "endpointType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityProviderType".into(),
                },
                register_interface::ResultField {
                    name: "invocationRole".into(),
                },
                register_interface::ResultField {
                    name: "loggingRole".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "securityPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "structuredLogDestinations".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityProviderType").unwrap(),
            ),
            invocation_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invocationRole").unwrap(),
            ),
            logging_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingRole").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            security_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPolicyName").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            structured_log_destinations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("structuredLogDestinations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
