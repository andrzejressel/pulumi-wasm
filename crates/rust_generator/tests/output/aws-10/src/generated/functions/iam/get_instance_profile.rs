#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceProfileArgs {
        /// Friendly IAM instance profile name to match.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceProfileResult {
        /// ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// String representation of the date the instance profile was created.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path to the instance profile.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Role ARN associated with this instance profile.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Role ID associated with this instance profile.
        pub role_id: pulumi_gestalt_rust::Output<String>,
        /// Role name associated with this instance profile.
        pub role_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceProfileArgs,
    ) -> GetInstanceProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getInstanceProfile:getInstanceProfile".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceProfileResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            create_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            role_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleId"),
            ),
            role_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleName"),
            ),
        }
    }
}
