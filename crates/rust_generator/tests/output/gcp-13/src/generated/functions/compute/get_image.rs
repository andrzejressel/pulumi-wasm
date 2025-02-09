#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// The family name of the image.
        #[builder(into, default)]
        pub family: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean to indicate either to take to most recent image if your filter
        /// returns more than one image.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// , `family` or `filter` - (Required) The name of a specific image or a family.
        /// Exactly one of `name`, `family` or `filter` must be specified. If `name` is specified, it will fetch
        /// the corresponding image. If `family` is specified, it will return the latest image
        /// that is part of an image family and is not deprecated. If you specify `filter`, your
        /// filter must return exactly one image unless you use `most_recent`.
        /// Filter syntax can be found [here](https://cloud.google.com/compute/docs/reference/rest/v1/images/list) in the filter section.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project in which the resource belongs. If it is not
        /// provided, the provider project is used. If you are using a
        /// [public base image][pubimg], be sure to specify the correct Image Project.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The size of the image tar.gz archive stored in Google Cloud Storage in bytes.
        pub archive_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this image.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The size of the image when restored onto a persistent disk in gigabytes.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The family name of the image.
        pub family: pulumi_gestalt_rust::Output<String>,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
        /// encoded SHA-256 hash of the [customer-supplied encryption key](https://cloud.google.com/compute/docs/disks/customer-supplied-encryption)
        /// that protects this image.
        pub image_encryption_key_sha256: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the image.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// A fingerprint for the labels being applied to this image.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of applicable license URI.
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the image.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the image.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The URL of the source disk used to create this image.
        pub source_disk: pulumi_gestalt_rust::Output<String>,
        /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
        /// encoded SHA-256 hash of the [customer-supplied encryption key](https://cloud.google.com/compute/docs/disks/customer-supplied-encryption)
        /// that protects this image.
        pub source_disk_encryption_key_sha256: pulumi_gestalt_rust::Output<String>,
        /// The ID value of the disk used to create this image.
        pub source_disk_id: pulumi_gestalt_rust::Output<String>,
        /// The ID value of the image used to create this image.
        pub source_image_id: pulumi_gestalt_rust::Output<String>,
        /// The status of the image. Possible values are **FAILED**, **PENDING**, or **READY**.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let family_binding_1 = args.family.get_output(context);
        let family_binding = family_binding_1.get_inner();
        let filter_binding_1 = args.filter.get_output(context);
        let filter_binding = filter_binding_1.get_inner();
        let most_recent_binding_1 = args.most_recent.get_output(context);
        let most_recent_binding = most_recent_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImageResult {
            archive_size_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("archiveSizeBytes"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskSizeGb"),
            ),
            family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("family"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_encryption_key_sha256: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageEncryptionKeySha256"),
            ),
            image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageId"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            licenses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenses"),
            ),
            most_recent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecent"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            source_disk: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDisk"),
            ),
            source_disk_encryption_key_sha256: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDiskEncryptionKeySha256"),
            ),
            source_disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDiskId"),
            ),
            source_image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceImageId"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
