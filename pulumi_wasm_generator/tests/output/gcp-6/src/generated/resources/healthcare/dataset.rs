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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod dataset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetArgs {
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::DatasetEncryptionSpec>,
        >,
        /// The location for the Dataset.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for the Dataset.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The default timezone used by this dataset. Must be a either a valid IANA time zone name such as
        /// "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources
        /// (e.g., HL7 messages) where no explicit timezone is specified.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatasetResult {
        /// A nested object resource.
        /// Structure is documented below.
        pub encryption_spec: pulumi_wasm_rust::Output<
            super::super::types::healthcare::DatasetEncryptionSpec,
        >,
        /// The location for the Dataset.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for the Dataset.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The default timezone used by this dataset. Must be a either a valid IANA time zone name such as
        /// "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources
        /// (e.g., HL7 messages) where no explicit timezone is specified.
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatasetArgs) -> DatasetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_spec_binding = args.encryption_spec.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/dataset:Dataset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionSpec".into(),
                    value: &encryption_spec_binding,
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
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionSpec".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatasetResult {
            encryption_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionSpec").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}
