#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_broker {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerArgs {
        /// Unique id of the mq broker.
        #[builder(into, default)]
        pub broker_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name of the mq broker.
        #[builder(into, default)]
        pub broker_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBrokerResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub authentication_strategy: pulumi_gestalt_rust::Output<String>,
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        pub broker_id: pulumi_gestalt_rust::Output<String>,
        pub broker_name: pulumi_gestalt_rust::Output<String>,
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::super::types::mq::GetBrokerConfiguration,
        >,
        pub deployment_mode: pulumi_gestalt_rust::Output<String>,
        pub encryption_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerEncryptionOption>,
        >,
        pub engine_type: pulumi_gestalt_rust::Output<String>,
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        pub host_instance_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instances: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerInstance>,
        >,
        pub ldap_server_metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerLdapServerMetadata>,
        >,
        pub logs: pulumi_gestalt_rust::Output<
            super::super::super::types::mq::GetBrokerLogs,
        >,
        pub maintenance_window_start_time: pulumi_gestalt_rust::Output<
            super::super::super::types::mq::GetBrokerMaintenanceWindowStartTime,
        >,
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub users: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mq::GetBrokerUser>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBrokerArgs,
    ) -> GetBrokerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let broker_id_binding = args.broker_id.get_output(context);
        let broker_name_binding = args.broker_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:mq/getBroker:getBroker".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "brokerId".into(),
                    value: &broker_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "brokerName".into(),
                    value: &broker_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBrokerResult {
            arn: o.get_field("arn"),
            authentication_strategy: o.get_field("authenticationStrategy"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            broker_id: o.get_field("brokerId"),
            broker_name: o.get_field("brokerName"),
            configuration: o.get_field("configuration"),
            deployment_mode: o.get_field("deploymentMode"),
            encryption_options: o.get_field("encryptionOptions"),
            engine_type: o.get_field("engineType"),
            engine_version: o.get_field("engineVersion"),
            host_instance_type: o.get_field("hostInstanceType"),
            id: o.get_field("id"),
            instances: o.get_field("instances"),
            ldap_server_metadatas: o.get_field("ldapServerMetadatas"),
            logs: o.get_field("logs"),
            maintenance_window_start_time: o.get_field("maintenanceWindowStartTime"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            security_groups: o.get_field("securityGroups"),
            storage_type: o.get_field("storageType"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            users: o.get_field("users"),
        }
    }
}
