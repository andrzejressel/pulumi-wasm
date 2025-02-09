/// The Consent Management API is a tool for tracking user consents and the documentation associated with the consents.
///
///
/// To get more information about ConsentStore, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets.consentStores)
/// * How-to Guides
///     * [Creating a Consent store](https://cloud.google.com/healthcare/docs/how-tos/consent)
///
/// ## Example Usage
///
/// ### Healthcare Consent Store Basic
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       location: us-central1
///       name: my-dataset
///   my-consent:
///     type: gcp:healthcare:ConsentStore
///     properties:
///       dataset: ${dataset.id}
///       name: my-consent-store
/// ```
/// ### Healthcare Consent Store Full
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       location: us-central1
///       name: my-dataset
///   my-consent:
///     type: gcp:healthcare:ConsentStore
///     properties:
///       dataset: ${dataset.id}
///       name: my-consent-store
///       enableConsentCreateOnUpdate: true
///       defaultConsentTtl: 90000s
///       labels:
///         label1: labelvalue1
/// ```
/// ### Healthcare Consent Store Iam
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       location: us-central1
///       name: my-dataset
///   my-consent:
///     type: gcp:healthcare:ConsentStore
///     properties:
///       dataset: ${dataset.id}
///       name: my-consent-store
///   test-account:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-account
///       displayName: Test Service Account
///   test-iam:
///     type: gcp:healthcare:ConsentStoreIamMember
///     properties:
///       dataset: ${dataset.id}
///       consentStoreId: ${["my-consent"].name}
///       role: roles/editor
///       member: serviceAccount:${["test-account"].email}
/// ```
///
/// ## Import
///
/// ConsentStore can be imported using any of these accepted formats:
///
/// * `{{dataset}}/consentStores/{{name}}`
///
/// When using the `pulumi import` command, ConsentStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/consentStore:ConsentStore default {{dataset}}/consentStores/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod consent_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsentStoreArgs {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Default time to live for consents in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents.
        /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        #[builder(into, default)]
        pub default_consent_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, [consents.patch] [google.cloud.healthcare.v1.consent.UpdateConsent] creates the consent if it does not already exist.
        #[builder(into, default)]
        pub enable_consent_create_on_update: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// User-supplied key-value pairs used to organize Consent stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of this ConsentStore, for example:
        /// "consent1"
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConsentStoreResult {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// Default time to live for consents in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents.
        /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        pub default_consent_ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If true, [consents.patch] [google.cloud.healthcare.v1.consent.UpdateConsent] creates the consent if it does not already exist.
        pub enable_consent_create_on_update: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User-supplied key-value pairs used to organize Consent stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of this ConsentStore, for example:
        /// "consent1"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConsentStoreArgs,
    ) -> ConsentStoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dataset_binding = args.dataset.get_output(context);
        let default_consent_ttl_binding = args.default_consent_ttl.get_output(context);
        let enable_consent_create_on_update_binding = args
            .enable_consent_create_on_update
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/consentStore:ConsentStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: dataset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultConsentTtl".into(),
                    value: default_consent_ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableConsentCreateOnUpdate".into(),
                    value: enable_consent_create_on_update_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConsentStoreResult {
            dataset: o.get_field("dataset"),
            default_consent_ttl: o.get_field("defaultConsentTtl"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_consent_create_on_update: o.get_field("enableConsentCreateOnUpdate"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
