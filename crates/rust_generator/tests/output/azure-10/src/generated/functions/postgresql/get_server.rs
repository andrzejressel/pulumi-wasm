pub mod get_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerArgs {
        /// The name of the PostgreSQL Server.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the PostgreSQL Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerResult {
        /// The administrator username of the PostgreSQL Server.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the PostgreSQL Server.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::postgresql::GetServerIdentity>,
        >,
        /// The location of the Resource Group in which the PostgreSQL Server exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the PostgreSQL Server.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The version of the PostgreSQL Server.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServerArgs,
    ) -> GetServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:postgresql/getServer:getServer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administratorLogin"),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(o.extract_field("fqdn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
