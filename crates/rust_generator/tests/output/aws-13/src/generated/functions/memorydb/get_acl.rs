#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAclArgs {
        /// Name of the ACL.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the ACL.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAclResult {
        /// ARN of the ACL.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The minimum engine version supported by the ACL.
        pub minimum_engine_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the ACL.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Set of MemoryDB user names included in this ACL.
        pub user_names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAclArgs,
    ) -> GetAclResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:memorydb/getAcl:getAcl".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAclResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            minimum_engine_version: o.get_field("minimumEngineVersion"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            user_names: o.get_field("userNames"),
        }
    }
}
