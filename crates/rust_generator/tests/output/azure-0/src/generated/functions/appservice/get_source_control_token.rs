#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_source_control_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSourceControlTokenArgs {
        /// The Token type. Possible values include `Bitbucket`, `Dropbox`, `Github`, and `OneDrive`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSourceControlTokenResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The GitHub Token value.
        pub token: pulumi_gestalt_rust::Output<String>,
        pub token_secret: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSourceControlTokenArgs,
    ) -> GetSourceControlTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getSourceControlToken:getSourceControlToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSourceControlTokenResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            token: pulumi_gestalt_rust::__private::into_domain(o.extract_field("token")),
            token_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tokenSecret"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
