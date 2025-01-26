pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// The family name of the image.
        #[builder(into, default)]
        pub family: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A boolean to indicate either to take to most recent image if your filter
        /// returns more than one image.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// , `family` or `filter` - (Required) The name of a specific image or a family.
        /// Exactly one of `name`, `family` or `filter` must be specified. If `name` is specified, it will fetch
        /// the corresponding image. If `family` is specified, it will return the latest image
        /// that is part of an image family and is not deprecated. If you specify `filter`, your
        /// filter must return exactly one image unless you use `most_recent`.
        /// Filter syntax can be found [here](https://cloud.google.com/compute/docs/reference/rest/v1/images/list) in the filter section.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project in which the resource belongs. If it is not
        /// provided, the provider project is used. If you are using a
        /// [public base image][pubimg], be sure to specify the correct Image Project.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The size of the image tar.gz archive stored in Google Cloud Storage in bytes.
        pub archive_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this image.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The size of the image when restored onto a persistent disk in gigabytes.
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        /// The family name of the image.
        pub family: pulumi_wasm_rust::Output<String>,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
        /// encoded SHA-256 hash of the [customer-supplied encryption key](https://cloud.google.com/compute/docs/disks/customer-supplied-encryption)
        /// that protects this image.
        pub image_encryption_key_sha256: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the image.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// A fingerprint for the labels being applied to this image.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of applicable license URI.
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the image.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the image.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The URL of the source disk used to create this image.
        pub source_disk: pulumi_wasm_rust::Output<String>,
        /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
        /// encoded SHA-256 hash of the [customer-supplied encryption key](https://cloud.google.com/compute/docs/disks/customer-supplied-encryption)
        /// that protects this image.
        pub source_disk_encryption_key_sha256: pulumi_wasm_rust::Output<String>,
        /// The ID value of the disk used to create this image.
        pub source_disk_id: pulumi_wasm_rust::Output<String>,
        /// The ID value of the image used to create this image.
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        /// The status of the image. Possible values are **FAILED**, **PENDING**, or **READY**.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let family_binding = args.family.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "archiveSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "family".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageEncryptionKeySha256".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "licenses".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
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
                    name: "sourceDisk".into(),
                },
                register_interface::ResultField {
                    name: "sourceDiskEncryptionKeySha256".into(),
                },
                register_interface::ResultField {
                    name: "sourceDiskId".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImageResult {
            archive_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("archiveSizeBytes").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSizeGb").unwrap(),
            ),
            family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("family").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_encryption_key_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageEncryptionKeySha256").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            licenses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenses").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
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
            source_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDisk").unwrap(),
            ),
            source_disk_encryption_key_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDiskEncryptionKeySha256").unwrap(),
            ),
            source_disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDiskId").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
