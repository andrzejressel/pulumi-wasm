/// Provides an Elastic File System (EFS) access point.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = access_point::create(
///         "test",
///         AccessPointArgs::builder().file_system_id("${foo.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EFS access points using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:efs/accessPoint:AccessPoint test fsap-52a643fb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointArgs {
        /// ID of the file system for which the access point is intended.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Operating system user and group applied to all file system requests made using the access point. Detailed below.
        #[builder(into, default)]
        pub posix_user: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::efs::AccessPointPosixUser>,
        >,
        /// Directory on the Amazon EFS file system that the access point provides access to. Detailed below.
        #[builder(into, default)]
        pub root_directory: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::efs::AccessPointRootDirectory>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessPointResult {
        /// ARN of the access point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the file system.
        pub file_system_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the file system for which the access point is intended.
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Operating system user and group applied to all file system requests made using the access point. Detailed below.
        pub posix_user: pulumi_gestalt_rust::Output<
            Option<super::super::types::efs::AccessPointPosixUser>,
        >,
        /// Directory on the Amazon EFS file system that the access point provides access to. Detailed below.
        pub root_directory: pulumi_gestalt_rust::Output<
            super::super::types::efs::AccessPointRootDirectory,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
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
        args: AccessPointArgs,
    ) -> AccessPointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let file_system_id_binding = args.file_system_id.get_output(context);
        let posix_user_binding = args.posix_user.get_output(context);
        let root_directory_binding = args.root_directory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:efs/accessPoint:AccessPoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: file_system_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "posixUser".into(),
                    value: posix_user_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootDirectory".into(),
                    value: root_directory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPointResult {
            arn: o.get_field("arn"),
            file_system_arn: o.get_field("fileSystemArn"),
            file_system_id: o.get_field("fileSystemId"),
            owner_id: o.get_field("ownerId"),
            posix_user: o.get_field("posixUser"),
            root_directory: o.get_field("rootDirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
