/// The Eventarc Channel resource
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
///     type: gcp:eventarc:Channel
///     properties:
///       location: us-west1
///       name: channel
///       project: ${testProject.projectId}
///       cryptoKeyName: ${key1.id}
///       thirdPartyProvider: projects/${testProject.projectId}/locations/us-west1/providers/datadog
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
/// Channel can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/channels/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Channel can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:eventarc/channel:Channel default projects/{{project}}/locations/{{location}}/channels/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/channel:Channel default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/channel:Channel default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelArgs {
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        #[builder(into, default)]
        pub crypto_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. The resource name of the channel. Must be unique within the location on the project.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the event provider (e.g. Eventarc SaaS partner) associated with the channel. This provider will be granted permissions to publish events to the channel. Format: `projects/{project}/locations/{location}/providers/{provider_id}`.
        #[builder(into, default)]
        pub third_party_provider: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ChannelResult {
        /// Output only. The activation token for the channel. The token must be used by the provider to register the channel for publishing.
        pub activation_token: pulumi_gestalt_rust::Output<String>,
        /// Output only. The creation time.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        pub crypto_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Required. The resource name of the channel. Must be unique within the location on the project.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{project}/topics/{topic_id}`.
        pub pubsub_topic: pulumi_gestalt_rust::Output<String>,
        /// Output only. The state of a Channel. Possible values: STATE_UNSPECIFIED, PENDING, ACTIVE, INACTIVE
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The name of the event provider (e.g. Eventarc SaaS partner) associated with the channel. This provider will be granted permissions to publish events to the channel. Format: `projects/{project}/locations/{location}/providers/{provider_id}`.
        pub third_party_provider: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The last-modified time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelArgs,
    ) -> ChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_name_binding = args.crypto_key_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let third_party_provider_binding = args.third_party_provider.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:eventarc/channel:Channel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyName".into(),
                    value: crypto_key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thirdPartyProvider".into(),
                    value: third_party_provider_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelResult {
            activation_token: o.get_field("activationToken"),
            create_time: o.get_field("createTime"),
            crypto_key_name: o.get_field("cryptoKeyName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pubsub_topic: o.get_field("pubsubTopic"),
            state: o.get_field("state"),
            third_party_provider: o.get_field("thirdPartyProvider"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
