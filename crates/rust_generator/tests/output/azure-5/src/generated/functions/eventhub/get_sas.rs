#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sas {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSasArgs {
        /// The connection string for the Event Hub to which this SAS applies.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub expiry: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSasResult {
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        pub expiry: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The computed Event Hub Shared Access Signature (SAS).
        pub sas: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSasArgs,
    ) -> GetSasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let connection_string_binding_1 = args.connection_string.get_output(context);
        let connection_string_binding = connection_string_binding_1.get_inner();
        let expiry_binding_1 = args.expiry.get_output(context);
        let expiry_binding = expiry_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getSas:getSas".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "expiry".into(),
                    value: &expiry_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSasResult {
            connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            expiry: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiry"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            sas: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sas")),
        }
    }
}
