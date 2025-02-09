/// The Eventarc GoogleChannelConfig resource
///
/// ## Example Usage
///
/// ### Basic
/// ```yaml
/// resources:
///   key1Member:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: key1_member
///     properties:
///       cryptoKeyId: ${key1.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${testProject.number}@gcp-sa-eventarc.iam.gserviceaccount.com
///   primary:
///     type: gcp:eventarc:GoogleChannelConfig
///     properties:
///       location: us-west1
///       name: channel
///       project: ${testProject.projectId}
///       cryptoKeyName: ${key1.id}
///     options:
///       dependsOn:
///         - ${key1Member}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments:
///         projectId: my-project-name
///   testKeyRing:
///     fn::invoke:
///       function: gcp:kms:getKMSKeyRing
///       arguments:
///         name: keyring
///         location: us-west1
///   key:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKey
///       arguments:
///         name: key
///         keyRing: ${testKeyRing.id}
/// ```
///
/// ## Import
///
/// GoogleChannelConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/googleChannelConfig`
///
/// * `{{project}}/{{location}}`
///
/// * `{{location}}`
///
/// When using the `pulumi import` command, GoogleChannelConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default projects/{{project}}/locations/{{location}}/googleChannelConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default {{project}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default {{location}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod google_channel_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GoogleChannelConfigArgs {
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        #[builder(into, default)]
        pub crypto_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GoogleChannelConfigResult {
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        pub crypto_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. The last-modified time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GoogleChannelConfigArgs,
    ) -> GoogleChannelConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let crypto_key_name_binding_1 = args.crypto_key_name.get_output(context);
        let crypto_key_name_binding = crypto_key_name_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:eventarc/googleChannelConfig:GoogleChannelConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKeyName".into(),
                    value: &crypto_key_name_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GoogleChannelConfigResult {
            crypto_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cryptoKeyName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
