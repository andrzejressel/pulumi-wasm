#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod pulumi_wasm {
        #[allow(dead_code, clippy::all)]
        pub mod output_interface {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Output {
                handle: _rt::Resource<Output>,
            }
            impl Output {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Output {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "component:pulumi-wasm/output-interface@0.0.0-DEV"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]output"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(value: &str) -> Self {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "component:pulumi-wasm/output-interface@0.0.0-DEV"
                        )]
                        extern "C" {
                            #[link_name = "[constructor]output"]
                            fn wit_import(_: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(ptr0.cast_mut(), len0);
                        Output::from_handle(ret as u32)
                    }
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn map(&self, function_name: &str) -> Output {
                    unsafe {
                        let vec0 = function_name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "component:pulumi-wasm/output-interface@0.0.0-DEV"
                        )]
                        extern "C" {
                            #[link_name = "[method]output.map"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                        );
                        Output::from_handle(ret as u32)
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn combine(outputs: &[&Output]) -> Output {
                unsafe {
                    let vec0 = outputs;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm/output-interface@0.0.0-DEV"
                    )]
                    extern "C" {
                        #[link_name = "combine"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(result0, len0);
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    Output::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod register_interface {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Output = super::super::super::component::pulumi_wasm::output_interface::Output;
            pub struct ObjectField<'a> {
                pub name: _rt::String,
                pub value: &'a Output,
            }
            impl<'a> ::core::fmt::Debug for ObjectField<'a> {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ObjectField")
                        .field("name", &self.name)
                        .field("value", &self.value)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct ResultField {
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for ResultField {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ResultField").field("name", &self.name).finish()
                }
            }
            pub struct RegisterResourceResultField {
                pub name: _rt::String,
                pub output: Output,
            }
            impl ::core::fmt::Debug for RegisterResourceResultField {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RegisterResourceResultField")
                        .field("name", &self.name)
                        .field("output", &self.output)
                        .finish()
                }
            }
            pub struct RegisterResourceRequest<'a> {
                pub type_: _rt::String,
                pub name: _rt::String,
                pub object: _rt::Vec<ObjectField<'a>>,
                pub results: _rt::Vec<ResultField>,
            }
            impl<'a> ::core::fmt::Debug for RegisterResourceRequest<'a> {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RegisterResourceRequest")
                        .field("type", &self.type_)
                        .field("name", &self.name)
                        .field("object", &self.object)
                        .field("results", &self.results)
                        .finish()
                }
            }
            pub struct RegisterResourceResult {
                pub fields: _rt::Vec<RegisterResourceResultField>,
            }
            impl ::core::fmt::Debug for RegisterResourceResult {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RegisterResourceResult")
                        .field("fields", &self.fields)
                        .finish()
                }
            }
            pub struct ResourceInvokeResultField {
                pub name: _rt::String,
                pub output: Output,
            }
            impl ::core::fmt::Debug for ResourceInvokeResultField {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ResourceInvokeResultField")
                        .field("name", &self.name)
                        .field("output", &self.output)
                        .finish()
                }
            }
            pub struct ResourceInvokeRequest<'a> {
                pub token: _rt::String,
                pub object: _rt::Vec<ObjectField<'a>>,
                pub results: _rt::Vec<ResultField>,
            }
            impl<'a> ::core::fmt::Debug for ResourceInvokeRequest<'a> {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ResourceInvokeRequest")
                        .field("token", &self.token)
                        .field("object", &self.object)
                        .field("results", &self.results)
                        .finish()
                }
            }
            pub struct ResourceInvokeResult {
                pub fields: _rt::Vec<ResourceInvokeResultField>,
            }
            impl ::core::fmt::Debug for ResourceInvokeResult {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ResourceInvokeResult")
                        .field("fields", &self.fields)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn register(
                request: &RegisterResourceRequest<'_>,
            ) -> RegisterResourceResult {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let RegisterResourceRequest {
                        type_: type_0,
                        name: name0,
                        object: object0,
                        results: results0,
                    } = request;
                    let vec1 = type_0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = name0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    let vec5 = object0;
                    let len5 = vec5.len();
                    let layout5 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec5.len() * 12,
                        4,
                    );
                    let result5 = if layout5.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout5);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec5.into_iter().enumerate() {
                        let base = result5.add(i * 12);
                        {
                            let ObjectField { name: name3, value: value3 } = e;
                            let vec4 = name3;
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            *base.add(4).cast::<usize>() = len4;
                            *base.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                            *base.add(8).cast::<i32>() = (value3).handle() as i32;
                        }
                    }
                    let vec8 = results0;
                    let len8 = vec8.len();
                    let layout8 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec8.len() * 8,
                        4,
                    );
                    let result8 = if layout8.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout8).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout8);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec8.into_iter().enumerate() {
                        let base = result8.add(i * 8);
                        {
                            let ResultField { name: name6 } = e;
                            let vec7 = name6;
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            *base.add(4).cast::<usize>() = len7;
                            *base.add(0).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                    }
                    let ptr9 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm/register-interface@0.0.0-DEV"
                    )]
                    extern "C" {
                        #[link_name = "register"]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        ptr1.cast_mut(),
                        len1,
                        ptr2.cast_mut(),
                        len2,
                        result5,
                        len5,
                        result8,
                        len8,
                        ptr9,
                    );
                    let l10 = *ptr9.add(0).cast::<*mut u8>();
                    let l11 = *ptr9.add(4).cast::<usize>();
                    let base16 = l10;
                    let len16 = l11;
                    let mut result16 = _rt::Vec::with_capacity(len16);
                    for i in 0..len16 {
                        let base = base16.add(i * 12);
                        let e16 = {
                            let l12 = *base.add(0).cast::<*mut u8>();
                            let l13 = *base.add(4).cast::<usize>();
                            let len14 = l13;
                            let bytes14 = _rt::Vec::from_raw_parts(
                                l12.cast(),
                                len14,
                                len14,
                            );
                            let l15 = *base.add(8).cast::<i32>();
                            RegisterResourceResultField {
                                name: _rt::string_lift(bytes14),
                                output: super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l15 as u32,
                                ),
                            }
                        };
                        result16.push(e16);
                    }
                    _rt::cabi_dealloc(base16, len16 * 12, 4);
                    if layout5.size() != 0 {
                        _rt::alloc::dealloc(result5.cast(), layout5);
                    }
                    if layout8.size() != 0 {
                        _rt::alloc::dealloc(result8.cast(), layout8);
                    }
                    RegisterResourceResult {
                        fields: result16,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn invoke(request: &ResourceInvokeRequest<'_>) -> ResourceInvokeResult {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ResourceInvokeRequest {
                        token: token0,
                        object: object0,
                        results: results0,
                    } = request;
                    let vec1 = token0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec4 = object0;
                    let len4 = vec4.len();
                    let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec4.len() * 12,
                        4,
                    );
                    let result4 = if layout4.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout4);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec4.into_iter().enumerate() {
                        let base = result4.add(i * 12);
                        {
                            let ObjectField { name: name2, value: value2 } = e;
                            let vec3 = name2;
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            *base.add(4).cast::<usize>() = len3;
                            *base.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                            *base.add(8).cast::<i32>() = (value2).handle() as i32;
                        }
                    }
                    let vec7 = results0;
                    let len7 = vec7.len();
                    let layout7 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec7.len() * 8,
                        4,
                    );
                    let result7 = if layout7.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout7);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7.add(i * 8);
                        {
                            let ResultField { name: name5 } = e;
                            let vec6 = name5;
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            *base.add(4).cast::<usize>() = len6;
                            *base.add(0).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                    }
                    let ptr8 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm/register-interface@0.0.0-DEV"
                    )]
                    extern "C" {
                        #[link_name = "invoke"]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        ptr1.cast_mut(),
                        len1,
                        result4,
                        len4,
                        result7,
                        len7,
                        ptr8,
                    );
                    let l9 = *ptr8.add(0).cast::<*mut u8>();
                    let l10 = *ptr8.add(4).cast::<usize>();
                    let base15 = l9;
                    let len15 = l10;
                    let mut result15 = _rt::Vec::with_capacity(len15);
                    for i in 0..len15 {
                        let base = base15.add(i * 12);
                        let e15 = {
                            let l11 = *base.add(0).cast::<*mut u8>();
                            let l12 = *base.add(4).cast::<usize>();
                            let len13 = l12;
                            let bytes13 = _rt::Vec::from_raw_parts(
                                l11.cast(),
                                len13,
                                len13,
                            );
                            let l14 = *base.add(8).cast::<i32>();
                            ResourceInvokeResultField {
                                name: _rt::string_lift(bytes13),
                                output: super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l14 as u32,
                                ),
                            }
                        };
                        result15.push(e15);
                    }
                    _rt::cabi_dealloc(base15, len15 * 12, 4);
                    if layout4.size() != 0 {
                        _rt::alloc::dealloc(result4.cast(), layout4);
                    }
                    if layout7.size() != 0 {
                        _rt::alloc::dealloc(result7.cast(), layout7);
                    }
                    ResourceInvokeResult {
                        fields: result15,
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod pulumi {
        #[allow(dead_code)]
        pub mod docker {
            #[allow(dead_code, clippy::all)]
            pub mod container {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub attach: &'a Output,
                    pub capabilities: &'a Output,
                    pub cgroupns_mode: &'a Output,
                    pub command: &'a Output,
                    pub container_read_refresh_timeout_milliseconds: &'a Output,
                    pub cpu_set: &'a Output,
                    pub cpu_shares: &'a Output,
                    pub destroy_grace_seconds: &'a Output,
                    pub devices: &'a Output,
                    pub dns: &'a Output,
                    pub dns_opts: &'a Output,
                    pub dns_searches: &'a Output,
                    pub domainname: &'a Output,
                    pub entrypoints: &'a Output,
                    pub envs: &'a Output,
                    pub gpus: &'a Output,
                    pub group_adds: &'a Output,
                    pub healthcheck: &'a Output,
                    pub hostname: &'a Output,
                    pub hosts: &'a Output,
                    pub image: &'a Output,
                    pub init: &'a Output,
                    pub ipc_mode: &'a Output,
                    pub labels: &'a Output,
                    pub log_driver: &'a Output,
                    pub log_opts: &'a Output,
                    pub logs: &'a Output,
                    pub max_retry_count: &'a Output,
                    pub memory: &'a Output,
                    pub memory_swap: &'a Output,
                    pub mounts: &'a Output,
                    pub must_run: &'a Output,
                    pub name: &'a Output,
                    pub network_mode: &'a Output,
                    pub networks_advanced: &'a Output,
                    pub pid_mode: &'a Output,
                    pub ports: &'a Output,
                    pub privileged: &'a Output,
                    pub publish_all_ports: &'a Output,
                    pub read_only: &'a Output,
                    pub remove_volumes: &'a Output,
                    pub restart: &'a Output,
                    pub rm: &'a Output,
                    pub runtime: &'a Output,
                    pub security_opts: &'a Output,
                    pub shm_size: &'a Output,
                    pub start: &'a Output,
                    pub stdin_open: &'a Output,
                    pub stop_signal: &'a Output,
                    pub stop_timeout: &'a Output,
                    pub storage_opts: &'a Output,
                    pub sysctls: &'a Output,
                    pub tmpfs: &'a Output,
                    pub tty: &'a Output,
                    pub ulimits: &'a Output,
                    pub uploads: &'a Output,
                    pub user: &'a Output,
                    pub userns_mode: &'a Output,
                    pub volumes: &'a Output,
                    pub wait: &'a Output,
                    pub wait_timeout: &'a Output,
                    pub working_dir: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("attach", &self.attach)
                            .field("capabilities", &self.capabilities)
                            .field("cgroupns-mode", &self.cgroupns_mode)
                            .field("command", &self.command)
                            .field(
                                "container-read-refresh-timeout-milliseconds",
                                &self.container_read_refresh_timeout_milliseconds,
                            )
                            .field("cpu-set", &self.cpu_set)
                            .field("cpu-shares", &self.cpu_shares)
                            .field("destroy-grace-seconds", &self.destroy_grace_seconds)
                            .field("devices", &self.devices)
                            .field("dns", &self.dns)
                            .field("dns-opts", &self.dns_opts)
                            .field("dns-searches", &self.dns_searches)
                            .field("domainname", &self.domainname)
                            .field("entrypoints", &self.entrypoints)
                            .field("envs", &self.envs)
                            .field("gpus", &self.gpus)
                            .field("group-adds", &self.group_adds)
                            .field("healthcheck", &self.healthcheck)
                            .field("hostname", &self.hostname)
                            .field("hosts", &self.hosts)
                            .field("image", &self.image)
                            .field("init", &self.init)
                            .field("ipc-mode", &self.ipc_mode)
                            .field("labels", &self.labels)
                            .field("log-driver", &self.log_driver)
                            .field("log-opts", &self.log_opts)
                            .field("logs", &self.logs)
                            .field("max-retry-count", &self.max_retry_count)
                            .field("memory", &self.memory)
                            .field("memory-swap", &self.memory_swap)
                            .field("mounts", &self.mounts)
                            .field("must-run", &self.must_run)
                            .field("name", &self.name)
                            .field("network-mode", &self.network_mode)
                            .field("networks-advanced", &self.networks_advanced)
                            .field("pid-mode", &self.pid_mode)
                            .field("ports", &self.ports)
                            .field("privileged", &self.privileged)
                            .field("publish-all-ports", &self.publish_all_ports)
                            .field("read-only", &self.read_only)
                            .field("remove-volumes", &self.remove_volumes)
                            .field("restart", &self.restart)
                            .field("rm", &self.rm)
                            .field("runtime", &self.runtime)
                            .field("security-opts", &self.security_opts)
                            .field("shm-size", &self.shm_size)
                            .field("start", &self.start)
                            .field("stdin-open", &self.stdin_open)
                            .field("stop-signal", &self.stop_signal)
                            .field("stop-timeout", &self.stop_timeout)
                            .field("storage-opts", &self.storage_opts)
                            .field("sysctls", &self.sysctls)
                            .field("tmpfs", &self.tmpfs)
                            .field("tty", &self.tty)
                            .field("ulimits", &self.ulimits)
                            .field("uploads", &self.uploads)
                            .field("user", &self.user)
                            .field("userns-mode", &self.userns_mode)
                            .field("volumes", &self.volumes)
                            .field("wait", &self.wait)
                            .field("wait-timeout", &self.wait_timeout)
                            .field("working-dir", &self.working_dir)
                            .finish()
                    }
                }
                pub struct Res {
                    pub attach: Output,
                    pub bridge: Output,
                    pub capabilities: Output,
                    pub cgroupns_mode: Output,
                    pub command: Output,
                    pub container_logs: Output,
                    pub container_read_refresh_timeout_milliseconds: Output,
                    pub cpu_set: Output,
                    pub cpu_shares: Output,
                    pub destroy_grace_seconds: Output,
                    pub devices: Output,
                    pub dns: Output,
                    pub dns_opts: Output,
                    pub dns_searches: Output,
                    pub domainname: Output,
                    pub entrypoints: Output,
                    pub envs: Output,
                    pub exit_code: Output,
                    pub gpus: Output,
                    pub group_adds: Output,
                    pub healthcheck: Output,
                    pub hostname: Output,
                    pub hosts: Output,
                    pub image: Output,
                    pub init: Output,
                    pub ipc_mode: Output,
                    pub labels: Output,
                    pub log_driver: Output,
                    pub log_opts: Output,
                    pub logs: Output,
                    pub max_retry_count: Output,
                    pub memory: Output,
                    pub memory_swap: Output,
                    pub mounts: Output,
                    pub must_run: Output,
                    pub name: Output,
                    pub network_datas: Output,
                    pub network_mode: Output,
                    pub networks_advanced: Output,
                    pub pid_mode: Output,
                    pub ports: Output,
                    pub privileged: Output,
                    pub publish_all_ports: Output,
                    pub read_only: Output,
                    pub remove_volumes: Output,
                    pub restart: Output,
                    pub rm: Output,
                    pub runtime: Output,
                    pub security_opts: Output,
                    pub shm_size: Output,
                    pub start: Output,
                    pub stdin_open: Output,
                    pub stop_signal: Output,
                    pub stop_timeout: Output,
                    pub storage_opts: Output,
                    pub sysctls: Output,
                    pub tmpfs: Output,
                    pub tty: Output,
                    pub ulimits: Output,
                    pub uploads: Output,
                    pub user: Output,
                    pub userns_mode: Output,
                    pub volumes: Output,
                    pub wait: Output,
                    pub wait_timeout: Output,
                    pub working_dir: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("attach", &self.attach)
                            .field("bridge", &self.bridge)
                            .field("capabilities", &self.capabilities)
                            .field("cgroupns-mode", &self.cgroupns_mode)
                            .field("command", &self.command)
                            .field("container-logs", &self.container_logs)
                            .field(
                                "container-read-refresh-timeout-milliseconds",
                                &self.container_read_refresh_timeout_milliseconds,
                            )
                            .field("cpu-set", &self.cpu_set)
                            .field("cpu-shares", &self.cpu_shares)
                            .field("destroy-grace-seconds", &self.destroy_grace_seconds)
                            .field("devices", &self.devices)
                            .field("dns", &self.dns)
                            .field("dns-opts", &self.dns_opts)
                            .field("dns-searches", &self.dns_searches)
                            .field("domainname", &self.domainname)
                            .field("entrypoints", &self.entrypoints)
                            .field("envs", &self.envs)
                            .field("exit-code", &self.exit_code)
                            .field("gpus", &self.gpus)
                            .field("group-adds", &self.group_adds)
                            .field("healthcheck", &self.healthcheck)
                            .field("hostname", &self.hostname)
                            .field("hosts", &self.hosts)
                            .field("image", &self.image)
                            .field("init", &self.init)
                            .field("ipc-mode", &self.ipc_mode)
                            .field("labels", &self.labels)
                            .field("log-driver", &self.log_driver)
                            .field("log-opts", &self.log_opts)
                            .field("logs", &self.logs)
                            .field("max-retry-count", &self.max_retry_count)
                            .field("memory", &self.memory)
                            .field("memory-swap", &self.memory_swap)
                            .field("mounts", &self.mounts)
                            .field("must-run", &self.must_run)
                            .field("name", &self.name)
                            .field("network-datas", &self.network_datas)
                            .field("network-mode", &self.network_mode)
                            .field("networks-advanced", &self.networks_advanced)
                            .field("pid-mode", &self.pid_mode)
                            .field("ports", &self.ports)
                            .field("privileged", &self.privileged)
                            .field("publish-all-ports", &self.publish_all_ports)
                            .field("read-only", &self.read_only)
                            .field("remove-volumes", &self.remove_volumes)
                            .field("restart", &self.restart)
                            .field("rm", &self.rm)
                            .field("runtime", &self.runtime)
                            .field("security-opts", &self.security_opts)
                            .field("shm-size", &self.shm_size)
                            .field("start", &self.start)
                            .field("stdin-open", &self.stdin_open)
                            .field("stop-signal", &self.stop_signal)
                            .field("stop-timeout", &self.stop_timeout)
                            .field("storage-opts", &self.storage_opts)
                            .field("sysctls", &self.sysctls)
                            .field("tmpfs", &self.tmpfs)
                            .field("tty", &self.tty)
                            .field("ulimits", &self.ulimits)
                            .field("uploads", &self.uploads)
                            .field("user", &self.user)
                            .field("userns-mode", &self.userns_mode)
                            .field("volumes", &self.volumes)
                            .field("wait", &self.wait)
                            .field("wait-timeout", &self.wait_timeout)
                            .field("working-dir", &self.working_dir)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle4;
                    let handle6;
                    let handle8;
                    let handle10;
                    let handle12;
                    let handle14;
                    let handle16;
                    let handle18;
                    let handle20;
                    let handle22;
                    let handle24;
                    let handle26;
                    let handle28;
                    let handle30;
                    let handle32;
                    let handle34;
                    let handle36;
                    let handle38;
                    let handle40;
                    let handle42;
                    let handle44;
                    let handle46;
                    let handle48;
                    let handle50;
                    let handle52;
                    let handle54;
                    let handle56;
                    let handle58;
                    let handle60;
                    let handle62;
                    let handle64;
                    let handle66;
                    let handle68;
                    let handle70;
                    let handle72;
                    let handle74;
                    let handle76;
                    let handle78;
                    let handle80;
                    let handle82;
                    let handle84;
                    let handle86;
                    let handle88;
                    let handle90;
                    let handle92;
                    let handle94;
                    let handle96;
                    let handle98;
                    let handle100;
                    let handle102;
                    let handle104;
                    let handle106;
                    let handle108;
                    let handle110;
                    let handle112;
                    let handle114;
                    let handle116;
                    let handle118;
                    let handle120;
                    let handle122;
                    let handle124;
                    let handle126;
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let len2 = l1;
                    let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                    let l3 = *arg0.add(8).cast::<i32>();
                    let l5 = *arg0.add(12).cast::<i32>();
                    let l7 = *arg0.add(16).cast::<i32>();
                    let l9 = *arg0.add(20).cast::<i32>();
                    let l11 = *arg0.add(24).cast::<i32>();
                    let l13 = *arg0.add(28).cast::<i32>();
                    let l15 = *arg0.add(32).cast::<i32>();
                    let l17 = *arg0.add(36).cast::<i32>();
                    let l19 = *arg0.add(40).cast::<i32>();
                    let l21 = *arg0.add(44).cast::<i32>();
                    let l23 = *arg0.add(48).cast::<i32>();
                    let l25 = *arg0.add(52).cast::<i32>();
                    let l27 = *arg0.add(56).cast::<i32>();
                    let l29 = *arg0.add(60).cast::<i32>();
                    let l31 = *arg0.add(64).cast::<i32>();
                    let l33 = *arg0.add(68).cast::<i32>();
                    let l35 = *arg0.add(72).cast::<i32>();
                    let l37 = *arg0.add(76).cast::<i32>();
                    let l39 = *arg0.add(80).cast::<i32>();
                    let l41 = *arg0.add(84).cast::<i32>();
                    let l43 = *arg0.add(88).cast::<i32>();
                    let l45 = *arg0.add(92).cast::<i32>();
                    let l47 = *arg0.add(96).cast::<i32>();
                    let l49 = *arg0.add(100).cast::<i32>();
                    let l51 = *arg0.add(104).cast::<i32>();
                    let l53 = *arg0.add(108).cast::<i32>();
                    let l55 = *arg0.add(112).cast::<i32>();
                    let l57 = *arg0.add(116).cast::<i32>();
                    let l59 = *arg0.add(120).cast::<i32>();
                    let l61 = *arg0.add(124).cast::<i32>();
                    let l63 = *arg0.add(128).cast::<i32>();
                    let l65 = *arg0.add(132).cast::<i32>();
                    let l67 = *arg0.add(136).cast::<i32>();
                    let l69 = *arg0.add(140).cast::<i32>();
                    let l71 = *arg0.add(144).cast::<i32>();
                    let l73 = *arg0.add(148).cast::<i32>();
                    let l75 = *arg0.add(152).cast::<i32>();
                    let l77 = *arg0.add(156).cast::<i32>();
                    let l79 = *arg0.add(160).cast::<i32>();
                    let l81 = *arg0.add(164).cast::<i32>();
                    let l83 = *arg0.add(168).cast::<i32>();
                    let l85 = *arg0.add(172).cast::<i32>();
                    let l87 = *arg0.add(176).cast::<i32>();
                    let l89 = *arg0.add(180).cast::<i32>();
                    let l91 = *arg0.add(184).cast::<i32>();
                    let l93 = *arg0.add(188).cast::<i32>();
                    let l95 = *arg0.add(192).cast::<i32>();
                    let l97 = *arg0.add(196).cast::<i32>();
                    let l99 = *arg0.add(200).cast::<i32>();
                    let l101 = *arg0.add(204).cast::<i32>();
                    let l103 = *arg0.add(208).cast::<i32>();
                    let l105 = *arg0.add(212).cast::<i32>();
                    let l107 = *arg0.add(216).cast::<i32>();
                    let l109 = *arg0.add(220).cast::<i32>();
                    let l111 = *arg0.add(224).cast::<i32>();
                    let l113 = *arg0.add(228).cast::<i32>();
                    let l115 = *arg0.add(232).cast::<i32>();
                    let l117 = *arg0.add(236).cast::<i32>();
                    let l119 = *arg0.add(240).cast::<i32>();
                    let l121 = *arg0.add(244).cast::<i32>();
                    let l123 = *arg0.add(248).cast::<i32>();
                    let l125 = *arg0.add(252).cast::<i32>();
                    let result127 = T::invoke(
                        _rt::string_lift(bytes2),
                        Args {
                            attach: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l3 as u32,
                                );
                                &handle4
                            },
                            capabilities: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l5 as u32,
                                );
                                &handle6
                            },
                            cgroupns_mode: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l7 as u32,
                                );
                                &handle8
                            },
                            command: {
                                handle10 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l9 as u32,
                                );
                                &handle10
                            },
                            container_read_refresh_timeout_milliseconds: {
                                handle12 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l11 as u32,
                                );
                                &handle12
                            },
                            cpu_set: {
                                handle14 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l13 as u32,
                                );
                                &handle14
                            },
                            cpu_shares: {
                                handle16 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l15 as u32,
                                );
                                &handle16
                            },
                            destroy_grace_seconds: {
                                handle18 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l17 as u32,
                                );
                                &handle18
                            },
                            devices: {
                                handle20 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l19 as u32,
                                );
                                &handle20
                            },
                            dns: {
                                handle22 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l21 as u32,
                                );
                                &handle22
                            },
                            dns_opts: {
                                handle24 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l23 as u32,
                                );
                                &handle24
                            },
                            dns_searches: {
                                handle26 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l25 as u32,
                                );
                                &handle26
                            },
                            domainname: {
                                handle28 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l27 as u32,
                                );
                                &handle28
                            },
                            entrypoints: {
                                handle30 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l29 as u32,
                                );
                                &handle30
                            },
                            envs: {
                                handle32 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l31 as u32,
                                );
                                &handle32
                            },
                            gpus: {
                                handle34 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l33 as u32,
                                );
                                &handle34
                            },
                            group_adds: {
                                handle36 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l35 as u32,
                                );
                                &handle36
                            },
                            healthcheck: {
                                handle38 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l37 as u32,
                                );
                                &handle38
                            },
                            hostname: {
                                handle40 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l39 as u32,
                                );
                                &handle40
                            },
                            hosts: {
                                handle42 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l41 as u32,
                                );
                                &handle42
                            },
                            image: {
                                handle44 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l43 as u32,
                                );
                                &handle44
                            },
                            init: {
                                handle46 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l45 as u32,
                                );
                                &handle46
                            },
                            ipc_mode: {
                                handle48 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l47 as u32,
                                );
                                &handle48
                            },
                            labels: {
                                handle50 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l49 as u32,
                                );
                                &handle50
                            },
                            log_driver: {
                                handle52 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l51 as u32,
                                );
                                &handle52
                            },
                            log_opts: {
                                handle54 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l53 as u32,
                                );
                                &handle54
                            },
                            logs: {
                                handle56 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l55 as u32,
                                );
                                &handle56
                            },
                            max_retry_count: {
                                handle58 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l57 as u32,
                                );
                                &handle58
                            },
                            memory: {
                                handle60 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l59 as u32,
                                );
                                &handle60
                            },
                            memory_swap: {
                                handle62 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l61 as u32,
                                );
                                &handle62
                            },
                            mounts: {
                                handle64 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l63 as u32,
                                );
                                &handle64
                            },
                            must_run: {
                                handle66 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l65 as u32,
                                );
                                &handle66
                            },
                            name: {
                                handle68 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l67 as u32,
                                );
                                &handle68
                            },
                            network_mode: {
                                handle70 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l69 as u32,
                                );
                                &handle70
                            },
                            networks_advanced: {
                                handle72 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l71 as u32,
                                );
                                &handle72
                            },
                            pid_mode: {
                                handle74 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l73 as u32,
                                );
                                &handle74
                            },
                            ports: {
                                handle76 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l75 as u32,
                                );
                                &handle76
                            },
                            privileged: {
                                handle78 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l77 as u32,
                                );
                                &handle78
                            },
                            publish_all_ports: {
                                handle80 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l79 as u32,
                                );
                                &handle80
                            },
                            read_only: {
                                handle82 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l81 as u32,
                                );
                                &handle82
                            },
                            remove_volumes: {
                                handle84 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l83 as u32,
                                );
                                &handle84
                            },
                            restart: {
                                handle86 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l85 as u32,
                                );
                                &handle86
                            },
                            rm: {
                                handle88 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l87 as u32,
                                );
                                &handle88
                            },
                            runtime: {
                                handle90 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l89 as u32,
                                );
                                &handle90
                            },
                            security_opts: {
                                handle92 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l91 as u32,
                                );
                                &handle92
                            },
                            shm_size: {
                                handle94 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l93 as u32,
                                );
                                &handle94
                            },
                            start: {
                                handle96 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l95 as u32,
                                );
                                &handle96
                            },
                            stdin_open: {
                                handle98 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l97 as u32,
                                );
                                &handle98
                            },
                            stop_signal: {
                                handle100 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l99 as u32,
                                );
                                &handle100
                            },
                            stop_timeout: {
                                handle102 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l101 as u32,
                                );
                                &handle102
                            },
                            storage_opts: {
                                handle104 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l103 as u32,
                                );
                                &handle104
                            },
                            sysctls: {
                                handle106 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l105 as u32,
                                );
                                &handle106
                            },
                            tmpfs: {
                                handle108 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l107 as u32,
                                );
                                &handle108
                            },
                            tty: {
                                handle110 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l109 as u32,
                                );
                                &handle110
                            },
                            ulimits: {
                                handle112 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l111 as u32,
                                );
                                &handle112
                            },
                            uploads: {
                                handle114 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l113 as u32,
                                );
                                &handle114
                            },
                            user: {
                                handle116 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l115 as u32,
                                );
                                &handle116
                            },
                            userns_mode: {
                                handle118 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l117 as u32,
                                );
                                &handle118
                            },
                            volumes: {
                                handle120 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l119 as u32,
                                );
                                &handle120
                            },
                            wait: {
                                handle122 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l121 as u32,
                                );
                                &handle122
                            },
                            wait_timeout: {
                                handle124 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l123 as u32,
                                );
                                &handle124
                            },
                            working_dir: {
                                handle126 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    l125 as u32,
                                );
                                &handle126
                            },
                        },
                    );
                    _rt::cabi_dealloc(arg0, 256, 4);
                    let ptr128 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        attach: attach129,
                        bridge: bridge129,
                        capabilities: capabilities129,
                        cgroupns_mode: cgroupns_mode129,
                        command: command129,
                        container_logs: container_logs129,
                        container_read_refresh_timeout_milliseconds: container_read_refresh_timeout_milliseconds129,
                        cpu_set: cpu_set129,
                        cpu_shares: cpu_shares129,
                        destroy_grace_seconds: destroy_grace_seconds129,
                        devices: devices129,
                        dns: dns129,
                        dns_opts: dns_opts129,
                        dns_searches: dns_searches129,
                        domainname: domainname129,
                        entrypoints: entrypoints129,
                        envs: envs129,
                        exit_code: exit_code129,
                        gpus: gpus129,
                        group_adds: group_adds129,
                        healthcheck: healthcheck129,
                        hostname: hostname129,
                        hosts: hosts129,
                        image: image129,
                        init: init129,
                        ipc_mode: ipc_mode129,
                        labels: labels129,
                        log_driver: log_driver129,
                        log_opts: log_opts129,
                        logs: logs129,
                        max_retry_count: max_retry_count129,
                        memory: memory129,
                        memory_swap: memory_swap129,
                        mounts: mounts129,
                        must_run: must_run129,
                        name: name129,
                        network_datas: network_datas129,
                        network_mode: network_mode129,
                        networks_advanced: networks_advanced129,
                        pid_mode: pid_mode129,
                        ports: ports129,
                        privileged: privileged129,
                        publish_all_ports: publish_all_ports129,
                        read_only: read_only129,
                        remove_volumes: remove_volumes129,
                        restart: restart129,
                        rm: rm129,
                        runtime: runtime129,
                        security_opts: security_opts129,
                        shm_size: shm_size129,
                        start: start129,
                        stdin_open: stdin_open129,
                        stop_signal: stop_signal129,
                        stop_timeout: stop_timeout129,
                        storage_opts: storage_opts129,
                        sysctls: sysctls129,
                        tmpfs: tmpfs129,
                        tty: tty129,
                        ulimits: ulimits129,
                        uploads: uploads129,
                        user: user129,
                        userns_mode: userns_mode129,
                        volumes: volumes129,
                        wait: wait129,
                        wait_timeout: wait_timeout129,
                        working_dir: working_dir129,
                    } = result127;
                    *ptr128.add(0).cast::<i32>() = (attach129).take_handle() as i32;
                    *ptr128.add(4).cast::<i32>() = (bridge129).take_handle() as i32;
                    *ptr128.add(8).cast::<i32>() = (capabilities129).take_handle()
                        as i32;
                    *ptr128.add(12).cast::<i32>() = (cgroupns_mode129).take_handle()
                        as i32;
                    *ptr128.add(16).cast::<i32>() = (command129).take_handle() as i32;
                    *ptr128.add(20).cast::<i32>() = (container_logs129).take_handle()
                        as i32;
                    *ptr128.add(24).cast::<i32>() = (container_read_refresh_timeout_milliseconds129)
                        .take_handle() as i32;
                    *ptr128.add(28).cast::<i32>() = (cpu_set129).take_handle() as i32;
                    *ptr128.add(32).cast::<i32>() = (cpu_shares129).take_handle() as i32;
                    *ptr128.add(36).cast::<i32>() = (destroy_grace_seconds129)
                        .take_handle() as i32;
                    *ptr128.add(40).cast::<i32>() = (devices129).take_handle() as i32;
                    *ptr128.add(44).cast::<i32>() = (dns129).take_handle() as i32;
                    *ptr128.add(48).cast::<i32>() = (dns_opts129).take_handle() as i32;
                    *ptr128.add(52).cast::<i32>() = (dns_searches129).take_handle()
                        as i32;
                    *ptr128.add(56).cast::<i32>() = (domainname129).take_handle() as i32;
                    *ptr128.add(60).cast::<i32>() = (entrypoints129).take_handle()
                        as i32;
                    *ptr128.add(64).cast::<i32>() = (envs129).take_handle() as i32;
                    *ptr128.add(68).cast::<i32>() = (exit_code129).take_handle() as i32;
                    *ptr128.add(72).cast::<i32>() = (gpus129).take_handle() as i32;
                    *ptr128.add(76).cast::<i32>() = (group_adds129).take_handle() as i32;
                    *ptr128.add(80).cast::<i32>() = (healthcheck129).take_handle()
                        as i32;
                    *ptr128.add(84).cast::<i32>() = (hostname129).take_handle() as i32;
                    *ptr128.add(88).cast::<i32>() = (hosts129).take_handle() as i32;
                    *ptr128.add(92).cast::<i32>() = (image129).take_handle() as i32;
                    *ptr128.add(96).cast::<i32>() = (init129).take_handle() as i32;
                    *ptr128.add(100).cast::<i32>() = (ipc_mode129).take_handle() as i32;
                    *ptr128.add(104).cast::<i32>() = (labels129).take_handle() as i32;
                    *ptr128.add(108).cast::<i32>() = (log_driver129).take_handle()
                        as i32;
                    *ptr128.add(112).cast::<i32>() = (log_opts129).take_handle() as i32;
                    *ptr128.add(116).cast::<i32>() = (logs129).take_handle() as i32;
                    *ptr128.add(120).cast::<i32>() = (max_retry_count129).take_handle()
                        as i32;
                    *ptr128.add(124).cast::<i32>() = (memory129).take_handle() as i32;
                    *ptr128.add(128).cast::<i32>() = (memory_swap129).take_handle()
                        as i32;
                    *ptr128.add(132).cast::<i32>() = (mounts129).take_handle() as i32;
                    *ptr128.add(136).cast::<i32>() = (must_run129).take_handle() as i32;
                    *ptr128.add(140).cast::<i32>() = (name129).take_handle() as i32;
                    *ptr128.add(144).cast::<i32>() = (network_datas129).take_handle()
                        as i32;
                    *ptr128.add(148).cast::<i32>() = (network_mode129).take_handle()
                        as i32;
                    *ptr128.add(152).cast::<i32>() = (networks_advanced129).take_handle()
                        as i32;
                    *ptr128.add(156).cast::<i32>() = (pid_mode129).take_handle() as i32;
                    *ptr128.add(160).cast::<i32>() = (ports129).take_handle() as i32;
                    *ptr128.add(164).cast::<i32>() = (privileged129).take_handle()
                        as i32;
                    *ptr128.add(168).cast::<i32>() = (publish_all_ports129).take_handle()
                        as i32;
                    *ptr128.add(172).cast::<i32>() = (read_only129).take_handle() as i32;
                    *ptr128.add(176).cast::<i32>() = (remove_volumes129).take_handle()
                        as i32;
                    *ptr128.add(180).cast::<i32>() = (restart129).take_handle() as i32;
                    *ptr128.add(184).cast::<i32>() = (rm129).take_handle() as i32;
                    *ptr128.add(188).cast::<i32>() = (runtime129).take_handle() as i32;
                    *ptr128.add(192).cast::<i32>() = (security_opts129).take_handle()
                        as i32;
                    *ptr128.add(196).cast::<i32>() = (shm_size129).take_handle() as i32;
                    *ptr128.add(200).cast::<i32>() = (start129).take_handle() as i32;
                    *ptr128.add(204).cast::<i32>() = (stdin_open129).take_handle()
                        as i32;
                    *ptr128.add(208).cast::<i32>() = (stop_signal129).take_handle()
                        as i32;
                    *ptr128.add(212).cast::<i32>() = (stop_timeout129).take_handle()
                        as i32;
                    *ptr128.add(216).cast::<i32>() = (storage_opts129).take_handle()
                        as i32;
                    *ptr128.add(220).cast::<i32>() = (sysctls129).take_handle() as i32;
                    *ptr128.add(224).cast::<i32>() = (tmpfs129).take_handle() as i32;
                    *ptr128.add(228).cast::<i32>() = (tty129).take_handle() as i32;
                    *ptr128.add(232).cast::<i32>() = (ulimits129).take_handle() as i32;
                    *ptr128.add(236).cast::<i32>() = (uploads129).take_handle() as i32;
                    *ptr128.add(240).cast::<i32>() = (user129).take_handle() as i32;
                    *ptr128.add(244).cast::<i32>() = (userns_mode129).take_handle()
                        as i32;
                    *ptr128.add(248).cast::<i32>() = (volumes129).take_handle() as i32;
                    *ptr128.add(252).cast::<i32>() = (wait129).take_handle() as i32;
                    *ptr128.add(256).cast::<i32>() = (wait_timeout129).take_handle()
                        as i32;
                    *ptr128.add(260).cast::<i32>() = (working_dir129).take_handle()
                        as i32;
                    ptr128
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_container_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/container@4.5.3--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_container_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 264]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 264],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod image {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub build: &'a Output,
                    pub build_on_preview: &'a Output,
                    pub image_name: &'a Output,
                    pub registry: &'a Output,
                    pub skip_push: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("build", &self.build)
                            .field("build-on-preview", &self.build_on_preview)
                            .field("image-name", &self.image_name)
                            .field("registry", &self.registry)
                            .field("skip-push", &self.skip_push)
                            .finish()
                    }
                }
                pub struct Res {
                    pub base_image_name: Output,
                    pub context: Output,
                    pub dockerfile: Output,
                    pub image_name: Output,
                    pub platform: Output,
                    pub registry_server: Output,
                    pub repo_digest: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("base-image-name", &self.base_image_name)
                            .field("context", &self.context)
                            .field("dockerfile", &self.dockerfile)
                            .field("image-name", &self.image_name)
                            .field("platform", &self.platform)
                            .field("registry-server", &self.registry_server)
                            .field("repo-digest", &self.repo_digest)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result6 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            build: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            build_on_preview: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            image_name: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            registry: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            skip_push: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                        },
                    );
                    let ptr7 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        base_image_name: base_image_name8,
                        context: context8,
                        dockerfile: dockerfile8,
                        image_name: image_name8,
                        platform: platform8,
                        registry_server: registry_server8,
                        repo_digest: repo_digest8,
                    } = result6;
                    *ptr7.add(0).cast::<i32>() = (base_image_name8).take_handle() as i32;
                    *ptr7.add(4).cast::<i32>() = (context8).take_handle() as i32;
                    *ptr7.add(8).cast::<i32>() = (dockerfile8).take_handle() as i32;
                    *ptr7.add(12).cast::<i32>() = (image_name8).take_handle() as i32;
                    *ptr7.add(16).cast::<i32>() = (platform8).take_handle() as i32;
                    *ptr7.add(20).cast::<i32>() = (registry_server8).take_handle()
                        as i32;
                    *ptr7.add(24).cast::<i32>() = (repo_digest8).take_handle() as i32;
                    ptr7
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_image_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/image@4.5.3--0.0.0-DEV#invoke"] unsafe extern "C"
                        fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32, arg3
                        : i32, arg4 : i32, arg5 : i32, arg6 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5, arg6) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_image_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 28],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod network {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub attachable: &'a Output,
                    pub check_duplicate: &'a Output,
                    pub driver: &'a Output,
                    pub ingress: &'a Output,
                    pub internal: &'a Output,
                    pub ipam_configs: &'a Output,
                    pub ipam_driver: &'a Output,
                    pub ipam_options: &'a Output,
                    pub ipv6: &'a Output,
                    pub labels: &'a Output,
                    pub name: &'a Output,
                    pub options: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("attachable", &self.attachable)
                            .field("check-duplicate", &self.check_duplicate)
                            .field("driver", &self.driver)
                            .field("ingress", &self.ingress)
                            .field("internal", &self.internal)
                            .field("ipam-configs", &self.ipam_configs)
                            .field("ipam-driver", &self.ipam_driver)
                            .field("ipam-options", &self.ipam_options)
                            .field("ipv6", &self.ipv6)
                            .field("labels", &self.labels)
                            .field("name", &self.name)
                            .field("options", &self.options)
                            .finish()
                    }
                }
                pub struct Res {
                    pub attachable: Output,
                    pub check_duplicate: Output,
                    pub driver: Output,
                    pub ingress: Output,
                    pub internal: Output,
                    pub ipam_configs: Output,
                    pub ipam_driver: Output,
                    pub ipam_options: Output,
                    pub ipv6: Output,
                    pub labels: Output,
                    pub name: Output,
                    pub options: Output,
                    pub scope: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("attachable", &self.attachable)
                            .field("check-duplicate", &self.check_duplicate)
                            .field("driver", &self.driver)
                            .field("ingress", &self.ingress)
                            .field("internal", &self.internal)
                            .field("ipam-configs", &self.ipam_configs)
                            .field("ipam-driver", &self.ipam_driver)
                            .field("ipam-options", &self.ipam_options)
                            .field("ipv6", &self.ipv6)
                            .field("labels", &self.labels)
                            .field("name", &self.name)
                            .field("options", &self.options)
                            .field("scope", &self.scope)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                    arg10: i32,
                    arg11: i32,
                    arg12: i32,
                    arg13: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let handle6;
                    let handle7;
                    let handle8;
                    let handle9;
                    let handle10;
                    let handle11;
                    let handle12;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result13 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            attachable: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            check_duplicate: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            driver: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            ingress: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            internal: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            ipam_configs: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            ipam_driver: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                            ipam_options: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg9 as u32,
                                );
                                &handle8
                            },
                            ipv6: {
                                handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg10 as u32,
                                );
                                &handle9
                            },
                            labels: {
                                handle10 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg11 as u32,
                                );
                                &handle10
                            },
                            name: {
                                handle11 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg12 as u32,
                                );
                                &handle11
                            },
                            options: {
                                handle12 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg13 as u32,
                                );
                                &handle12
                            },
                        },
                    );
                    let ptr14 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        attachable: attachable15,
                        check_duplicate: check_duplicate15,
                        driver: driver15,
                        ingress: ingress15,
                        internal: internal15,
                        ipam_configs: ipam_configs15,
                        ipam_driver: ipam_driver15,
                        ipam_options: ipam_options15,
                        ipv6: ipv615,
                        labels: labels15,
                        name: name15,
                        options: options15,
                        scope: scope15,
                    } = result13;
                    *ptr14.add(0).cast::<i32>() = (attachable15).take_handle() as i32;
                    *ptr14.add(4).cast::<i32>() = (check_duplicate15).take_handle()
                        as i32;
                    *ptr14.add(8).cast::<i32>() = (driver15).take_handle() as i32;
                    *ptr14.add(12).cast::<i32>() = (ingress15).take_handle() as i32;
                    *ptr14.add(16).cast::<i32>() = (internal15).take_handle() as i32;
                    *ptr14.add(20).cast::<i32>() = (ipam_configs15).take_handle() as i32;
                    *ptr14.add(24).cast::<i32>() = (ipam_driver15).take_handle() as i32;
                    *ptr14.add(28).cast::<i32>() = (ipam_options15).take_handle() as i32;
                    *ptr14.add(32).cast::<i32>() = (ipv615).take_handle() as i32;
                    *ptr14.add(36).cast::<i32>() = (labels15).take_handle() as i32;
                    *ptr14.add(40).cast::<i32>() = (name15).take_handle() as i32;
                    *ptr14.add(44).cast::<i32>() = (options15).take_handle() as i32;
                    *ptr14.add(48).cast::<i32>() = (scope15).take_handle() as i32;
                    ptr14
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_network_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/network@4.5.3--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32,
                        arg3 : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32, arg8
                        : i32, arg9 : i32, arg10 : i32, arg11 : i32, arg12 : i32, arg13 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_network_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 52]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 52],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod plugin {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub alias: &'a Output,
                    pub enable_timeout: &'a Output,
                    pub enabled: &'a Output,
                    pub envs: &'a Output,
                    pub force_destroy: &'a Output,
                    pub force_disable: &'a Output,
                    pub grant_all_permissions: &'a Output,
                    pub grant_permissions: &'a Output,
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("alias", &self.alias)
                            .field("enable-timeout", &self.enable_timeout)
                            .field("enabled", &self.enabled)
                            .field("envs", &self.envs)
                            .field("force-destroy", &self.force_destroy)
                            .field("force-disable", &self.force_disable)
                            .field("grant-all-permissions", &self.grant_all_permissions)
                            .field("grant-permissions", &self.grant_permissions)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                pub struct Res {
                    pub alias: Output,
                    pub enable_timeout: Output,
                    pub enabled: Output,
                    pub envs: Output,
                    pub force_destroy: Output,
                    pub force_disable: Output,
                    pub grant_all_permissions: Output,
                    pub grant_permissions: Output,
                    pub name: Output,
                    pub plugin_reference: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("alias", &self.alias)
                            .field("enable-timeout", &self.enable_timeout)
                            .field("enabled", &self.enabled)
                            .field("envs", &self.envs)
                            .field("force-destroy", &self.force_destroy)
                            .field("force-disable", &self.force_disable)
                            .field("grant-all-permissions", &self.grant_all_permissions)
                            .field("grant-permissions", &self.grant_permissions)
                            .field("name", &self.name)
                            .field("plugin-reference", &self.plugin_reference)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                    arg10: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let handle6;
                    let handle7;
                    let handle8;
                    let handle9;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result10 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            alias: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            enable_timeout: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            enabled: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            envs: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            force_destroy: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            force_disable: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            grant_all_permissions: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                            grant_permissions: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg9 as u32,
                                );
                                &handle8
                            },
                            name: {
                                handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg10 as u32,
                                );
                                &handle9
                            },
                        },
                    );
                    let ptr11 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        alias: alias12,
                        enable_timeout: enable_timeout12,
                        enabled: enabled12,
                        envs: envs12,
                        force_destroy: force_destroy12,
                        force_disable: force_disable12,
                        grant_all_permissions: grant_all_permissions12,
                        grant_permissions: grant_permissions12,
                        name: name12,
                        plugin_reference: plugin_reference12,
                    } = result10;
                    *ptr11.add(0).cast::<i32>() = (alias12).take_handle() as i32;
                    *ptr11.add(4).cast::<i32>() = (enable_timeout12).take_handle()
                        as i32;
                    *ptr11.add(8).cast::<i32>() = (enabled12).take_handle() as i32;
                    *ptr11.add(12).cast::<i32>() = (envs12).take_handle() as i32;
                    *ptr11.add(16).cast::<i32>() = (force_destroy12).take_handle()
                        as i32;
                    *ptr11.add(20).cast::<i32>() = (force_disable12).take_handle()
                        as i32;
                    *ptr11.add(24).cast::<i32>() = (grant_all_permissions12)
                        .take_handle() as i32;
                    *ptr11.add(28).cast::<i32>() = (grant_permissions12).take_handle()
                        as i32;
                    *ptr11.add(32).cast::<i32>() = (name12).take_handle() as i32;
                    *ptr11.add(36).cast::<i32>() = (plugin_reference12).take_handle()
                        as i32;
                    ptr11
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_plugin_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/plugin@4.5.3--0.0.0-DEV#invoke"] unsafe extern "C"
                        fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32, arg3
                        : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32, arg8 :
                        i32, arg9 : i32, arg10 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_plugin_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 40]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 40],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod registry_image {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub insecure_skip_verify: &'a Output,
                    pub keep_remotely: &'a Output,
                    pub name: &'a Output,
                    pub triggers: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("insecure-skip-verify", &self.insecure_skip_verify)
                            .field("keep-remotely", &self.keep_remotely)
                            .field("name", &self.name)
                            .field("triggers", &self.triggers)
                            .finish()
                    }
                }
                pub struct Res {
                    pub insecure_skip_verify: Output,
                    pub keep_remotely: Output,
                    pub name: Output,
                    pub sha256_digest: Output,
                    pub triggers: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("insecure-skip-verify", &self.insecure_skip_verify)
                            .field("keep-remotely", &self.keep_remotely)
                            .field("name", &self.name)
                            .field("sha256-digest", &self.sha256_digest)
                            .field("triggers", &self.triggers)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result5 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            insecure_skip_verify: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            keep_remotely: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            name: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            triggers: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                        },
                    );
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        insecure_skip_verify: insecure_skip_verify7,
                        keep_remotely: keep_remotely7,
                        name: name7,
                        sha256_digest: sha256_digest7,
                        triggers: triggers7,
                    } = result5;
                    *ptr6.add(0).cast::<i32>() = (insecure_skip_verify7).take_handle()
                        as i32;
                    *ptr6.add(4).cast::<i32>() = (keep_remotely7).take_handle() as i32;
                    *ptr6.add(8).cast::<i32>() = (name7).take_handle() as i32;
                    *ptr6.add(12).cast::<i32>() = (sha256_digest7).take_handle() as i32;
                    *ptr6.add(16).cast::<i32>() = (triggers7).take_handle() as i32;
                    ptr6
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_registry_image_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/registry-image@4.5.3--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_registry_image_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod remote_image {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub build: &'a Output,
                    pub force_remove: &'a Output,
                    pub keep_locally: &'a Output,
                    pub name: &'a Output,
                    pub platform: &'a Output,
                    pub pull_triggers: &'a Output,
                    pub triggers: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("build", &self.build)
                            .field("force-remove", &self.force_remove)
                            .field("keep-locally", &self.keep_locally)
                            .field("name", &self.name)
                            .field("platform", &self.platform)
                            .field("pull-triggers", &self.pull_triggers)
                            .field("triggers", &self.triggers)
                            .finish()
                    }
                }
                pub struct Res {
                    pub build: Output,
                    pub force_remove: Output,
                    pub image_id: Output,
                    pub keep_locally: Output,
                    pub name: Output,
                    pub platform: Output,
                    pub pull_triggers: Output,
                    pub repo_digest: Output,
                    pub triggers: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("build", &self.build)
                            .field("force-remove", &self.force_remove)
                            .field("image-id", &self.image_id)
                            .field("keep-locally", &self.keep_locally)
                            .field("name", &self.name)
                            .field("platform", &self.platform)
                            .field("pull-triggers", &self.pull_triggers)
                            .field("repo-digest", &self.repo_digest)
                            .field("triggers", &self.triggers)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: i32,
                    arg8: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let handle6;
                    let handle7;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result8 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            build: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            force_remove: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            keep_locally: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            name: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            platform: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            pull_triggers: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            triggers: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                        },
                    );
                    let ptr9 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        build: build10,
                        force_remove: force_remove10,
                        image_id: image_id10,
                        keep_locally: keep_locally10,
                        name: name10,
                        platform: platform10,
                        pull_triggers: pull_triggers10,
                        repo_digest: repo_digest10,
                        triggers: triggers10,
                    } = result8;
                    *ptr9.add(0).cast::<i32>() = (build10).take_handle() as i32;
                    *ptr9.add(4).cast::<i32>() = (force_remove10).take_handle() as i32;
                    *ptr9.add(8).cast::<i32>() = (image_id10).take_handle() as i32;
                    *ptr9.add(12).cast::<i32>() = (keep_locally10).take_handle() as i32;
                    *ptr9.add(16).cast::<i32>() = (name10).take_handle() as i32;
                    *ptr9.add(20).cast::<i32>() = (platform10).take_handle() as i32;
                    *ptr9.add(24).cast::<i32>() = (pull_triggers10).take_handle() as i32;
                    *ptr9.add(28).cast::<i32>() = (repo_digest10).take_handle() as i32;
                    *ptr9.add(32).cast::<i32>() = (triggers10).take_handle() as i32;
                    ptr9
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_remote_image_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/remote-image@4.5.3--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32,
                        arg8 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_remote_image_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 36]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 36],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod secret {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub data: &'a Output,
                    pub labels: &'a Output,
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("data", &self.data)
                            .field("labels", &self.labels)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                pub struct Res {
                    pub data: Output,
                    pub labels: Output,
                    pub name: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("data", &self.data)
                            .field("labels", &self.labels)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result4 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            data: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            labels: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            name: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                        },
                    );
                    let ptr5 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res { data: data6, labels: labels6, name: name6 } = result4;
                    *ptr5.add(0).cast::<i32>() = (data6).take_handle() as i32;
                    *ptr5.add(4).cast::<i32>() = (labels6).take_handle() as i32;
                    *ptr5.add(8).cast::<i32>() = (name6).take_handle() as i32;
                    ptr5
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_secret_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/secret@4.5.3--0.0.0-DEV#invoke"] unsafe extern "C"
                        fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32, arg3
                        : i32, arg4 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_secret_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod service {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub auth: &'a Output,
                    pub converge_config: &'a Output,
                    pub endpoint_spec: &'a Output,
                    pub labels: &'a Output,
                    pub mode: &'a Output,
                    pub name: &'a Output,
                    pub rollback_config: &'a Output,
                    pub task_spec: &'a Output,
                    pub update_config: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("auth", &self.auth)
                            .field("converge-config", &self.converge_config)
                            .field("endpoint-spec", &self.endpoint_spec)
                            .field("labels", &self.labels)
                            .field("mode", &self.mode)
                            .field("name", &self.name)
                            .field("rollback-config", &self.rollback_config)
                            .field("task-spec", &self.task_spec)
                            .field("update-config", &self.update_config)
                            .finish()
                    }
                }
                pub struct Res {
                    pub auth: Output,
                    pub converge_config: Output,
                    pub endpoint_spec: Output,
                    pub labels: Output,
                    pub mode: Output,
                    pub name: Output,
                    pub rollback_config: Output,
                    pub task_spec: Output,
                    pub update_config: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("auth", &self.auth)
                            .field("converge-config", &self.converge_config)
                            .field("endpoint-spec", &self.endpoint_spec)
                            .field("labels", &self.labels)
                            .field("mode", &self.mode)
                            .field("name", &self.name)
                            .field("rollback-config", &self.rollback_config)
                            .field("task-spec", &self.task_spec)
                            .field("update-config", &self.update_config)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                    arg10: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let handle6;
                    let handle7;
                    let handle8;
                    let handle9;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result10 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            auth: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            converge_config: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            endpoint_spec: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            labels: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            mode: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            name: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            rollback_config: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                            task_spec: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg9 as u32,
                                );
                                &handle8
                            },
                            update_config: {
                                handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg10 as u32,
                                );
                                &handle9
                            },
                        },
                    );
                    let ptr11 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        auth: auth12,
                        converge_config: converge_config12,
                        endpoint_spec: endpoint_spec12,
                        labels: labels12,
                        mode: mode12,
                        name: name12,
                        rollback_config: rollback_config12,
                        task_spec: task_spec12,
                        update_config: update_config12,
                    } = result10;
                    *ptr11.add(0).cast::<i32>() = (auth12).take_handle() as i32;
                    *ptr11.add(4).cast::<i32>() = (converge_config12).take_handle()
                        as i32;
                    *ptr11.add(8).cast::<i32>() = (endpoint_spec12).take_handle() as i32;
                    *ptr11.add(12).cast::<i32>() = (labels12).take_handle() as i32;
                    *ptr11.add(16).cast::<i32>() = (mode12).take_handle() as i32;
                    *ptr11.add(20).cast::<i32>() = (name12).take_handle() as i32;
                    *ptr11.add(24).cast::<i32>() = (rollback_config12).take_handle()
                        as i32;
                    *ptr11.add(28).cast::<i32>() = (task_spec12).take_handle() as i32;
                    *ptr11.add(32).cast::<i32>() = (update_config12).take_handle()
                        as i32;
                    ptr11
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_service_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/service@4.5.3--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32,
                        arg3 : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32, arg8
                        : i32, arg9 : i32, arg10 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_service_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 36]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 36],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod service_config {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub data: &'a Output,
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("data", &self.data)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                pub struct Res {
                    pub data: Output,
                    pub name: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("data", &self.data)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result3 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            data: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            name: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                        },
                    );
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res { data: data5, name: name5 } = result3;
                    *ptr4.add(0).cast::<i32>() = (data5).take_handle() as i32;
                    *ptr4.add(4).cast::<i32>() = (name5).take_handle() as i32;
                    ptr4
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_service_config_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/service-config@4.5.3--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_service_config_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod tag {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub source_image: &'a Output,
                    pub target_image: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("source-image", &self.source_image)
                            .field("target-image", &self.target_image)
                            .finish()
                    }
                }
                pub struct Res {
                    pub source_image: Output,
                    pub source_image_id: Output,
                    pub target_image: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("source-image", &self.source_image)
                            .field("source-image-id", &self.source_image_id)
                            .field("target-image", &self.target_image)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result3 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            source_image: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            target_image: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                        },
                    );
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        source_image: source_image5,
                        source_image_id: source_image_id5,
                        target_image: target_image5,
                    } = result3;
                    *ptr4.add(0).cast::<i32>() = (source_image5).take_handle() as i32;
                    *ptr4.add(4).cast::<i32>() = (source_image_id5).take_handle() as i32;
                    *ptr4.add(8).cast::<i32>() = (target_image5).take_handle() as i32;
                    ptr4
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_tag_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/tag@4.5.3--0.0.0-DEV#invoke"] unsafe extern "C" fn
                        export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32, arg3 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_tag_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod volume {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub driver: &'a Output,
                    pub driver_opts: &'a Output,
                    pub labels: &'a Output,
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("driver", &self.driver)
                            .field("driver-opts", &self.driver_opts)
                            .field("labels", &self.labels)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                pub struct Res {
                    pub driver: Output,
                    pub driver_opts: Output,
                    pub labels: Output,
                    pub mountpoint: Output,
                    pub name: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("driver", &self.driver)
                            .field("driver-opts", &self.driver_opts)
                            .field("labels", &self.labels)
                            .field("mountpoint", &self.mountpoint)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result5 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            driver: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            driver_opts: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            labels: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            name: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                        },
                    );
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        driver: driver7,
                        driver_opts: driver_opts7,
                        labels: labels7,
                        mountpoint: mountpoint7,
                        name: name7,
                    } = result5;
                    *ptr6.add(0).cast::<i32>() = (driver7).take_handle() as i32;
                    *ptr6.add(4).cast::<i32>() = (driver_opts7).take_handle() as i32;
                    *ptr6.add(8).cast::<i32>() = (labels7).take_handle() as i32;
                    *ptr6.add(12).cast::<i32>() = (mountpoint7).take_handle() as i32;
                    *ptr6.add(16).cast::<i32>() = (name7).take_handle() as i32;
                    ptr6
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_volume_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/volume@4.5.3--0.0.0-DEV#invoke"] unsafe extern "C"
                        fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32, arg3
                        : i32, arg4 : i32, arg5 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_volume_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod get_logs {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub details: &'a Output,
                    pub discard_headers: &'a Output,
                    pub follow: &'a Output,
                    pub logs_list_string_enabled: &'a Output,
                    pub name: &'a Output,
                    pub show_stderr: &'a Output,
                    pub show_stdout: &'a Output,
                    pub since: &'a Output,
                    pub tail: &'a Output,
                    pub timestamps: &'a Output,
                    pub until: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("details", &self.details)
                            .field("discard-headers", &self.discard_headers)
                            .field("follow", &self.follow)
                            .field(
                                "logs-list-string-enabled",
                                &self.logs_list_string_enabled,
                            )
                            .field("name", &self.name)
                            .field("show-stderr", &self.show_stderr)
                            .field("show-stdout", &self.show_stdout)
                            .field("since", &self.since)
                            .field("tail", &self.tail)
                            .field("timestamps", &self.timestamps)
                            .field("until", &self.until)
                            .finish()
                    }
                }
                pub struct Res {
                    pub details: Output,
                    pub discard_headers: Output,
                    pub follow: Output,
                    pub id: Output,
                    pub logs_list_string_enabled: Output,
                    pub logs_list_strings: Output,
                    pub name: Output,
                    pub show_stderr: Output,
                    pub show_stdout: Output,
                    pub since: Output,
                    pub tail: Output,
                    pub timestamps: Output,
                    pub until: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("details", &self.details)
                            .field("discard-headers", &self.discard_headers)
                            .field("follow", &self.follow)
                            .field("id", &self.id)
                            .field(
                                "logs-list-string-enabled",
                                &self.logs_list_string_enabled,
                            )
                            .field("logs-list-strings", &self.logs_list_strings)
                            .field("name", &self.name)
                            .field("show-stderr", &self.show_stderr)
                            .field("show-stdout", &self.show_stdout)
                            .field("since", &self.since)
                            .field("tail", &self.tail)
                            .field("timestamps", &self.timestamps)
                            .field("until", &self.until)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                    arg10: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let handle1;
                    let handle2;
                    let handle3;
                    let handle4;
                    let handle5;
                    let handle6;
                    let handle7;
                    let handle8;
                    let handle9;
                    let handle10;
                    let result11 = T::invoke(Args {
                        details: {
                            handle0 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg0 as u32,
                            );
                            &handle0
                        },
                        discard_headers: {
                            handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg1 as u32,
                            );
                            &handle1
                        },
                        follow: {
                            handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg2 as u32,
                            );
                            &handle2
                        },
                        logs_list_string_enabled: {
                            handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg3 as u32,
                            );
                            &handle3
                        },
                        name: {
                            handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg4 as u32,
                            );
                            &handle4
                        },
                        show_stderr: {
                            handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg5 as u32,
                            );
                            &handle5
                        },
                        show_stdout: {
                            handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg6 as u32,
                            );
                            &handle6
                        },
                        since: {
                            handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg7 as u32,
                            );
                            &handle7
                        },
                        tail: {
                            handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg8 as u32,
                            );
                            &handle8
                        },
                        timestamps: {
                            handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg9 as u32,
                            );
                            &handle9
                        },
                        until: {
                            handle10 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg10 as u32,
                            );
                            &handle10
                        },
                    });
                    let ptr12 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        details: details13,
                        discard_headers: discard_headers13,
                        follow: follow13,
                        id: id13,
                        logs_list_string_enabled: logs_list_string_enabled13,
                        logs_list_strings: logs_list_strings13,
                        name: name13,
                        show_stderr: show_stderr13,
                        show_stdout: show_stdout13,
                        since: since13,
                        tail: tail13,
                        timestamps: timestamps13,
                        until: until13,
                    } = result11;
                    *ptr12.add(0).cast::<i32>() = (details13).take_handle() as i32;
                    *ptr12.add(4).cast::<i32>() = (discard_headers13).take_handle()
                        as i32;
                    *ptr12.add(8).cast::<i32>() = (follow13).take_handle() as i32;
                    *ptr12.add(12).cast::<i32>() = (id13).take_handle() as i32;
                    *ptr12.add(16).cast::<i32>() = (logs_list_string_enabled13)
                        .take_handle() as i32;
                    *ptr12.add(20).cast::<i32>() = (logs_list_strings13).take_handle()
                        as i32;
                    *ptr12.add(24).cast::<i32>() = (name13).take_handle() as i32;
                    *ptr12.add(28).cast::<i32>() = (show_stderr13).take_handle() as i32;
                    *ptr12.add(32).cast::<i32>() = (show_stdout13).take_handle() as i32;
                    *ptr12.add(36).cast::<i32>() = (since13).take_handle() as i32;
                    *ptr12.add(40).cast::<i32>() = (tail13).take_handle() as i32;
                    *ptr12.add(44).cast::<i32>() = (timestamps13).take_handle() as i32;
                    *ptr12.add(48).cast::<i32>() = (until13).take_handle() as i32;
                    ptr12
                }
                pub trait Guest {
                    fn invoke(args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_get_logs_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/get-logs@4.5.3--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : i32, arg1 : i32, arg2 : i32, arg3 :
                        i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32, arg8 : i32,
                        arg9 : i32, arg10 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_get_logs_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 52]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 52],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod get_network {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args").field("name", &self.name).finish()
                    }
                }
                pub struct Res {
                    pub driver: Output,
                    pub id: Output,
                    pub internal: Output,
                    pub ipam_configs: Output,
                    pub name: Output,
                    pub options: Output,
                    pub scope: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("driver", &self.driver)
                            .field("id", &self.id)
                            .field("internal", &self.internal)
                            .field("ipam-configs", &self.ipam_configs)
                            .field("name", &self.name)
                            .field("options", &self.options)
                            .field("scope", &self.scope)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let result1 = T::invoke(Args {
                        name: {
                            handle0 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg0 as u32,
                            );
                            &handle0
                        },
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        driver: driver3,
                        id: id3,
                        internal: internal3,
                        ipam_configs: ipam_configs3,
                        name: name3,
                        options: options3,
                        scope: scope3,
                    } = result1;
                    *ptr2.add(0).cast::<i32>() = (driver3).take_handle() as i32;
                    *ptr2.add(4).cast::<i32>() = (id3).take_handle() as i32;
                    *ptr2.add(8).cast::<i32>() = (internal3).take_handle() as i32;
                    *ptr2.add(12).cast::<i32>() = (ipam_configs3).take_handle() as i32;
                    *ptr2.add(16).cast::<i32>() = (name3).take_handle() as i32;
                    *ptr2.add(20).cast::<i32>() = (options3).take_handle() as i32;
                    *ptr2.add(24).cast::<i32>() = (scope3).take_handle() as i32;
                    ptr2
                }
                pub trait Guest {
                    fn invoke(args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_get_network_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/get-network@4.5.3--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_get_network_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 28],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod get_plugin {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub alias: &'a Output,
                    pub id: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("alias", &self.alias)
                            .field("id", &self.id)
                            .finish()
                    }
                }
                pub struct Res {
                    pub alias: Output,
                    pub enabled: Output,
                    pub envs: Output,
                    pub grant_all_permissions: Output,
                    pub id: Output,
                    pub name: Output,
                    pub plugin_reference: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("alias", &self.alias)
                            .field("enabled", &self.enabled)
                            .field("envs", &self.envs)
                            .field("grant-all-permissions", &self.grant_all_permissions)
                            .field("id", &self.id)
                            .field("name", &self.name)
                            .field("plugin-reference", &self.plugin_reference)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let handle1;
                    let result2 = T::invoke(Args {
                        alias: {
                            handle0 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg0 as u32,
                            );
                            &handle0
                        },
                        id: {
                            handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg1 as u32,
                            );
                            &handle1
                        },
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        alias: alias4,
                        enabled: enabled4,
                        envs: envs4,
                        grant_all_permissions: grant_all_permissions4,
                        id: id4,
                        name: name4,
                        plugin_reference: plugin_reference4,
                    } = result2;
                    *ptr3.add(0).cast::<i32>() = (alias4).take_handle() as i32;
                    *ptr3.add(4).cast::<i32>() = (enabled4).take_handle() as i32;
                    *ptr3.add(8).cast::<i32>() = (envs4).take_handle() as i32;
                    *ptr3.add(12).cast::<i32>() = (grant_all_permissions4).take_handle()
                        as i32;
                    *ptr3.add(16).cast::<i32>() = (id4).take_handle() as i32;
                    *ptr3.add(20).cast::<i32>() = (name4).take_handle() as i32;
                    *ptr3.add(24).cast::<i32>() = (plugin_reference4).take_handle()
                        as i32;
                    ptr3
                }
                pub trait Guest {
                    fn invoke(args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_get_plugin_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/get-plugin@4.5.3--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : i32, arg1 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_get_plugin_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 28],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod get_registry_image {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub insecure_skip_verify: &'a Output,
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("insecure-skip-verify", &self.insecure_skip_verify)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                pub struct Res {
                    pub id: Output,
                    pub insecure_skip_verify: Output,
                    pub name: Output,
                    pub sha256_digest: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("id", &self.id)
                            .field("insecure-skip-verify", &self.insecure_skip_verify)
                            .field("name", &self.name)
                            .field("sha256-digest", &self.sha256_digest)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let handle1;
                    let result2 = T::invoke(Args {
                        insecure_skip_verify: {
                            handle0 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg0 as u32,
                            );
                            &handle0
                        },
                        name: {
                            handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg1 as u32,
                            );
                            &handle1
                        },
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        id: id4,
                        insecure_skip_verify: insecure_skip_verify4,
                        name: name4,
                        sha256_digest: sha256_digest4,
                    } = result2;
                    *ptr3.add(0).cast::<i32>() = (id4).take_handle() as i32;
                    *ptr3.add(4).cast::<i32>() = (insecure_skip_verify4).take_handle()
                        as i32;
                    *ptr3.add(8).cast::<i32>() = (name4).take_handle() as i32;
                    *ptr3.add(12).cast::<i32>() = (sha256_digest4).take_handle() as i32;
                    ptr3
                }
                pub trait Guest {
                    fn invoke(args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_get_registry_image_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/get-registry-image@4.5.3--0.0.0-DEV#invoke"]
                        unsafe extern "C" fn export_invoke(arg0 : i32, arg1 : i32,) -> *
                        mut u8 { $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0,
                        arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_get_registry_image_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod get_remote_image {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub name: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args").field("name", &self.name).finish()
                    }
                }
                pub struct Res {
                    pub id: Output,
                    pub name: Output,
                    pub repo_digest: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("id", &self.id)
                            .field("name", &self.name)
                            .field("repo-digest", &self.repo_digest)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let result1 = T::invoke(Args {
                        name: {
                            handle0 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                arg0 as u32,
                            );
                            &handle0
                        },
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res { id: id3, name: name3, repo_digest: repo_digest3 } = result1;
                    *ptr2.add(0).cast::<i32>() = (id3).take_handle() as i32;
                    *ptr2.add(4).cast::<i32>() = (name3).take_handle() as i32;
                    *ptr2.add(8).cast::<i32>() = (repo_digest3).take_handle() as i32;
                    ptr2
                }
                pub trait Guest {
                    fn invoke(args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_docker_get_remote_image_4_5_3_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:docker/get-remote-image@4.5.3--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_docker_get_remote_image_4_5_3_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_docker_pulumi_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::pulumi::docker::container::__export_pulumi_docker_container_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::container);
        $($path_to_types_root)*::
        exports::pulumi::docker::image::__export_pulumi_docker_image_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::image);
        $($path_to_types_root)*::
        exports::pulumi::docker::network::__export_pulumi_docker_network_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::network);
        $($path_to_types_root)*::
        exports::pulumi::docker::plugin::__export_pulumi_docker_plugin_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::plugin);
        $($path_to_types_root)*::
        exports::pulumi::docker::registry_image::__export_pulumi_docker_registry_image_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::registry_image);
        $($path_to_types_root)*::
        exports::pulumi::docker::remote_image::__export_pulumi_docker_remote_image_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::remote_image);
        $($path_to_types_root)*::
        exports::pulumi::docker::secret::__export_pulumi_docker_secret_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::secret);
        $($path_to_types_root)*::
        exports::pulumi::docker::service::__export_pulumi_docker_service_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::service);
        $($path_to_types_root)*::
        exports::pulumi::docker::service_config::__export_pulumi_docker_service_config_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::service_config);
        $($path_to_types_root)*::
        exports::pulumi::docker::tag::__export_pulumi_docker_tag_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::tag);
        $($path_to_types_root)*::
        exports::pulumi::docker::volume::__export_pulumi_docker_volume_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::volume);
        $($path_to_types_root)*::
        exports::pulumi::docker::get_logs::__export_pulumi_docker_get_logs_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::get_logs);
        $($path_to_types_root)*::
        exports::pulumi::docker::get_network::__export_pulumi_docker_get_network_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::get_network);
        $($path_to_types_root)*::
        exports::pulumi::docker::get_plugin::__export_pulumi_docker_get_plugin_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::docker::get_plugin);
        $($path_to_types_root)*::
        exports::pulumi::docker::get_registry_image::__export_pulumi_docker_get_registry_image_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::pulumi::docker::get_registry_image); $($path_to_types_root)*::
        exports::pulumi::docker::get_remote_image::__export_pulumi_docker_get_remote_image_4_5_3_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::pulumi::docker::get_remote_image);
    };
}
#[doc(inline)]
pub(crate) use __export_docker_pulumi_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:pulumi:docker@4.5.3--0.0.0-DEV:docker-pulumi:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 6179] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9f/\x01A\x02\x01A%\x01\
B\x0a\x04\0\x06output\x03\x01\x01i\0\x01@\x01\x05values\0\x01\x04\0\x13[construc\
tor]output\x01\x02\x01h\0\x01@\x02\x04self\x03\x0dfunction-names\0\x01\x04\0\x12\
[method]output.map\x01\x04\x01p\x03\x01@\x01\x07outputs\x05\0\x01\x04\0\x07combi\
ne\x01\x06\x03\00component:pulumi-wasm/output-interface@0.0.0-DEV\x05\0\x02\x03\0\
\0\x06output\x01B\x1c\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x02\
\x04names\x05value\x02\x04\0\x0cobject-field\x03\0\x03\x01r\x01\x04names\x04\0\x0c\
result-field\x03\0\x05\x01i\x01\x01r\x02\x04names\x06output\x07\x04\0\x1eregiste\
r-resource-result-field\x03\0\x08\x01p\x04\x01p\x06\x01r\x04\x04types\x04names\x06\
object\x0a\x07results\x0b\x04\0\x19register-resource-request\x03\0\x0c\x01p\x09\x01\
r\x01\x06fields\x0e\x04\0\x18register-resource-result\x03\0\x0f\x01r\x02\x04name\
s\x06output\x07\x04\0\x1cresource-invoke-result-field\x03\0\x11\x01r\x03\x05toke\
ns\x06object\x0a\x07results\x0b\x04\0\x17resource-invoke-request\x03\0\x13\x01p\x12\
\x01r\x01\x06fields\x15\x04\0\x16resource-invoke-result\x03\0\x16\x01@\x01\x07re\
quest\x0d\0\x10\x04\0\x08register\x01\x18\x01@\x01\x07request\x14\0\x17\x04\0\x06\
invoke\x01\x19\x03\02component:pulumi-wasm/register-interface@0.0.0-DEV\x05\x02\x01\
B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r>\x06attach\x02\x0c\
capabilities\x02\x0dcgroupns-mode\x02\x07command\x02+container-read-refresh-time\
out-milliseconds\x02\x07cpu-set\x02\x0acpu-shares\x02\x15destroy-grace-seconds\x02\
\x07devices\x02\x03dns\x02\x08dns-opts\x02\x0cdns-searches\x02\x0adomainname\x02\
\x0bentrypoints\x02\x04envs\x02\x04gpus\x02\x0agroup-adds\x02\x0bhealthcheck\x02\
\x08hostname\x02\x05hosts\x02\x05image\x02\x04init\x02\x08ipc-mode\x02\x06labels\
\x02\x0alog-driver\x02\x08log-opts\x02\x04logs\x02\x0fmax-retry-count\x02\x06mem\
ory\x02\x0bmemory-swap\x02\x06mounts\x02\x08must-run\x02\x04name\x02\x0cnetwork-\
mode\x02\x11networks-advanced\x02\x08pid-mode\x02\x05ports\x02\x0aprivileged\x02\
\x11publish-all-ports\x02\x09read-only\x02\x0eremove-volumes\x02\x07restart\x02\x02\
rm\x02\x07runtime\x02\x0dsecurity-opts\x02\x08shm-size\x02\x05start\x02\x0astdin\
-open\x02\x0bstop-signal\x02\x0cstop-timeout\x02\x0cstorage-opts\x02\x07sysctls\x02\
\x05tmpfs\x02\x03tty\x02\x07ulimits\x02\x07uploads\x02\x04user\x02\x0buserns-mod\
e\x02\x07volumes\x02\x04wait\x02\x0cwait-timeout\x02\x0bworking-dir\x02\x04\0\x04\
args\x03\0\x03\x01i\x01\x01rB\x06attach\x05\x06bridge\x05\x0ccapabilities\x05\x0d\
cgroupns-mode\x05\x07command\x05\x0econtainer-logs\x05+container-read-refresh-ti\
meout-milliseconds\x05\x07cpu-set\x05\x0acpu-shares\x05\x15destroy-grace-seconds\
\x05\x07devices\x05\x03dns\x05\x08dns-opts\x05\x0cdns-searches\x05\x0adomainname\
\x05\x0bentrypoints\x05\x04envs\x05\x09exit-code\x05\x04gpus\x05\x0agroup-adds\x05\
\x0bhealthcheck\x05\x08hostname\x05\x05hosts\x05\x05image\x05\x04init\x05\x08ipc\
-mode\x05\x06labels\x05\x0alog-driver\x05\x08log-opts\x05\x04logs\x05\x0fmax-ret\
ry-count\x05\x06memory\x05\x0bmemory-swap\x05\x06mounts\x05\x08must-run\x05\x04n\
ame\x05\x0dnetwork-datas\x05\x0cnetwork-mode\x05\x11networks-advanced\x05\x08pid\
-mode\x05\x05ports\x05\x0aprivileged\x05\x11publish-all-ports\x05\x09read-only\x05\
\x0eremove-volumes\x05\x07restart\x05\x02rm\x05\x07runtime\x05\x0dsecurity-opts\x05\
\x08shm-size\x05\x05start\x05\x0astdin-open\x05\x0bstop-signal\x05\x0cstop-timeo\
ut\x05\x0cstorage-opts\x05\x07sysctls\x05\x05tmpfs\x05\x03tty\x05\x07ulimits\x05\
\x07uploads\x05\x04user\x05\x0buserns-mode\x05\x07volumes\x05\x04wait\x05\x0cwai\
t-timeout\x05\x0bworking-dir\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04arg\
s\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0(pulumi:docker/container@4.5.3--0.0.0-D\
EV\x05\x03\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x05\
\x05build\x02\x10build-on-preview\x02\x0aimage-name\x02\x08registry\x02\x09skip-\
push\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x07\x0fbase-image-name\x05\x07con\
text\x05\x0adockerfile\x05\x0aimage-name\x05\x08platform\x05\x0fregistry-server\x05\
\x0brepo-digest\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\
\0\x06invoke\x01\x08\x04\0$pulumi:docker/image@4.5.3--0.0.0-DEV\x05\x04\x01B\x0a\
\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x0c\x0aattachable\x02\
\x0fcheck-duplicate\x02\x06driver\x02\x07ingress\x02\x08internal\x02\x0cipam-con\
figs\x02\x0bipam-driver\x02\x0cipam-options\x02\x04ipv6\x02\x06labels\x02\x04nam\
e\x02\x07options\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x0d\x0aattachable\x05\
\x0fcheck-duplicate\x05\x06driver\x05\x07ingress\x05\x08internal\x05\x0cipam-con\
figs\x05\x0bipam-driver\x05\x0cipam-options\x05\x04ipv6\x05\x06labels\x05\x04nam\
e\x05\x07options\x05\x05scope\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04ar\
gs\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0&pulumi:docker/network@4.5.3--0.0.0-DE\
V\x05\x05\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x09\
\x05alias\x02\x0eenable-timeout\x02\x07enabled\x02\x04envs\x02\x0dforce-destroy\x02\
\x0dforce-disable\x02\x15grant-all-permissions\x02\x11grant-permissions\x02\x04n\
ame\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x0a\x05alias\x05\x0eenable-timeout\
\x05\x07enabled\x05\x04envs\x05\x0dforce-destroy\x05\x0dforce-disable\x05\x15gra\
nt-all-permissions\x05\x11grant-permissions\x05\x04name\x05\x10plugin-reference\x05\
\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\
\x04\0%pulumi:docker/plugin@4.5.3--0.0.0-DEV\x05\x06\x01B\x0a\x02\x03\x02\x01\x01\
\x04\0\x06output\x03\0\0\x01h\x01\x01r\x04\x14insecure-skip-verify\x02\x0dkeep-r\
emotely\x02\x04name\x02\x08triggers\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x05\
\x14insecure-skip-verify\x05\x0dkeep-remotely\x05\x04name\x05\x0dsha256-digest\x05\
\x08triggers\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\
\x06invoke\x01\x08\x04\0-pulumi:docker/registry-image@4.5.3--0.0.0-DEV\x05\x07\x01\
B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x07\x05build\x02\
\x0cforce-remove\x02\x0ckeep-locally\x02\x04name\x02\x08platform\x02\x0dpull-tri\
ggers\x02\x08triggers\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x09\x05build\x05\
\x0cforce-remove\x05\x08image-id\x05\x0ckeep-locally\x05\x04name\x05\x08platform\
\x05\x0dpull-triggers\x05\x0brepo-digest\x05\x08triggers\x05\x04\0\x03res\x03\0\x06\
\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0+pulumi:docker\
/remote-image@4.5.3--0.0.0-DEV\x05\x08\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06out\
put\x03\0\0\x01h\x01\x01r\x03\x04data\x02\x06labels\x02\x04name\x02\x04\0\x04arg\
s\x03\0\x03\x01i\x01\x01r\x03\x04data\x05\x06labels\x05\x04name\x05\x04\0\x03res\
\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0%pul\
umi:docker/secret@4.5.3--0.0.0-DEV\x05\x09\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06\
output\x03\0\0\x01h\x01\x01r\x09\x04auth\x02\x0fconverge-config\x02\x0dendpoint-\
spec\x02\x06labels\x02\x04mode\x02\x04name\x02\x0frollback-config\x02\x09task-sp\
ec\x02\x0dupdate-config\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x09\x04auth\x05\
\x0fconverge-config\x05\x0dendpoint-spec\x05\x06labels\x05\x04mode\x05\x04name\x05\
\x0frollback-config\x05\x09task-spec\x05\x0dupdate-config\x05\x04\0\x03res\x03\0\
\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0&pulumi:do\
cker/service@4.5.3--0.0.0-DEV\x05\x0a\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06outp\
ut\x03\0\0\x01h\x01\x01r\x02\x04data\x02\x04name\x02\x04\0\x04args\x03\0\x03\x01\
i\x01\x01r\x02\x04data\x05\x04name\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04\
args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0-pulumi:docker/service-config@4.5.3-\
-0.0.0-DEV\x05\x0b\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\
\x01r\x02\x0csource-image\x02\x0ctarget-image\x02\x04\0\x04args\x03\0\x03\x01i\x01\
\x01r\x03\x0csource-image\x05\x0fsource-image-id\x05\x0ctarget-image\x05\x04\0\x03\
res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0\"\
pulumi:docker/tag@4.5.3--0.0.0-DEV\x05\x0c\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06\
output\x03\0\0\x01h\x01\x01r\x04\x06driver\x02\x0bdriver-opts\x02\x06labels\x02\x04\
name\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x05\x06driver\x05\x0bdriver-opts\x05\
\x06labels\x05\x0amountpoint\x05\x04name\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04\
names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0%pulumi:docker/volume@4.5.3\
--0.0.0-DEV\x05\x0d\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\
\x01r\x0b\x07details\x02\x0fdiscard-headers\x02\x06follow\x02\x18logs-list-strin\
g-enabled\x02\x04name\x02\x0bshow-stderr\x02\x0bshow-stdout\x02\x05since\x02\x04\
tail\x02\x0atimestamps\x02\x05until\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x0d\
\x07details\x05\x0fdiscard-headers\x05\x06follow\x05\x02id\x05\x18logs-list-stri\
ng-enabled\x05\x11logs-list-strings\x05\x04name\x05\x0bshow-stderr\x05\x0bshow-s\
tdout\x05\x05since\x05\x04tail\x05\x0atimestamps\x05\x05until\x05\x04\0\x03res\x03\
\0\x06\x01@\x01\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0'pulumi:docker/ge\
t-logs@4.5.3--0.0.0-DEV\x05\x0e\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\
\0\0\x01h\x01\x01r\x01\x04name\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x07\x06\
driver\x05\x02id\x05\x08internal\x05\x0cipam-configs\x05\x04name\x05\x07options\x05\
\x05scope\x05\x04\0\x03res\x03\0\x06\x01@\x01\x04args\x04\0\x07\x04\0\x06invoke\x01\
\x08\x04\0*pulumi:docker/get-network@4.5.3--0.0.0-DEV\x05\x0f\x01B\x0a\x02\x03\x02\
\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x02\x05alias\x02\x02id\x02\x04\0\x04\
args\x03\0\x03\x01i\x01\x01r\x07\x05alias\x05\x07enabled\x05\x04envs\x05\x15gran\
t-all-permissions\x05\x02id\x05\x04name\x05\x10plugin-reference\x05\x04\0\x03res\
\x03\0\x06\x01@\x01\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0)pulumi:docke\
r/get-plugin@4.5.3--0.0.0-DEV\x05\x10\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06outp\
ut\x03\0\0\x01h\x01\x01r\x02\x14insecure-skip-verify\x02\x04name\x02\x04\0\x04ar\
gs\x03\0\x03\x01i\x01\x01r\x04\x02id\x05\x14insecure-skip-verify\x05\x04name\x05\
\x0dsha256-digest\x05\x04\0\x03res\x03\0\x06\x01@\x01\x04args\x04\0\x07\x04\0\x06\
invoke\x01\x08\x04\01pulumi:docker/get-registry-image@4.5.3--0.0.0-DEV\x05\x11\x01\
B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x01\x04name\x02\x04\
\0\x04args\x03\0\x03\x01i\x01\x01r\x03\x02id\x05\x04name\x05\x0brepo-digest\x05\x04\
\0\x03res\x03\0\x06\x01@\x01\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0/pul\
umi:docker/get-remote-image@4.5.3--0.0.0-DEV\x05\x12\x04\0,pulumi:docker/docker-\
pulumi@4.5.3--0.0.0-DEV\x04\0\x0b\x13\x01\0\x0ddocker-pulumi\x03\0\0\0G\x09produ\
cers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x06\
0.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
