/// A Healthcare `Dataset` is a toplevel logical grouping of `dicomStores`, `fhirStores` and `hl7V2Stores`.
///
///
/// To get more information about Dataset, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets)
/// * How-to Guides
///     * [Creating a dataset](https://cloud.google.com/healthcare/docs/how-tos/datasets)
///
/// ## Example Usage
///
/// ### Healthcare Dataset Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = dataset::create(
///         "default",
///         DatasetArgs::builder()
///             .location("us-central1")
///             .name("example-dataset")
///             .time_zone("UTC")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Healthcare Dataset Cmek
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
///       timeZone: UTC
///       encryptionSpec:
///         kmsKeyName: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${healthcareCmekKeyuser}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: example-key
///       keyRing: ${keyRing.id}
///       purpose: ENCRYPT_DECRYPT
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: example-keyring
///       location: us-central1
///   healthcareCmekKeyuser:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: healthcare_cmek_keyuser
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-healthcare.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Dataset can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/datasets/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Dataset can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/dataset:Dataset default projects/{{project}}/locations/{{location}}/datasets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/dataset:Dataset default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/dataset:Dataset default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetArgs {
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::DatasetEncryptionSpec>,
        >,
        /// The location for the Dataset.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name for the Dataset.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default timezone used by this dataset. Must be a either a valid IANA time zone name such as
        /// "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources
        /// (e.g., HL7 messages) where no explicit timezone is specified.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatasetResult {
        /// A nested object resource.
        /// Structure is documented below.
        pub encryption_spec: pulumi_gestalt_rust::Output<
            super::super::types::healthcare::DatasetEncryptionSpec,
        >,
        /// The location for the Dataset.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the Dataset.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The default timezone used by this dataset. Must be a either a valid IANA time zone name such as
        /// "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources
        /// (e.g., HL7 messages) where no explicit timezone is specified.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetArgs,
    ) -> DatasetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encryption_spec_binding = args.encryption_spec.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/dataset:Dataset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSpec".into(),
                    value: &encryption_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetResult {
            encryption_spec: o.get_field("encryptionSpec"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
