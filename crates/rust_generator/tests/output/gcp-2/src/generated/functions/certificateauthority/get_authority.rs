#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_authority {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorityArgs {
        /// ID of the certificate authority.
        ///
        /// - - -
        #[builder(into, default)]
        pub certificate_authority_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location the certificate authority exists in.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the pool the certificate authority belongs to.
        #[builder(into, default)]
        pub pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorityResult {
        pub access_urls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityAccessUrl>,
        >,
        pub certificate_authority_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityConfig>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub desired_state: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub gcs_bucket: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ignore_active_certificates_on_deletion: pulumi_gestalt_rust::Output<bool>,
        pub key_specs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::certificateauthority::GetAuthorityKeySpec>,
        >,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub lifetime: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub pem_ca_certificate: pulumi_gestalt_rust::Output<String>,
        pub pem_ca_certificates: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The PEM-encoded signed certificate signing request (CSR). This is only set on subordinate certificate authorities that are awaiting user activation.
        pub pem_csr: pulumi_gestalt_rust::Output<String>,
        pub pool: pulumi_gestalt_rust::Output<Option<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub skip_grace_period: pulumi_gestalt_rust::Output<bool>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub subordinate_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::certificateauthority::GetAuthoritySubordinateConfig,
            >,
        >,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAuthorityArgs,
    ) -> GetAuthorityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_authority_id_binding = args
            .certificate_authority_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let pool_binding = args.pool.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:certificateauthority/getAuthority:getAuthority".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityId".into(),
                    value: certificate_authority_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pool".into(),
                    value: pool_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAuthorityResult {
            access_urls: o.get_field("accessUrls"),
            certificate_authority_id: o.get_field("certificateAuthorityId"),
            configs: o.get_field("configs"),
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            desired_state: o.get_field("desiredState"),
            effective_labels: o.get_field("effectiveLabels"),
            gcs_bucket: o.get_field("gcsBucket"),
            id: o.get_field("id"),
            ignore_active_certificates_on_deletion: o
                .get_field("ignoreActiveCertificatesOnDeletion"),
            key_specs: o.get_field("keySpecs"),
            labels: o.get_field("labels"),
            lifetime: o.get_field("lifetime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pem_ca_certificate: o.get_field("pemCaCertificate"),
            pem_ca_certificates: o.get_field("pemCaCertificates"),
            pem_csr: o.get_field("pemCsr"),
            pool: o.get_field("pool"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            skip_grace_period: o.get_field("skipGracePeriod"),
            state: o.get_field("state"),
            subordinate_configs: o.get_field("subordinateConfigs"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
