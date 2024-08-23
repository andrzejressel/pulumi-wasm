pub struct WorkerScriptArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    pub compatibility_flags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub kv_namespace_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
    pub plain_text_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
    pub queue_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
    pub r2_bucket_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
    pub secret_text_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
    pub service_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub webassembly_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
}

pub struct WorkerScriptResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub kv_namespace_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
    pub plain_text_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
    pub queue_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
    pub r2_bucket_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
    pub secret_text_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
    pub service_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
    pub tags: pulumi_wasm_rust::Output<Vec<String>>,
    pub webassembly_bindings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
}

pub fn create(name: &str, args: WorkerScriptArgs) -> WorkerScriptResult {
    let result = crate::bindings::pulumi::cloudflare::worker_script::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::worker_script::Args {
            account_id: args.account_id.get_inner(),
            analytics_engine_bindings: args.analytics_engine_bindings.get_inner(),
            compatibility_date: args.compatibility_date.get_inner(),
            compatibility_flags: args.compatibility_flags.get_inner(),
            content: args.content.get_inner(),
            d1_database_bindings: args.d1_database_bindings.get_inner(),
            dispatch_namespace: args.dispatch_namespace.get_inner(),
            kv_namespace_bindings: args.kv_namespace_bindings.get_inner(),
            logpush: args.logpush.get_inner(),
            module: args.module.get_inner(),
            name: args.name.get_inner(),
            placements: args.placements.get_inner(),
            plain_text_bindings: args.plain_text_bindings.get_inner(),
            queue_bindings: args.queue_bindings.get_inner(),
            r2_bucket_bindings: args.r2_bucket_bindings.get_inner(),
            secret_text_bindings: args.secret_text_bindings.get_inner(),
            service_bindings: args.service_bindings.get_inner(),
            tags: args.tags.get_inner(),
            webassembly_bindings: args.webassembly_bindings.get_inner(),
        },
    );

    WorkerScriptResult {
        account_id: crate::into_domain(result.account_id),
        analytics_engine_bindings: crate::into_domain(result.analytics_engine_bindings),
        compatibility_date: crate::into_domain(result.compatibility_date),
        compatibility_flags: crate::into_domain(result.compatibility_flags),
        content: crate::into_domain(result.content),
        d1_database_bindings: crate::into_domain(result.d1_database_bindings),
        dispatch_namespace: crate::into_domain(result.dispatch_namespace),
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
