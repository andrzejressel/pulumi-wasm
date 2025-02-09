#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tag_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagKeyArgs {
        /// The resource name of the parent organization or project. It can be in format `organizations/{org_id}` or `projects/{project_id_or_number}`.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tag key's short_name.
        #[builder(into)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTagKeyResult {
        /// Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// an identifier for the resource with format `tagKeys/{{name}}`
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The generated numeric id for the TagKey.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Namespaced name of the TagKey which is in the format `{parentNamespace}/{shortName}`.
        pub namespaced_name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// Update time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTagKeyArgs,
    ) -> GetTagKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_binding = args.parent.get_output(context);
        let short_name_binding = args.short_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:tags/getTagKey:getTagKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortName".into(),
                    value: short_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTagKeyResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            namespaced_name: o.get_field("namespacedName"),
            parent: o.get_field("parent"),
            short_name: o.get_field("shortName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
