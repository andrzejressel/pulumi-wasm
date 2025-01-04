pub mod get_authority {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorityArgs {
        /// ID of the certificate authority.
        ///
        /// - - -
        #[builder(into, default)]
        pub certificate_authority_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The location the certificate authority exists in.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the pool the certificate authority belongs to.
        #[builder(into, default)]
        pub pool: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorityResult {
        pub access_urls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityAccessUrl>,
        >,
        pub certificate_authority_id: pulumi_wasm_rust::Output<Option<String>>,
        pub configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityConfig>,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub desired_state: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub gcs_bucket: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ignore_active_certificates_on_deletion: pulumi_wasm_rust::Output<bool>,
        pub key_specs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityKeySpec>,
        >,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub lifetime: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pem_ca_certificate: pulumi_wasm_rust::Output<String>,
        pub pem_ca_certificates: pulumi_wasm_rust::Output<Vec<String>>,
        /// The PEM-encoded signed certificate signing request (CSR). This is only set on subordinate certificate authorities that are awaiting user activation.
        pub pem_csr: pulumi_wasm_rust::Output<String>,
        pub pool: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub skip_grace_period: pulumi_wasm_rust::Output<bool>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub subordinate_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::certificateauthority::GetAuthoritySubordinateConfig,
            >,
        >,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAuthorityArgs) -> GetAuthorityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_authority_id_binding = args.certificate_authority_id.get_inner();
        let location_binding = args.location.get_inner();
        let pool_binding = args.pool.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:certificateauthority/getAuthority:getAuthority".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateAuthorityId".into(),
                    value: &certificate_authority_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "pool".into(),
                    value: &pool_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessUrls".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorityId".into(),
                },
                register_interface::ResultField {
                    name: "configs".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "desiredState".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "gcsBucket".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ignoreActiveCertificatesOnDeletion".into(),
                },
                register_interface::ResultField {
                    name: "keySpecs".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lifetime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pemCaCertificate".into(),
                },
                register_interface::ResultField {
                    name: "pemCaCertificates".into(),
                },
                register_interface::ResultField {
                    name: "pemCsr".into(),
                },
                register_interface::ResultField {
                    name: "pool".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "skipGracePeriod".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subordinateConfigs".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAuthorityResult {
            access_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessUrls").unwrap(),
            ),
            certificate_authority_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorityId").unwrap(),
            ),
            configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configs").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredState").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            gcs_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gcsBucket").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ignore_active_certificates_on_deletion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreActiveCertificatesOnDeletion").unwrap(),
            ),
            key_specs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySpecs").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            lifetime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifetime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pem_ca_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pemCaCertificate").unwrap(),
            ),
            pem_ca_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pemCaCertificates").unwrap(),
            ),
            pem_csr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pemCsr").unwrap(),
            ),
            pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pool").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            skip_grace_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipGracePeriod").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subordinate_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subordinateConfigs").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
