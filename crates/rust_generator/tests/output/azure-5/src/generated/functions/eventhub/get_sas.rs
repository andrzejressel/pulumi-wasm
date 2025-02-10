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
        context: &pulumi_gestalt_rust::Context,
        args: GetSasArgs,
    ) -> GetSasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_string_binding = args.connection_string.get_output(context);
        let expiry_binding = args.expiry.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:eventhub/getSas:getSas".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionString".into(),
                    value: connection_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiry".into(),
                    value: expiry_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSasResult {
            connection_string: o.get_field("connectionString"),
            expiry: o.get_field("expiry"),
            id: o.get_field("id"),
            sas: o.get_field("sas"),
        }
    }
}
