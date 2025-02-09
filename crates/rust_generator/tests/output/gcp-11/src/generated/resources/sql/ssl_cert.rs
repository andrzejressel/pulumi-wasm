/// Creates a new Google SQL SSL Cert on a Google SQL Instance. For more information, see the [official documentation](https://cloud.google.com/sql/), or the [JSON API](https://cloud.google.com/sql/docs/mysql/admin-api/v1beta4/sslCerts).
///
///
///
/// ## Example Usage
///
/// Example creating a SQL Client Certificate.
///
/// ```yaml
/// resources:
///   dbNameSuffix:
///     type: random:RandomId
///     name: db_name_suffix
///     properties:
///       byteLength: 4
///   main:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: main-instance-${dbNameSuffix.hex}
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///   clientCert:
///     type: gcp:sql:SslCert
///     name: client_cert
///     properties:
///       commonName: client-name
///       instance: ${main.name}
/// ```
///
/// ## Import
///
/// Since the contents of the certificate cannot be accessed after its creation, this resource cannot be imported.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssl_cert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SslCertArgs {
        /// The common name to be used in the certificate to identify the
        /// client. Constrained to [a-zA-Z.-_ ]+. Changing this forces a new resource to be created.
        #[builder(into)]
        pub common_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Cloud SQL instance. Changing this
        /// forces a new resource to be created.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SslCertResult {
        /// The actual certificate data for this client certificate.
        pub cert: pulumi_gestalt_rust::Output<String>,
        /// The serial number extracted from the certificate data.
        pub cert_serial_number: pulumi_gestalt_rust::Output<String>,
        /// The common name to be used in the certificate to identify the
        /// client. Constrained to [a-zA-Z.-_ ]+. Changing this forces a new resource to be created.
        pub common_name: pulumi_gestalt_rust::Output<String>,
        /// The time when the certificate was created in RFC 3339 format,
        /// for example 2012-11-15T16:19:00.094Z.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time when the certificate expires in RFC 3339 format,
        /// for example 2012-11-15T16:19:00.094Z.
        pub expiration_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cloud SQL instance. Changing this
        /// forces a new resource to be created.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The private key associated with the client certificate.
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The CA cert of the server this client cert was generated from.
        pub server_ca_cert: pulumi_gestalt_rust::Output<String>,
        /// The SHA1 Fingerprint of the certificate.
        pub sha1_fingerprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SslCertArgs,
    ) -> SslCertResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let common_name_binding = args.common_name.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:sql/sslCert:SslCert".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonName".into(),
                    value: common_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SslCertResult {
            cert: o.get_field("cert"),
            cert_serial_number: o.get_field("certSerialNumber"),
            common_name: o.get_field("commonName"),
            create_time: o.get_field("createTime"),
            expiration_time: o.get_field("expirationTime"),
            instance: o.get_field("instance"),
            private_key: o.get_field("privateKey"),
            project: o.get_field("project"),
            server_ca_cert: o.get_field("serverCaCert"),
            sha1_fingerprint: o.get_field("sha1Fingerprint"),
        }
    }
}
