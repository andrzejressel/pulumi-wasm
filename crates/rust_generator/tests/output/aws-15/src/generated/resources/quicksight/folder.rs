/// Resource for managing a QuickSight Folder.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = folder::create(
///         "example",
///         FolderArgs::builder().folder_id("example-id").name("example-name").build_struct(),
///     );
/// }
/// ```
///
/// ### With Permissions
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = folder::create(
///         "example",
///         FolderArgs::builder()
///             .folder_id("example-id")
///             .name("example-name")
///             .permissions(
///                 vec![
///                     FolderPermission::builder().actions(vec!["quicksight:CreateFolder",
///                     "quicksight:DescribeFolder", "quicksight:UpdateFolder",
///                     "quicksight:DeleteFolder", "quicksight:CreateFolderMembership",
///                     "quicksight:DeleteFolderMembership",
///                     "quicksight:DescribeFolderPermissions",
///                     "quicksight:UpdateFolderPermissions",])
///                     .principal("${exampleAwsQuicksightUser.arn}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Parent Folder
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = folder::create(
///         "example",
///         FolderArgs::builder()
///             .folder_id("example-id")
///             .name("example-name")
///             .parent_folder_arn("${parent.arn}")
///             .build_struct(),
///     );
///     let parent = folder::create(
///         "parent",
///         FolderArgs::builder().folder_id("parent-id").name("parent-name").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight folder using the AWS account ID and folder ID name separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/folder:Folder example 123456789012,example-id
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod folder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier for the folder.
        #[builder(into)]
        pub folder_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of folder. By default, it is `SHARED`. Valid values are: `SHARED`.
        #[builder(into, default)]
        pub folder_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name for the folder.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the parent folder. If not set, creates a root-level folder.
        #[builder(into, default)]
        pub parent_folder_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of resource permissions on the folder. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::FolderPermission>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FolderResult {
        /// ARN of the folder.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The time that the folder was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the folder.
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// An array of ancestor ARN strings for the folder. Empty for root-level folders.
        pub folder_paths: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The type of folder. By default, it is `SHARED`. Valid values are: `SHARED`.
        pub folder_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time that the folder was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name for the folder.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the parent folder. If not set, creates a root-level folder.
        pub parent_folder_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of resource permissions on the folder. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::FolderPermission>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FolderArgs,
    ) -> FolderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let folder_id_binding = args.folder_id.get_output(context).get_inner();
        let folder_type_binding = args.folder_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_folder_arn_binding = args
            .parent_folder_arn
            .get_output(context)
            .get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/folder:Folder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding,
                },
                register_interface::ObjectField {
                    name: "folderType".into(),
                    value: &folder_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentFolderArn".into(),
                    value: &parent_folder_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FolderResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            folder_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folderId"),
            ),
            folder_paths: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folderPaths"),
            ),
            folder_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folderType"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent_folder_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentFolderArn"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
