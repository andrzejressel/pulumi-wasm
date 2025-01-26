pub mod get_broker {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerArgs {
        /// Unique id of the mq broker.
        #[builder(into, default)]
        pub broker_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Unique name of the mq broker.
        #[builder(into, default)]
        pub broker_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBrokerResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub authentication_strategy: pulumi_wasm_rust::Output<String>,
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        pub broker_id: pulumi_wasm_rust::Output<String>,
        pub broker_name: pulumi_wasm_rust::Output<String>,
        pub configuration: pulumi_wasm_rust::Output<
            super::super::super::types::mq::GetBrokerConfiguration,
        >,
        pub deployment_mode: pulumi_wasm_rust::Output<String>,
        pub encryption_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerEncryptionOption>,
        >,
        pub engine_type: pulumi_wasm_rust::Output<String>,
        pub engine_version: pulumi_wasm_rust::Output<String>,
        pub host_instance_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instances: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerInstance>,
        >,
        pub ldap_server_metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerLdapServerMetadata>,
        >,
        pub logs: pulumi_wasm_rust::Output<
            super::super::super::types::mq::GetBrokerLogs,
        >,
        pub maintenance_window_start_time: pulumi_wasm_rust::Output<
            super::super::super::types::mq::GetBrokerMaintenanceWindowStartTime,
        >,
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        pub storage_type: pulumi_wasm_rust::Output<String>,
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub users: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerUser>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBrokerArgs,
    ) -> GetBrokerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let broker_id_binding = args.broker_id.get_output(context).get_inner();
        let broker_name_binding = args.broker_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:mq/getBroker:getBroker".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "brokerId".into(),
                    value: &broker_id_binding,
                },
                register_interface::ObjectField {
                    name: "brokerName".into(),
                    value: &broker_name_binding,
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
                    name: "authenticationStrategy".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "brokerId".into(),
                },
                register_interface::ResultField {
                    name: "brokerName".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "deploymentMode".into(),
                },
                register_interface::ResultField {
                    name: "encryptionOptions".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "hostInstanceType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instances".into(),
                },
                register_interface::ResultField {
                    name: "ldapServerMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "logs".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindowStartTime".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "users".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBrokerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationStrategy").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            broker_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("brokerId").unwrap(),
            ),
            broker_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("brokerName").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            deployment_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentMode").unwrap(),
            ),
            encryption_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionOptions").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            host_instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostInstanceType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instances").unwrap(),
            ),
            ldap_server_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ldapServerMetadatas").unwrap(),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logs").unwrap(),
            ),
            maintenance_window_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindowStartTime").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("users").unwrap(),
            ),
        }
    }
}
