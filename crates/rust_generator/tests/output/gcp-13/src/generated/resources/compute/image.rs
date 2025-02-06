/// Represents an Image resource.
///
/// Google Compute Engine uses operating system images to create the root
/// persistent disks for your instances. You specify an image when you create
/// an instance. Images contain a boot loader, an operating system, and a
/// root file system. Linux operating system images are also capable of
/// running containers on Compute Engine.
///
/// Images can be either public or custom.
///
/// Public images are provided and maintained by Google, open-source
/// communities, and third-party vendors. By default, all projects have
/// access to these images and can use them to create instances.  Custom
/// images are available only to your project. You can create a custom image
/// from root persistent disks and other images. Then, use the custom image
/// to create an instance.
///
///
/// To get more information about Image, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/images)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/images)
///
/// ## Example Usage
///
/// ### Image Basic
///
///
/// ```yaml
/// resources:
///   persistent:
///     type: gcp:compute:Disk
///     properties:
///       name: example-disk
///       image: ${debian.selfLink}
///       size: 10
///       type: pd-ssd
///       zone: us-central1-a
///   example:
///     type: gcp:compute:Image
///     properties:
///       name: example-image
///       sourceDisk: ${persistent.id}
/// variables:
///   debian:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-12
///         project: debian-cloud
/// ```
/// ### Image Guest Os
///
///
/// ```yaml
/// resources:
///   persistent:
///     type: gcp:compute:Disk
///     properties:
///       name: example-disk
///       image: ${debian.selfLink}
///       size: 10
///       type: pd-ssd
///       zone: us-central1-a
///   example:
///     type: gcp:compute:Image
///     properties:
///       name: example-image
///       sourceDisk: ${persistent.id}
///       guestOsFeatures:
///         - type: UEFI_COMPATIBLE
///         - type: VIRTIO_SCSI_MULTIQUEUE
///         - type: GVNIC
///         - type: SEV_CAPABLE
///         - type: SEV_LIVE_MIGRATABLE_V2
/// variables:
///   debian:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-12
///         project: debian-cloud
/// ```
/// ### Image Basic Storage Location
///
///
/// ```yaml
/// resources:
///   persistent:
///     type: gcp:compute:Disk
///     properties:
///       name: example-disk
///       image: ${debian.selfLink}
///       size: 10
///       type: pd-ssd
///       zone: us-central1-a
///   example:
///     type: gcp:compute:Image
///     properties:
///       name: example-sl-image
///       sourceDisk: ${persistent.id}
///       storageLocations:
///         - us-central1
/// variables:
///   debian:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-12
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// Image can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/images/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Image can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/image:Image default projects/{{project}}/global/images/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/image:Image default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/image:Image default {{name}}
/// ```
///
pub mod image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Size of the image when restored onto a persistent disk (in GB).
        #[builder(into, default)]
        pub disk_size_gb: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The name of the image family to which this image belongs. You can
        /// create disks by specifying an image family instead of a specific
        /// image name. The image family always returns its latest image that is
        /// not deprecated. The name of the image family must comply with
        /// RFC1035.
        #[builder(into, default)]
        pub family: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable images.
        /// Structure is documented below.
        #[builder(into, default)]
        pub guest_os_features: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ImageGuestOsFeature>>,
        >,
        /// Encrypts the image using a customer-supplied encryption key.
        /// After you encrypt an image with a customer-supplied key, you must
        /// provide the same key if you use the image later (e.g. to create a
        /// disk from the image)
        /// Structure is documented below.
        #[builder(into, default)]
        pub image_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::ImageImageEncryptionKey>,
        >,
        /// Labels to apply to this Image.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any applicable license URI.
        #[builder(into, default)]
        pub licenses: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The parameters of the raw disk image.
        /// Structure is documented below.
        #[builder(into, default)]
        pub raw_disk: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::ImageRawDisk>,
        >,
        /// The source disk to create this image based on.
        /// You must provide either this property or the
        /// rawDisk.source property but not both to create an image.
        #[builder(into, default)]
        pub source_disk: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URL of the source image used to create this image. In order to create an image, you must provide the full or partial
        /// URL of one of the following:
        /// * The selfLink URL
        /// * This property
        /// * The rawDisk.source URL
        /// * The sourceDisk URL
        #[builder(into, default)]
        pub source_image: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URL of the source snapshot used to create this image.
        /// In order to create an image, you must provide the full or partial URL of one of the following:
        /// * The selfLink URL
        /// * This property
        /// * The sourceImage URL
        /// * The rawDisk.source URL
        /// * The sourceDisk URL
        #[builder(into, default)]
        pub source_snapshot: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Cloud Storage bucket storage location of the image
        /// (regional or multi-regional).
        /// Reference link: https://cloud.google.com/compute/docs/reference/rest/v1/images
        #[builder(into, default)]
        pub storage_locations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// Size of the image tar.gz archive stored in Google Cloud Storage (in
        /// bytes).
        pub archive_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Size of the image when restored onto a persistent disk (in GB).
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the image family to which this image belongs. You can
        /// create disks by specifying an image family instead of a specific
        /// image name. The image family always returns its latest image that is
        /// not deprecated. The name of the image family must comply with
        /// RFC1035.
        pub family: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable images.
        /// Structure is documented below.
        pub guest_os_features: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::ImageGuestOsFeature>,
        >,
        /// Encrypts the image using a customer-supplied encryption key.
        /// After you encrypt an image with a customer-supplied key, you must
        /// provide the same key if you use the image later (e.g. to create a
        /// disk from the image)
        /// Structure is documented below.
        pub image_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ImageImageEncryptionKey>,
        >,
        /// The fingerprint used for optimistic locking of this resource. Used
        /// internally during updates.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Labels to apply to this Image.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any applicable license URI.
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The parameters of the raw disk image.
        /// Structure is documented below.
        pub raw_disk: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ImageRawDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The source disk to create this image based on.
        /// You must provide either this property or the
        /// rawDisk.source property but not both to create an image.
        pub source_disk: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the source image used to create this image. In order to create an image, you must provide the full or partial
        /// URL of one of the following:
        /// * The selfLink URL
        /// * This property
        /// * The rawDisk.source URL
        /// * The sourceDisk URL
        pub source_image: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the source snapshot used to create this image.
        /// In order to create an image, you must provide the full or partial URL of one of the following:
        /// * The selfLink URL
        /// * This property
        /// * The sourceImage URL
        /// * The rawDisk.source URL
        /// * The sourceDisk URL
        pub source_snapshot: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud Storage bucket storage location of the image
        /// (regional or multi-regional).
        /// Reference link: https://cloud.google.com/compute/docs/reference/rest/v1/images
        pub storage_locations: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ImageArgs,
    ) -> ImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let disk_size_gb_binding = args.disk_size_gb.get_output(context).get_inner();
        let family_binding = args.family.get_output(context).get_inner();
        let guest_os_features_binding = args
            .guest_os_features
            .get_output(context)
            .get_inner();
        let image_encryption_key_binding = args
            .image_encryption_key
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let licenses_binding = args.licenses.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let raw_disk_binding = args.raw_disk.get_output(context).get_inner();
        let source_disk_binding = args.source_disk.get_output(context).get_inner();
        let source_image_binding = args.source_image.get_output(context).get_inner();
        let source_snapshot_binding = args
            .source_snapshot
            .get_output(context)
            .get_inner();
        let storage_locations_binding = args
            .storage_locations
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/image:Image".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "diskSizeGb".into(),
                    value: &disk_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "guestOsFeatures".into(),
                    value: &guest_os_features_binding,
                },
                register_interface::ObjectField {
                    name: "imageEncryptionKey".into(),
                    value: &image_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "licenses".into(),
                    value: &licenses_binding,
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
                    name: "rawDisk".into(),
                    value: &raw_disk_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDisk".into(),
                    value: &source_disk_binding,
                },
                register_interface::ObjectField {
                    name: "sourceImage".into(),
                    value: &source_image_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSnapshot".into(),
                    value: &source_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "storageLocations".into(),
                    value: &storage_locations_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ImageResult {
            archive_size_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("archiveSizeBytes"),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("diskSizeGb"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            family: pulumi_wasm_rust::__private::into_domain(o.extract_field("family")),
            guest_os_features: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guestOsFeatures"),
            ),
            image_encryption_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageEncryptionKey"),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            licenses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenses"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            raw_disk: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rawDisk"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            source_disk: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceDisk"),
            ),
            source_image: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceImage"),
            ),
            source_snapshot: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceSnapshot"),
            ),
            storage_locations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageLocations"),
            ),
        }
    }
}
