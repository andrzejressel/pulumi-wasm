//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.List;
//! import com.pulumi.cloudflare.ListArgs;
//! import com.pulumi.cloudflare.inputs.ListItemArgs;
//! import com.pulumi.cloudflare.inputs.ListItemValueArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         // Hostname list
//!         var example = new List("example", ListArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("example hostnames for a list")
//!             .items(            
//!                 ListItemArgs.builder()
//!                     .comment("one")
//!                     .value(ListItemValueArgs.builder()
//!                         .hostname(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
//!                         .build())
//!                     .build(),
//!                 ListItemArgs.builder()
//!                     .comment("two")
//!                     .value(ListItemValueArgs.builder()
//!                         .hostname(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
//!                         .build())
//!                     .build())
//!             .kind("hostname")
//!             .name("example_list")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Hostname list
//!   example:
//!     type: cloudflare:List
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: example hostnames for a list
//!       items:
//!         - comment: one
//!           value:
//!             hostname:
//!               - urlHostname: example.com
//!         - comment: two
//!           value:
//!             hostname:
//!               - urlHostname: '*.example.com'
//!       kind: hostname
//!       name: example_list
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/list:List example <account_id>/<list_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    /// The type of items the list will contain. Available values: `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct ListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    /// The type of items the list will contain. Available values: `ip`, `redirect`, `hostname`, `asn`. **Modifying this attribute will force creation of a new resource.**
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ListArgs) -> ListResult {

    let result = crate::bindings::pulumi::cloudflare::list::invoke(name, &crate::bindings::pulumi::cloudflare::list::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        items: &args.items.get_inner(),
        kind: &args.kind.get_inner(),
        name: &args.name.get_inner(),
    });

    ListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
    }
}
