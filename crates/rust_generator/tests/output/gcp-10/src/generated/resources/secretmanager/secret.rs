/// A Secret is a logical secret whose value and versions can be accessed.
///
///
/// To get more information about Secret, see:
///
/// * [API documentation](https://cloud.google.com/secret-manager/docs/reference/rest/v1/projects.secrets)
///
/// ## Example Usage
///
/// ### Secret Config Basic
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       labels:
///         label: my-label
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///             - location: us-east1
/// ```
/// ### Secret With Annotations
///
///
/// ```yaml
/// resources:
///   secret-with-annotations:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       labels:
///         label: my-label
///       annotations:
///         key1: someval
///         key2: someval2
///         key3: someval3
///         key4: someval4
///         key5: someval5
///       replication:
///         auto: {}
/// ```
/// ### Secret With Version Destroy Ttl
///
///
/// ```yaml
/// resources:
///   secret-with-version-destroy-ttl:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       versionDestroyTtl: 2592000s
///       replication:
///         auto: {}
/// ```
/// ### Secret With Automatic Cmek
///
///
/// ```yaml
/// resources:
///   kms-secret-binding:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-secretmanager.iam.gserviceaccount.com
///   secret-with-automatic-cmek:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       replication:
///         auto:
///           customerManagedEncryption:
///             kmsKeyName: kms-key
///     options:
///       dependsOn:
///         - ${["kms-secret-binding"]}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Secret can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/secrets/{{secret_id}}`
///
/// * `{{project}}/{{secret_id}}`
///
/// * `{{secret_id}}`
///
/// When using the `pulumi import` command, Secret can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secret:Secret default projects/{{project}}/secrets/{{secret_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secret:Secret default {{project}}/{{secret_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secret:Secret default {{secret_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretArgs {
        /// Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow
        /// client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63
        /// characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]),
        /// and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of
        /// annotation keys and values must be less than 16KiB. An object containing a list of "key": value pairs. Example: {
        /// "name": "wrench", "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent
        /// on input. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". Only one of 'expire_time' or 'ttl' can be
        /// provided.
        #[builder(into, default)]
        pub expire_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to
        /// the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be assigned to a given
        /// resource. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3"
        /// }. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The replication policy of the secret data attached to the Secret. It cannot be changed
        /// after the Secret has been created.
        /// Structure is documented below.
        #[builder(into)]
        pub replication: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::secretmanager::SecretReplication,
        >,
        /// The rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to
        /// the topics configured on the Secret. 'topics' must be set to configure rotation.
        #[builder(into, default)]
        pub rotation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::secretmanager::SecretRotation>,
        >,
        /// This must be unique within the project.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret
        /// or its versions.
        #[builder(into, default)]
        pub topics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::secretmanager::SecretTopic>>,
        >,
        /// The TTL for the Secret. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        /// Only one of 'ttl' or 'expire_time' can be provided.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can
        /// contain uppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_') characters. An alias string
        /// must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given
        /// secret. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        #[builder(into, default)]
        pub version_aliases: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Secret Version TTL after destruction request. This is a part of the delayed delete feature on Secret Version. For secret
        /// with versionDestroyTtl>0, version destruction doesn't happen immediately on calling destroy instead the version goes to
        /// a disabled state and the actual destruction happens after this TTL expires.
        #[builder(into, default)]
        pub version_destroy_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecretResult {
        /// Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow
        /// client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63
        /// characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]),
        /// and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of
        /// annotation keys and values must be less than 16KiB. An object containing a list of "key": value pairs. Example: {
        /// "name": "wrench", "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time at which the Secret was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent
        /// on input. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". Only one of 'expire_time' or 'ttl' can be
        /// provided.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to
        /// the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be assigned to a given
        /// resource. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3"
        /// }. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Secret. Format:
        /// `projects/{{project}}/secrets/{{secret_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The replication policy of the secret data attached to the Secret. It cannot be changed
        /// after the Secret has been created.
        /// Structure is documented below.
        pub replication: pulumi_gestalt_rust::Output<
            super::super::types::secretmanager::SecretReplication,
        >,
        /// The rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to
        /// the topics configured on the Secret. 'topics' must be set to configure rotation.
        pub rotation: pulumi_gestalt_rust::Output<
            Option<super::super::types::secretmanager::SecretRotation>,
        >,
        /// This must be unique within the project.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret
        /// or its versions.
        pub topics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::secretmanager::SecretTopic>>,
        >,
        /// The TTL for the Secret. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        /// Only one of 'ttl' or 'expire_time' can be provided.
        pub ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can
        /// contain uppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_') characters. An alias string
        /// must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given
        /// secret. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        pub version_aliases: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Secret Version TTL after destruction request. This is a part of the delayed delete feature on Secret Version. For secret
        /// with versionDestroyTtl>0, version destruction doesn't happen immediately on calling destroy instead the version goes to
        /// a disabled state and the actual destruction happens after this TTL expires.
        pub version_destroy_ttl: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretArgs,
    ) -> SecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let expire_time_binding = args.expire_time.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let replication_binding = args.replication.get_output(context);
        let rotation_binding = args.rotation.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let topics_binding = args.topics.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let version_aliases_binding = args.version_aliases.get_output(context);
        let version_destroy_ttl_binding = args.version_destroy_ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:secretmanager/secret:Secret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expireTime".into(),
                    value: expire_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replication".into(),
                    value: replication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotation".into(),
                    value: rotation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: secret_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topics".into(),
                    value: topics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionAliases".into(),
                    value: version_aliases_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionDestroyTtl".into(),
                    value: version_destroy_ttl_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            replication: o.get_field("replication"),
            rotation: o.get_field("rotation"),
            secret_id: o.get_field("secretId"),
            topics: o.get_field("topics"),
            ttl: o.get_field("ttl"),
            version_aliases: o.get_field("versionAliases"),
            version_destroy_ttl: o.get_field("versionDestroyTtl"),
        }
    }
}
