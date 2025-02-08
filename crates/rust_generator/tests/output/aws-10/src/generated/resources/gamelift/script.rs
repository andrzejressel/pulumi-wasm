/// Provides an GameLift Script resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = script::create(
///         "example",
///         ScriptArgs::builder()
///             .name("example-script")
///             .storage_location(
///                 ScriptStorageLocation::builder()
///                     .bucket("${exampleAwsS3Bucket.id}")
///                     .key("${exampleAwsS3Object.key}")
///                     .roleArn("${exampleAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Scripts using the ID. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/script:Script example <script-id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod script {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScriptArgs {
        /// Name of the script
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information indicating where your game script files are stored. See below.
        #[builder(into, default)]
        pub storage_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gamelift::ScriptStorageLocation>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Version that is associated with this script.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A data object containing your Realtime scripts and dependencies as a zip  file. The zip file can have one or multiple files. Maximum size of a zip file is 5 MB.
        #[builder(into, default)]
        pub zip_file: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScriptResult {
        /// GameLift Script ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the script
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Information indicating where your game script files are stored. See below.
        pub storage_location: pulumi_gestalt_rust::Output<
            super::super::types::gamelift::ScriptStorageLocation,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version that is associated with this script.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// A data object containing your Realtime scripts and dependencies as a zip  file. The zip file can have one or multiple files. Maximum size of a zip file is 5 MB.
        pub zip_file: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScriptArgs,
    ) -> ScriptResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let storage_location_binding = args
            .storage_location
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let zip_file_binding = args.zip_file.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/script:Script".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageLocation".into(),
                    value: &storage_location_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "zipFile".into(),
                    value: &zip_file_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScriptResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            storage_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageLocation"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            zip_file: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zipFile"),
            ),
        }
    }
}
