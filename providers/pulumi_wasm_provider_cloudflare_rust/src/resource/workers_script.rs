//! Provides a Cloudflare worker script resource. In order for a script to be active, you'll also need to setup a `cloudflare.WorkerRoute`.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   myNamespace:
//!     type: cloudflare:WorkersKvNamespace
//!     name: my_namespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       title: example
//!   # Sets the script with the name "script_1"
//!   myScript:
//!     type: cloudflare:WorkersScript
//!     name: my_script
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: script_1
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: script.js
//!           Return: result
//!       kvNamespaceBindings:
//!         - name: MY_EXAMPLE_KV_NAMESPACE
//!           namespaceId: ${myNamespace.id}
//!       plainTextBindings:
//!         - name: MY_EXAMPLE_PLAIN_TEXT
//!           text: foobar
//!       secretTextBindings:
//!         - name: MY_EXAMPLE_SECRET_TEXT
//!           text: ${secretFooValue}
//!       webassemblyBindings:
//!         - name: MY_EXAMPLE_WASM
//!           module:
//!             fn::invoke:
//!               Function: std:filebase64
//!               Arguments:
//!                 input: example.wasm
//!               Return: result
//!       serviceBindings:
//!         - name: MY_SERVICE_BINDING
//!           service: MY_SERVICE
//!           environment: production
//!       r2BucketBindings:
//!         - name: MY_BUCKET
//!           bucketName: MY_BUCKET_NAME
//!       analyticsEngineBindings:
//!         - name: MY_DATASET
//!           dataset: dataset1
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersScript:WorkersScript example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptAnalyticsEngineBinding>>>,
    /// The date to use for the compatibility flag.
    #[builder(into, default)]
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    #[builder(into, default)]
    pub compatibility_flags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The script content.
    #[builder(into)]
    pub content: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    #[builder(into, default)]
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptHyperdriveConfigBinding>>>,
    #[builder(into, default)]
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    #[builder(into, default)]
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    #[builder(into, default)]
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlacement>>>,
    #[builder(into, default)]
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlainTextBinding>>>,
    #[builder(into, default)]
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptQueueBinding>>>,
    #[builder(into, default)]
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptR2BucketBinding>>>,
    #[builder(into, default)]
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptSecretTextBinding>>>,
    #[builder(into, default)]
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptServiceBinding>>>,
    #[builder(into, default)]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default)]
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptWebassemblyBinding>>>,
}

pub struct WorkersScriptResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptAnalyticsEngineBinding>>>,
    /// The date to use for the compatibility flag.
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
    /// The script content.
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptHyperdriveConfigBinding>>>,
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlacement>>>,
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlainTextBinding>>>,
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptQueueBinding>>>,
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptR2BucketBinding>>>,
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptSecretTextBinding>>>,
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptServiceBinding>>>,
    pub tags: pulumi_wasm_rust::Output<Vec<String>>,
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptWebassemblyBinding>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersScriptArgs) -> WorkersScriptResult {

    let result = crate::bindings::pulumi::cloudflare::workers_script::invoke(name, &crate::bindings::pulumi::cloudflare::workers_script::Args {
        account_id: &args.account_id.get_inner(),
        analytics_engine_bindings: &args.analytics_engine_bindings.get_inner(),
        compatibility_date: &args.compatibility_date.get_inner(),
        compatibility_flags: &args.compatibility_flags.get_inner(),
        content: &args.content.get_inner(),
        d1_database_bindings: &args.d1_database_bindings.get_inner(),
        dispatch_namespace: &args.dispatch_namespace.get_inner(),
        hyperdrive_config_bindings: &args.hyperdrive_config_bindings.get_inner(),
        kv_namespace_bindings: &args.kv_namespace_bindings.get_inner(),
        logpush: &args.logpush.get_inner(),
        module: &args.module.get_inner(),
        name: &args.name.get_inner(),
        placements: &args.placements.get_inner(),
        plain_text_bindings: &args.plain_text_bindings.get_inner(),
        queue_bindings: &args.queue_bindings.get_inner(),
        r2_bucket_bindings: &args.r2_bucket_bindings.get_inner(),
        secret_text_bindings: &args.secret_text_bindings.get_inner(),
        service_bindings: &args.service_bindings.get_inner(),
        tags: &args.tags.get_inner(),
        webassembly_bindings: &args.webassembly_bindings.get_inner(),
    });

    WorkersScriptResult {
        account_id: crate::into_domain(result.account_id),
        analytics_engine_bindings: crate::into_domain(result.analytics_engine_bindings),
        compatibility_date: crate::into_domain(result.compatibility_date),
        compatibility_flags: crate::into_domain(result.compatibility_flags),
        content: crate::into_domain(result.content),
        d1_database_bindings: crate::into_domain(result.d1_database_bindings),
        dispatch_namespace: crate::into_domain(result.dispatch_namespace),
        hyperdrive_config_bindings: crate::into_domain(result.hyperdrive_config_bindings),
        kv_namespace_bindings: crate::into_domain(result.kv_namespace_bindings),
        logpush: crate::into_domain(result.logpush),
        module: crate::into_domain(result.module),
        name: crate::into_domain(result.name),
        placements: crate::into_domain(result.placements),
        plain_text_bindings: crate::into_domain(result.plain_text_bindings),
        queue_bindings: crate::into_domain(result.queue_bindings),
        r2_bucket_bindings: crate::into_domain(result.r2_bucket_bindings),
        secret_text_bindings: crate::into_domain(result.secret_text_bindings),
        service_bindings: crate::into_domain(result.service_bindings),
        tags: crate::into_domain(result.tags),
        webassembly_bindings: crate::into_domain(result.webassembly_bindings),
    }
}
