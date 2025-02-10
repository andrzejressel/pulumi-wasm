#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceArgs {
        /// The name of the instance.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceResult {
        pub available_maintenance_versions: pulumi_gestalt_rust::Output<Vec<String>>,
        pub clones: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceClone>,
        >,
        pub connection_name: pulumi_gestalt_rust::Output<String>,
        pub database_version: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub encryption_key_name: pulumi_gestalt_rust::Output<String>,
        pub first_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        pub ip_addresses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceIpAddress>,
        >,
        pub maintenance_version: pulumi_gestalt_rust::Output<String>,
        pub master_instance_name: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub psc_service_attachment_link: pulumi_gestalt_rust::Output<String>,
        pub public_ip_address: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        pub replica_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceReplicaConfiguration>,
        >,
        pub replica_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub restore_backup_contexts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceRestoreBackupContext>,
        >,
        pub root_password: pulumi_gestalt_rust::Output<String>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub server_ca_certs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceServerCaCert>,
        >,
        pub service_account_email_address: pulumi_gestalt_rust::Output<String>,
        pub settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatabaseInstanceArgs,
    ) -> GetDatabaseInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:sql/getDatabaseInstance:getDatabaseInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatabaseInstanceResult {
            available_maintenance_versions: o.get_field("availableMaintenanceVersions"),
            clones: o.get_field("clones"),
            connection_name: o.get_field("connectionName"),
            database_version: o.get_field("databaseVersion"),
            deletion_protection: o.get_field("deletionProtection"),
            dns_name: o.get_field("dnsName"),
            encryption_key_name: o.get_field("encryptionKeyName"),
            first_ip_address: o.get_field("firstIpAddress"),
            id: o.get_field("id"),
            instance_type: o.get_field("instanceType"),
            ip_addresses: o.get_field("ipAddresses"),
            maintenance_version: o.get_field("maintenanceVersion"),
            master_instance_name: o.get_field("masterInstanceName"),
            name: o.get_field("name"),
            private_ip_address: o.get_field("privateIpAddress"),
            project: o.get_field("project"),
            psc_service_attachment_link: o.get_field("pscServiceAttachmentLink"),
            public_ip_address: o.get_field("publicIpAddress"),
            region: o.get_field("region"),
            replica_configurations: o.get_field("replicaConfigurations"),
            replica_names: o.get_field("replicaNames"),
            restore_backup_contexts: o.get_field("restoreBackupContexts"),
            root_password: o.get_field("rootPassword"),
            self_link: o.get_field("selfLink"),
            server_ca_certs: o.get_field("serverCaCerts"),
            service_account_email_address: o.get_field("serviceAccountEmailAddress"),
            settings: o.get_field("settings"),
        }
    }
}
