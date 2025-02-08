#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_object_signed_url {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetObjectSignedUrlArgs {
        /// The name of the bucket to read the object from
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The [MD5 digest](https://cloud.google.com/storage/docs/hashes-etags#_MD5) value in Base64.
        /// Typically retrieved from `google_storage_bucket_object.object.md5hash` attribute.
        /// If you provide this in the datasource, the client (e.g. browser, curl) must provide the `Content-MD5` HTTP header with this same value in its request.
        #[builder(into, default)]
        pub content_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If you specify this in the datasource, the client must provide the `Content-Type` HTTP header with the same value in its request.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// What Google service account credentials json should be used to sign the URL.
        /// This data source checks the following locations for credentials, in order of preference: data source `credentials` attribute, provider `credentials` attribute and finally the GOOGLE_APPLICATION_CREDENTIALS environment variable.
        ///
        /// > **NOTE** the default google credentials configured by `gcloud` sdk or the service account associated with a compute instance cannot be used, because these do not include the private key required to sign the URL. A valid `json` service account credentials key file must be used, as generated via Google cloud console.
        #[builder(into, default)]
        pub credentials: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// For how long shall the signed URL be valid (defaults to 1 hour - i.e. `1h`).
        /// See [here](https://golang.org/pkg/time/#ParseDuration) for info on valid duration formats.
        #[builder(into, default)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// As needed. The server checks to make sure that the client provides matching values in requests using the signed URL.
        /// Any header starting with `x-goog-` is accepted but see the [Google Docs](https://cloud.google.com/storage/docs/xml-api/reference-headers) for list of headers that are supported by Google.
        #[builder(into, default)]
        pub extension_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// What HTTP Method will the signed URL allow (defaults to `GET`)
        #[builder(into, default)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full path to the object inside the bucket
        #[builder(into)]
        pub path: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetObjectSignedUrlResult {
        pub bucket: pulumi_gestalt_rust::Output<String>,
        pub content_md5: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub credentials: pulumi_gestalt_rust::Output<Option<String>>,
        pub duration: pulumi_gestalt_rust::Output<Option<String>>,
        pub extension_headers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub http_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub path: pulumi_gestalt_rust::Output<String>,
        /// The signed URL that can be used to access the storage object without authentication.
        pub signed_url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetObjectSignedUrlArgs,
    ) -> GetObjectSignedUrlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let content_md5_binding = args.content_md5.get_output(context).get_inner();
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let credentials_binding = args.credentials.get_output(context).get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let extension_headers_binding = args
            .extension_headers
            .get_output(context)
            .get_inner();
        let http_method_binding = args.http_method.get_output(context).get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getObjectSignedUrl:getObjectSignedUrl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "contentMd5".into(),
                    value: &content_md5_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "extensionHeaders".into(),
                    value: &extension_headers_binding,
                },
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetObjectSignedUrlResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            content_md5: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentMd5"),
            ),
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            credentials: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("credentials"),
            ),
            duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            extension_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extensionHeaders"),
            ),
            http_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpMethod"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            signed_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signedUrl"),
            ),
        }
    }
}
