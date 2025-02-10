/// A Lien represents an encumbrance on the actions that can be performed on a resource.
///
///
///
/// ## Example Usage
///
/// ### Resource Manager Lien
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lien = lien::create(
///         "lien",
///         LienArgs::builder()
///             .origin("machine-readable-explanation")
///             .parent("projects/${project.number}")
///             .reason("This project is an important environment")
///             .restrictions(vec!["resourcemanager.projects.delete",])
///             .build_struct(),
///     );
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("A very important project!")
///             .project_id("staging-project")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Lien can be imported using any of these accepted formats:
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Lien can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:resourcemanager/lien:Lien default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lien {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LienArgs {
        /// A stable, user-visible/meaningful string identifying the origin
        /// of the Lien, intended to be inspected programmatically. Maximum length of
        /// 200 characters.
        #[builder(into)]
        pub origin: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A reference to the resource this Lien is attached to.
        /// The server will validate the parent against those for which Liens are supported.
        /// Since a variety of objects can have Liens against them, you must provide the type
        /// prefix (e.g. "projects/my-project-name").
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Concise user-visible strings indicating why an action cannot be performed
        /// on a resource. Maximum length of 200 characters.
        #[builder(into)]
        pub reason: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The types of operations which should be blocked as a result of this Lien.
        /// Each value should correspond to an IAM permission. The server will validate
        /// the permissions against those for which Liens are supported.  An empty
        /// list is meaningless and will be rejected.
        /// e.g. ['resourcemanager.projects.delete']
        ///
        ///
        /// - - -
        #[builder(into)]
        pub restrictions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct LienResult {
        /// Time of creation
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A system-generated unique identifier for this Lien.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A stable, user-visible/meaningful string identifying the origin
        /// of the Lien, intended to be inspected programmatically. Maximum length of
        /// 200 characters.
        pub origin: pulumi_gestalt_rust::Output<String>,
        /// A reference to the resource this Lien is attached to.
        /// The server will validate the parent against those for which Liens are supported.
        /// Since a variety of objects can have Liens against them, you must provide the type
        /// prefix (e.g. "projects/my-project-name").
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Concise user-visible strings indicating why an action cannot be performed
        /// on a resource. Maximum length of 200 characters.
        pub reason: pulumi_gestalt_rust::Output<String>,
        /// The types of operations which should be blocked as a result of this Lien.
        /// Each value should correspond to an IAM permission. The server will validate
        /// the permissions against those for which Liens are supported.  An empty
        /// list is meaningless and will be rejected.
        /// e.g. ['resourcemanager.projects.delete']
        ///
        ///
        /// - - -
        pub restrictions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LienArgs,
    ) -> LienResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let origin_binding = args.origin.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let reason_binding = args.reason.get_output(context);
        let restrictions_binding = args.restrictions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:resourcemanager/lien:Lien".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "origin".into(),
                    value: origin_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reason".into(),
                    value: reason_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restrictions".into(),
                    value: restrictions_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LienResult {
            create_time: o.get_field("createTime"),
            name: o.get_field("name"),
            origin: o.get_field("origin"),
            parent: o.get_field("parent"),
            reason: o.get_field("reason"),
            restrictions: o.get_field("restrictions"),
        }
    }
}
