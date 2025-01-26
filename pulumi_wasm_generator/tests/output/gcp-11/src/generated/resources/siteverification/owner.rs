/// An owner is an additional user that may manage a verified web site in the
/// [Google Search Console](https://www.google.com/webmasters/tools/). There
/// are two types of web resource owners:
///
/// * Verified owners, which are added to a web resource automatically when it
///     is created (i.e., when the resource is verified). A verified owner is
///     determined by the identity of the user requesting verification.
/// * Additional owners, which can be added to the resource by verified owners.
///
/// `gcp.siteverification.Owner` creates additional owners. If your web site
/// was verified using the
/// `gcp.siteverification.WebResource`
/// resource then you (or the identity was used to create the resource, such as a
/// service account) are already an owner.
///
/// > **Note:** The email address of the owner must belong to a Google account,
/// such as a Gmail account, a Google Workspace account, or a GCP service account.
///
/// Working with site verification requires the `https://www.googleapis.com/auth/siteverification`
/// authentication scope. See the
/// Google Provider authentication documentation
/// to learn how to configure additional scopes.
///
/// To get more information about site owners, see:
///
/// * [API documentation](https://developers.google.com/site-verification/v1)
/// * How-to Guides
///     * [Getting Started](https://developers.google.com/site-verification/v1/getting_started)
///
/// ## Example Usage
///
/// ### Site Verification Storage Bucket
///
/// This example uses the `FILE` verification method to verify ownership of web site hosted
/// in a Google Cloud Storage bucket. Ownership is proved by creating a file with a Google-provided
/// value in a known location. The user applying this configuration will automatically be
/// added as a verified owner, and the `gcp.siteverification.Owner` resource will add
/// `user@example.com` as an additional owner.
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: example-storage-bucket
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: ${token.token}
///       content: 'google-site-verification: ${token.token}'
///       bucket: ${bucket.name}
///   publicRule:
///     type: gcp:storage:ObjectAccessControl
///     name: public_rule
///     properties:
///       bucket: ${bucket.name}
///       object: ${object.name}
///       role: READER
///       entity: allUsers
///   example:
///     type: gcp:siteverification:WebResource
///     properties:
///       site:
///         type: ${token.type}
///         identifier: ${token.identifier}
///       verificationMethod: ${token.verificationMethod}
///   exampleOwner:
///     type: gcp:siteverification:Owner
///     name: example
///     properties:
///       webResourceId: ${example.id}
///       email: user@example.com
/// variables:
///   token:
///     fn::invoke:
///       function: gcp:siteverification:getToken
///       arguments:
///         type: SITE
///         identifier: https://${bucket.name}.storage.googleapis.com/
///         verificationMethod: FILE
/// ```
///
/// ## Import
///
/// Owner can be imported using this format:
///
/// * `webResource/{{web_resource_id}}/{{email}}`
///
/// When using the `pulumi import` command, Site owners can be imported using the format above. For example:
///
/// ```sh
/// $ pulumi import gcp:siteverification/owner:Owner default webResource/{{web_resource_id}}/{{email}}
/// ```
///
/// verified owners is to delete the web resource itself.
///
pub mod owner {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OwnerArgs {
        /// The email of the user to be added as an owner.
        ///
        /// - - -
        #[builder(into)]
        pub email: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of of the web resource to which the owner will be added, in the form `webResource/<resource_id>`,
        /// such as `webResource/https://www.example.com/`
        #[builder(into)]
        pub web_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OwnerResult {
        /// The email of the user to be added as an owner.
        ///
        /// - - -
        pub email: pulumi_wasm_rust::Output<String>,
        /// The id of of the web resource to which the owner will be added, in the form `webResource/<resource_id>`,
        /// such as `webResource/https://www.example.com/`
        pub web_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OwnerArgs,
    ) -> OwnerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_output(context).get_inner();
        let web_resource_id_binding = args
            .web_resource_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:siteverification/owner:Owner".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "webResourceId".into(),
                    value: &web_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "webResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OwnerResult {
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            web_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webResourceId").unwrap(),
            ),
        }
    }
}
