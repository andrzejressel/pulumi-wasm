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
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBrokerResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            authentication_strategy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationStrategy"),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            broker_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("brokerId"),
            ),
            broker_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("brokerName"),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            deployment_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentMode"),
            ),
            encryption_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionOptions"),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineType"),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            host_instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostInstanceType"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            ldap_server_metadatas: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ldapServerMetadatas"),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(o.extract_field("logs")),
            maintenance_window_start_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceWindowStartTime"),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publiclyAccessible"),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            users: pulumi_wasm_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
