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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatabaseInstanceArgs,
    ) -> GetDatabaseInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getDatabaseInstance:getDatabaseInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatabaseInstanceResult {
            available_maintenance_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availableMaintenanceVersions"),
            ),
            clones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clones"),
            ),
            connection_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionName"),
            ),
            database_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseVersion"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            encryption_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionKeyName"),
            ),
            first_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firstIpAddress"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            maintenance_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceVersion"),
            ),
            master_instance_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterInstanceName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_service_attachment_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscServiceAttachmentLink"),
            ),
            public_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddress"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            replica_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaConfigurations"),
            ),
            replica_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaNames"),
            ),
            restore_backup_contexts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restoreBackupContexts"),
            ),
            root_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootPassword"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            server_ca_certs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverCaCerts"),
            ),
            service_account_email_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccountEmailAddress"),
            ),
            settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
        }
    }
}
