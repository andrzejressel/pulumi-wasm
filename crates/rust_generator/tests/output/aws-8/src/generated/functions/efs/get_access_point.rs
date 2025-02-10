#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPointArgs {
        /// ID that identifies the file system.
        #[builder(into)]
        pub access_point_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAccessPointResult {
        pub access_point_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub file_system_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the file system for which the access point is intended.
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Single element list containing operating system user and group applied to all file system requests made using the access point.
        pub posix_users: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::efs::GetAccessPointPosixUser>,
        >,
        /// Single element list containing information on the directory on the Amazon EFS file system that the access point provides access to.
        pub root_directories: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::efs::GetAccessPointRootDirectory>,
        >,
        /// Key-value mapping of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccessPointArgs,
    ) -> GetAccessPointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_point_id_binding = args.access_point_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:efs/getAccessPoint:getAccessPoint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPointId".into(),
                    value: access_point_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccessPointResult {
            access_point_id: o.get_field("accessPointId"),
            arn: o.get_field("arn"),
            file_system_arn: o.get_field("fileSystemArn"),
            file_system_id: o.get_field("fileSystemId"),
            id: o.get_field("id"),
            owner_id: o.get_field("ownerId"),
            posix_users: o.get_field("posixUsers"),
            root_directories: o.get_field("rootDirectories"),
            tags: o.get_field("tags"),
        }
    }
}
