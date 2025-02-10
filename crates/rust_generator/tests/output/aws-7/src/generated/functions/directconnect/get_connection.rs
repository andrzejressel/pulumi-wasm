#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// Name of the connection to retrieve.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        /// ARN of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Direct Connect endpoint on which the physical connection terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// Bandwidth of the connection.
        pub bandwidth: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// AWS Direct Connect location where the connection is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the connection.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the AWS Direct Connect service provider associated with the connection.
        pub partner_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the service provider associated with the connection.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The VLAN ID.
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directconnect/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectionResult {
            arn: o.get_field("arn"),
            aws_device: o.get_field("awsDevice"),
            bandwidth: o.get_field("bandwidth"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
            partner_name: o.get_field("partnerName"),
            provider_name: o.get_field("providerName"),
            tags: o.get_field("tags"),
            vlan_id: o.get_field("vlanId"),
        }
    }
}
