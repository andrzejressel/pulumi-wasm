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
pub mod authorized_view {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizedViewArgs {
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Bigtable instance in which the authorized view belongs.
        #[builder(into)]
        pub instance_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An AuthorizedView permitting access to an explicit subset of a Table. Structure is documented below.
        ///
        /// -----
        #[builder(into, default)]
        pub subset_view: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigtable::AuthorizedViewSubsetView>,
        >,
        /// The name of the Bigtable table in which the authorized view belongs.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizedViewResult {
        pub deletion_protection: pulumi_wasm_rust::Output<String>,
        /// The name of the Bigtable instance in which the authorized view belongs.
        pub instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// An AuthorizedView permitting access to an explicit subset of a Table. Structure is documented below.
        ///
        /// -----
        pub subset_view: pulumi_wasm_rust::Output<
            Option<super::super::types::bigtable::AuthorizedViewSubsetView>,
        >,
        /// The name of the Bigtable table in which the authorized view belongs.
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthorizedViewArgs,
    ) -> AuthorizedViewResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let instance_name_binding = args.instance_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let subset_view_binding = args.subset_view.get_output(context).get_inner();
        let table_name_binding = args.table_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigtable/authorizedView:AuthorizedView".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subsetView".into(),
                    value: &subset_view_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AuthorizedViewResult {
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            instance_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            subset_view: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subsetView"),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableName"),
            ),
        }
    }
}
