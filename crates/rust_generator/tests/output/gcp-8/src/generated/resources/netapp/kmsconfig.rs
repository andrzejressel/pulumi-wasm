/// NetApp Volumes always encrypts your data at rest using volume-specific keys.
///
/// A CMEK policy (customer-managed encryption key) warps such volume-specific keys in a key stored in Cloud Key Management Service (KMS).
///
///
/// To get more information about kmsconfig, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.kmsConfigs)
/// * How-to Guides
///     * [Documentation](https://cloud.google.com/netapp/volumes/docs/configure-and-use/cmek/cmek-overview)
///
/// ## Example Usage
///
/// ### Kms Config Create
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${keyring.id}")
///             .name("crypto-name")
///             .build_struct(),
///     );
///     let keyring = key_ring::create(
///         "keyring",
///         KeyRingArgs::builder().location("us-central1").name("key-ring").build_struct(),
///     );
///     let kmsConfig = kmsconfig::create(
///         "kmsConfig",
///         KmsconfigArgs::builder()
///             .crypto_key_name("${cryptoKey.id}")
///             .description("this is a test description")
///             .location("us-central1")
///             .name("kms-test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// kmsconfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, kmsconfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/kmsconfig:Kmsconfig default projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/kmsconfig:Kmsconfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/kmsconfig:Kmsconfig default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kmsconfig {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KmsconfigArgs {
        /// Resource name of the KMS key to use. Only regional keys are supported. Format: `projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{key}}`.
        #[builder(into)]
        pub crypto_key_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description for the CMEK policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the policy location. CMEK policies apply to the whole region.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the CMEK policy.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KmsconfigResult {
        /// Resource name of the KMS key to use. Only regional keys are supported. Format: `projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{key}}`.
        pub crypto_key_name: pulumi_gestalt_rust::Output<String>,
        /// Description for the CMEK policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Access to the key needs to be granted. The instructions contain gcloud commands to run to grant access.
        /// To make the policy work, a CMEK policy check is required, which verifies key access.
        pub instructions: pulumi_gestalt_rust::Output<String>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the policy location. CMEK policies apply to the whole region.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the CMEK policy.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Service account which needs to have access to the  provided KMS key.
        pub service_account: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KmsconfigArgs,
    ) -> KmsconfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_name_binding = args.crypto_key_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:netapp/kmsconfig:Kmsconfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyName".into(),
                    value: crypto_key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
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
            ],
        };
        let o = context.register_resource(request);
        KmsconfigResult {
            crypto_key_name: o.get_field("cryptoKeyName"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            instructions: o.get_field("instructions"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_account: o.get_field("serviceAccount"),
        }
    }
}
