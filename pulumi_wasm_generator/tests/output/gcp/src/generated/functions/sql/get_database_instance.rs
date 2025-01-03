pub mod get_database_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceArgs {
        /// The name of the instance.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceResult {
        pub available_maintenance_versions: pulumi_wasm_rust::Output<Vec<String>>,
        pub clones: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceClone>,
        >,
        pub connection_name: pulumi_wasm_rust::Output<String>,
        pub database_version: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub dns_name: pulumi_wasm_rust::Output<String>,
        pub encryption_key_name: pulumi_wasm_rust::Output<String>,
        pub first_ip_address: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub ip_addresses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceIpAddress>,
        >,
        pub maintenance_version: pulumi_wasm_rust::Output<String>,
        pub master_instance_name: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub psc_service_attachment_link: pulumi_wasm_rust::Output<String>,
        pub public_ip_address: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
        pub replica_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceReplicaConfiguration>,
        >,
        pub replica_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub restore_backup_contexts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceRestoreBackupContext>,
        >,
        pub root_password: pulumi_wasm_rust::Output<String>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub server_ca_certs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceServerCaCert>,
        >,
        pub service_account_email_address: pulumi_wasm_rust::Output<String>,
        pub settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstanceSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDatabaseInstanceArgs) -> GetDatabaseInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getDatabaseInstance:getDatabaseInstance".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "availableMaintenanceVersions".into(),
                },
                register_interface::ResultField {
                    name: "clones".into(),
                },
                register_interface::ResultField {
                    name: "connectionName".into(),
                },
                register_interface::ResultField {
                    name: "databaseVersion".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "encryptionKeyName".into(),
                },
                register_interface::ResultField {
                    name: "firstIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "ipAddresses".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceVersion".into(),
                },
                register_interface::ResultField {
                    name: "masterInstanceName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pscServiceAttachmentLink".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "replicaConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "replicaNames".into(),
                },
                register_interface::ResultField {
                    name: "restoreBackupContexts".into(),
                },
                register_interface::ResultField {
                    name: "rootPassword".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serverCaCerts".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountEmailAddress".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatabaseInstanceResult {
            available_maintenance_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableMaintenanceVersions").unwrap(),
            ),
            clones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clones").unwrap(),
            ),
            connection_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionName").unwrap(),
            ),
            database_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseVersion").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            encryption_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKeyName").unwrap(),
            ),
            first_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firstIpAddress").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddresses").unwrap(),
            ),
            maintenance_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceVersion").unwrap(),
            ),
            master_instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterInstanceName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddress").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            psc_service_attachment_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscServiceAttachmentLink").unwrap(),
            ),
            public_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddress").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            replica_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaConfigurations").unwrap(),
            ),
            replica_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaNames").unwrap(),
            ),
            restore_backup_contexts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreBackupContexts").unwrap(),
            ),
            root_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootPassword").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            server_ca_certs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCaCerts").unwrap(),
            ),
            service_account_email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountEmailAddress").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
        }
    }
}
