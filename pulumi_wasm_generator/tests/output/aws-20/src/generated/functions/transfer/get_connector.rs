pub mod get_connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectorArgs {
        /// Unique identifier for connector
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetConnectorResult {
        /// ARN of the AWS Identity and Access Management role.
        pub access_role: pulumi_wasm_rust::Output<String>,
        /// ARN of the Connector.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Structure containing the parameters for an AS2 connector object. Contains the following attributes:
        pub as2_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::transfer::GetConnectorAs2Config>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that allows a connector to turn on CLoudwatch logging for Amazon S3 events.
        pub logging_role: pulumi_wasm_rust::Output<String>,
        /// Name of security policy.
        pub security_policy_name: pulumi_wasm_rust::Output<String>,
        /// List of egress Ip addresses.
        pub service_managed_egress_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Object containing the following attributes:
        pub sftp_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::transfer::GetConnectorSftpConfig>,
        >,
        /// Object containing the following attributes:
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// URL of the partner's AS2 or SFTP endpoint.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConnectorArgs) -> GetConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:transfer/getConnector:getConnector".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessRole".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "as2Configs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "loggingRole".into(),
                },
                register_interface::ResultField {
                    name: "securityPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "serviceManagedEgressIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "sftpConfigs".into(),
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
        GetConnectorResult {
            access_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessRole").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            as2_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("as2Configs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            logging_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingRole").unwrap(),
            ),
            security_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPolicyName").unwrap(),
            ),
            service_managed_egress_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceManagedEgressIpAddresses").unwrap(),
            ),
            sftp_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sftpConfigs").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
