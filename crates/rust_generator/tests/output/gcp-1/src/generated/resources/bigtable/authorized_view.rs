/// ## Example Usage
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:bigtable:Instance
///     properties:
///       name: tf-instance
///       clusters:
///         - clusterId: tf-instance-cluster
///           zone: us-central1-b
///           numNodes: 3
///           storageType: HDD
///   table:
///     type: gcp:bigtable:Table
///     properties:
///       name: tf-table
///       instanceName: ${instance.name}
///       splitKeys:
///         - a
///         - b
///         - c
///       columnFamilies:
///         - family: family-first
///         - family: family-second
///       changeStreamRetention: 24h0m0s
///   authorizedView:
///     type: gcp:bigtable:AuthorizedView
///     name: authorized_view
///     properties:
///       name: tf-authorized-view
///       instanceName: ${instance.name}
///       tableName: ${table.name}
///       subsetView:
///         rowPrefixes:
///           - fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: prefix#
///               return: result
///         familySubsets:
///           - familyName: family-first
///             qualifiers:
///               - fn::invoke:
///                   function: std:base64encode
///                   arguments:
///                     input: qualifier
///                   return: result
///               - fn::invoke:
///                   function: std:base64encode
///                   arguments:
///                     input: qualifier-second
///                   return: result
///           - familyName: family-second
///             qualifierPrefixes:
///               - ""
/// ```
///
/// ## Import
///
/// Bigtable Authorized Views can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance_name}}/tables/{{table_name}}/authorizedViews/{{name}}`
///
/// * `{{project}}/{{instance_name}}/{{table_name}}/{{name}}`
///
/// * `{{instance_name}}/{{table_name}}/{{name}}`
///
/// When using the `pulumi import` command, Bigtable Authorized Views can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigtable/authorizedView:AuthorizedView default projects/{{project}}/instances/{{instance_name}}/tables/{{table_name}}/authorizedViews/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigtable/authorizedView:AuthorizedView default {{project}}/{{instance_name}}/{{table_name}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigtable/authorizedView:AuthorizedView default {{instance_name}}/{{table_name}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorized_view {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizedViewArgs {
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Bigtable instance in which the authorized view belongs.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An AuthorizedView permitting access to an explicit subset of a Table. Structure is documented below.
        ///
        /// -----
        #[builder(into, default)]
        pub subset_view: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigtable::AuthorizedViewSubsetView>,
        >,
        /// The name of the Bigtable table in which the authorized view belongs.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizedViewResult {
        pub deletion_protection: pulumi_gestalt_rust::Output<String>,
        /// The name of the Bigtable instance in which the authorized view belongs.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// An AuthorizedView permitting access to an explicit subset of a Table. Structure is documented below.
        ///
        /// -----
        pub subset_view: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigtable::AuthorizedViewSubsetView>,
        >,
        /// The name of the Bigtable table in which the authorized view belongs.
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizedViewArgs,
    ) -> AuthorizedViewResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let instance_name_binding = args.instance_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let subset_view_binding = args.subset_view.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigtable/authorizedView:AuthorizedView".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subsetView".into(),
                    value: &subset_view_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizedViewResult {
            deletion_protection: o.get_field("deletionProtection"),
            instance_name: o.get_field("instanceName"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            subset_view: o.get_field("subsetView"),
            table_name: o.get_field("tableName"),
        }
    }
}
