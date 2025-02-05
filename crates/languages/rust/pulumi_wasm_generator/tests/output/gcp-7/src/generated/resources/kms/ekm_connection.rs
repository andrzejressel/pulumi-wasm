/// `Ekm Connections` are used to control the connection settings for an `EXTERNAL_VPC` CryptoKey.
/// It is used to connect customer's external key manager to Google Cloud EKM.
///
///
/// > **Note:** Ekm Connections cannot be deleted from Google Cloud Platform.
///
///
/// To get more information about EkmConnection, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.ekmConnections)
/// * How-to Guides
///     * [Creating a Ekm Connection](https://cloud.google.com/kms/docs/create-ekm-connection)
///
/// ## Example Usage
///
/// ### Kms Ekm Connection Basic
///
///
/// ```yaml
/// resources:
///   example-ekmconnection:
///     type: gcp:kms:EkmConnection
///     properties:
///       name: ekmconnection_example
///       location: us-central1
///       keyManagementMode: MANUAL
///       serviceResolvers:
///         - serviceDirectoryService: projects/project_id/locations/us-central1/namespaces/namespace_name/services/service_name
///           hostname: example-ekm.goog
///           serverCertificates:
///             - rawDer: ==HAwIBCCAr6gAwIBAgIUWR+EV4lqiV7Ql12VY==
/// ```
///
/// ## Import
///
/// EkmConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/ekmConnections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, EkmConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default projects/{{project}}/locations/{{location}}/ekmConnections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default {{location}}/{{name}}
/// ```
///
pub mod ekm_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EkmConnectionArgs {
        /// Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if
        /// KeyManagementMode is CLOUD_KMS.
        #[builder(into, default)]
        pub crypto_space_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Etag of the currently stored EkmConnection.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default
        /// value: "MANUAL" Possible values: ["MANUAL", "CLOUD_KMS"]
        #[builder(into, default)]
        pub key_management_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name for the EkmConnection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported
        /// Structure is documented below.
        #[builder(into)]
        pub service_resolvers: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::kms::EkmConnectionServiceResolver>,
        >,
    }
    #[allow(dead_code)]
    pub struct EkmConnectionResult {
        /// Output only. The time at which the EkmConnection was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if
        /// KeyManagementMode is CLOUD_KMS.
        pub crypto_space_path: pulumi_wasm_rust::Output<String>,
        /// Optional. Etag of the currently stored EkmConnection.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default
        /// value: "MANUAL" Possible values: ["MANUAL", "CLOUD_KMS"]
        pub key_management_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for the EkmConnection.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported
        /// Structure is documented below.
        pub service_resolvers: pulumi_wasm_rust::Output<
            Vec<super::super::types::kms::EkmConnectionServiceResolver>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EkmConnectionArgs,
    ) -> EkmConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_space_path_binding = args
            .crypto_space_path
            .get_output(context)
            .get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let key_management_mode_binding = args
            .key_management_mode
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_resolvers_binding = args
            .service_resolvers
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/ekmConnection:EkmConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoSpacePath".into(),
                    value: &crypto_space_path_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "keyManagementMode".into(),
                    value: &key_management_mode_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceResolvers".into(),
                    value: &service_resolvers_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EkmConnectionResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            crypto_space_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cryptoSpacePath"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            key_management_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyManagementMode"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_resolvers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceResolvers"),
            ),
        }
    }
}
