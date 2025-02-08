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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAclArgs,
    ) -> GetAclResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:memorydb/getAcl:getAcl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAclResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            minimum_engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumEngineVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            user_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userNames"),
            ),
        }
    }
}
