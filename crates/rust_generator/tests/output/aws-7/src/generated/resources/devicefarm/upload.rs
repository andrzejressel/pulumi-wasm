/// Provides a resource to manage AWS Device Farm Uploads.
///
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder().name("example").build_struct(),
///     );
///     let exampleUpload = upload::create(
///         "exampleUpload",
///         UploadArgs::builder()
///             .name("example")
///             .project_arn("${example.arn}")
///             .type_("APPIUM_JAVA_TESTNG_TEST_SPEC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Uploads using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/upload:Upload example arn:aws:devicefarm:us-west-2:123456789012:upload:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
pub mod upload {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UploadArgs {
        /// The upload's content type (for example, application/octet-stream).
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The upload's file name. The name should not contain any forward slashes (/). If you are uploading an iOS app, the file name must end with the .ipa extension. If you are uploading an Android app, the file name must end with the .apk extension. For all others, the file name must end with the .zip file extension.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN of the project for the upload.
        #[builder(into)]
        pub project_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The upload's upload type. See [AWS Docs](https://docs.aws.amazon.com/devicefarm/latest/APIReference/API_CreateUpload.html#API_CreateUpload_RequestSyntax) for valid list of values.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UploadResult {
        /// The Amazon Resource Name of this upload.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The upload's category.
        pub category: pulumi_wasm_rust::Output<String>,
        /// The upload's content type (for example, application/octet-stream).
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The upload's metadata. For example, for Android, this contains information that is parsed from the manifest and is displayed in the AWS Device Farm console after the associated app is uploaded.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The upload's file name. The name should not contain any forward slashes (/). If you are uploading an iOS app, the file name must end with the .ipa extension. If you are uploading an Android app, the file name must end with the .apk extension. For all others, the file name must end with the .zip file extension.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the project for the upload.
        pub project_arn: pulumi_wasm_rust::Output<String>,
        /// The upload's upload type. See [AWS Docs](https://docs.aws.amazon.com/devicefarm/latest/APIReference/API_CreateUpload.html#API_CreateUpload_RequestSyntax) for valid list of values.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The presigned Amazon S3 URL that was used to store a file using a PUT request.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UploadArgs,
    ) -> UploadResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_arn_binding = args.project_arn.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devicefarm/upload:Upload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "projectArn".into(),
                    value: &project_arn_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UploadResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            category: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("category"),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("projectArn"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            url: pulumi_wasm_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
