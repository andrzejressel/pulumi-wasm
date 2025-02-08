#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_parameters_by_path {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetParametersByPathArgs {
        /// The hierarchy for the parameter. Hierarchies start with a forward slash (/). The hierarchy is the parameter name except the last part of the parameter. The last part of the parameter name can't be in the path. A parameter name hierarchy can have a maximum of 15 levels. **Note:** If the parameter name (e.g., `/my-app/my-param`) is specified, the data source will not retrieve any value as designed, unless there are other parameters that happen to use the former path in their hierarchy (e.g., `/my-app/my-param/my-actual-param`).
        #[builder(into)]
        pub path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to retrieve all parameters within the hirerachy. Defaults to `false`.
        #[builder(into, default)]
        pub recursive: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to retrieve all parameters in the hierarchy, particularly those of `SecureString` type, with their value decrypted. Defaults to `true`.
        #[builder(into, default)]
        pub with_decryption: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetParametersByPathResult {
        /// A list that contains the Amazon Resource Names (ARNs) of the retrieved parameters.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list that contains the names of the retrieved parameters.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub path: pulumi_gestalt_rust::Output<String>,
        pub recursive: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list that contains the types (`String`, `StringList`, or `SecureString`) of retrieved parameters.
        pub types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list that contains the retrieved parameter values. **Note:** This value is always marked as sensitive in the pulumi preview output, regardless of whether any retrieved parameters are of `SecureString` type. Use the `nonsensitive` function to override the behavior at your own risk and discretion, if you are certain that there are no sensitive values being retrieved.
        pub values: pulumi_gestalt_rust::Output<Vec<String>>,
        pub with_decryption: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetParametersByPathArgs,
    ) -> GetParametersByPathResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let path_binding = args.path.get_output(context).get_inner();
        let recursive_binding = args.recursive.get_output(context).get_inner();
        let with_decryption_binding = args
            .with_decryption
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getParametersByPath:getParametersByPath".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "recursive".into(),
                    value: &recursive_binding,
                },
                register_interface::ObjectField {
                    name: "withDecryption".into(),
                    value: &with_decryption_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetParametersByPathResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_gestalt_rust::__private::into_domain(o.extract_field("names")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            recursive: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recursive"),
            ),
            types: pulumi_gestalt_rust::__private::into_domain(o.extract_field("types")),
            values: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("values"),
            ),
            with_decryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("withDecryption"),
            ),
        }
    }
}
