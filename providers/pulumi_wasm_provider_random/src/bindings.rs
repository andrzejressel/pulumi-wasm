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
        pub mod random {
            #[allow(dead_code, clippy::all)]
            pub mod random_bytes {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                    pub length: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .finish()
                    }
                }
                pub struct Res {
                    pub base64: Output,
                    pub hex: Output,
                    pub keepers: Output,
                    pub length: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("base64", &self.base64)
                            .field("hex", &self.hex)
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
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
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            length: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                        },
                    );
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        base64: base645,
                        hex: hex5,
                        keepers: keepers5,
                        length: length5,
                    } = result3;
                    *ptr4.add(0).cast::<i32>() = (base645).take_handle() as i32;
                    *ptr4.add(4).cast::<i32>() = (hex5).take_handle() as i32;
                    *ptr4.add(8).cast::<i32>() = (keepers5).take_handle() as i32;
                    *ptr4.add(12).cast::<i32>() = (length5).take_handle() as i32;
                    ptr4
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_bytes_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-bytes@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_bytes_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_id {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub byte_length: &'a Output,
                    pub keepers: &'a Output,
                    pub prefix: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("byte-length", &self.byte_length)
                            .field("keepers", &self.keepers)
                            .field("prefix", &self.prefix)
                            .finish()
                    }
                }
                pub struct Res {
                    pub b64_std: Output,
                    pub b64_url: Output,
                    pub byte_length: Output,
                    pub dec: Output,
                    pub hex: Output,
                    pub keepers: Output,
                    pub prefix: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("b64-std", &self.b64_std)
                            .field("b64-url", &self.b64_url)
                            .field("byte-length", &self.byte_length)
                            .field("dec", &self.dec)
                            .field("hex", &self.hex)
                            .field("keepers", &self.keepers)
                            .field("prefix", &self.prefix)
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
                            byte_length: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            keepers: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            prefix: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                        },
                    );
                    let ptr5 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        b64_std: b64_std6,
                        b64_url: b64_url6,
                        byte_length: byte_length6,
                        dec: dec6,
                        hex: hex6,
                        keepers: keepers6,
                        prefix: prefix6,
                    } = result4;
                    *ptr5.add(0).cast::<i32>() = (b64_std6).take_handle() as i32;
                    *ptr5.add(4).cast::<i32>() = (b64_url6).take_handle() as i32;
                    *ptr5.add(8).cast::<i32>() = (byte_length6).take_handle() as i32;
                    *ptr5.add(12).cast::<i32>() = (dec6).take_handle() as i32;
                    *ptr5.add(16).cast::<i32>() = (hex6).take_handle() as i32;
                    *ptr5.add(20).cast::<i32>() = (keepers6).take_handle() as i32;
                    *ptr5.add(24).cast::<i32>() = (prefix6).take_handle() as i32;
                    ptr5
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_id_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-id@4.15.0--0.0.0-DEV#invoke"] unsafe extern
                        "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 : i32,
                        arg3 : i32, arg4 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_id_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 28],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_integer {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                    pub max: &'a Output,
                    pub min: &'a Output,
                    pub seed: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("keepers", &self.keepers)
                            .field("max", &self.max)
                            .field("min", &self.min)
                            .field("seed", &self.seed)
                            .finish()
                    }
                }
                pub struct Res {
                    pub keepers: Output,
                    pub max: Output,
                    pub min: Output,
                    pub result: Output,
                    pub seed: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("keepers", &self.keepers)
                            .field("max", &self.max)
                            .field("min", &self.min)
                            .field("result", &self.result)
                            .field("seed", &self.seed)
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
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            max: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            min: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            seed: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                        },
                    );
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        keepers: keepers7,
                        max: max7,
                        min: min7,
                        result: result7,
                        seed: seed7,
                    } = result5;
                    *ptr6.add(0).cast::<i32>() = (keepers7).take_handle() as i32;
                    *ptr6.add(4).cast::<i32>() = (max7).take_handle() as i32;
                    *ptr6.add(8).cast::<i32>() = (min7).take_handle() as i32;
                    *ptr6.add(12).cast::<i32>() = (result7).take_handle() as i32;
                    *ptr6.add(16).cast::<i32>() = (seed7).take_handle() as i32;
                    ptr6
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_integer_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-integer@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_integer_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_password {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                    pub length: &'a Output,
                    pub lower: &'a Output,
                    pub min_lower: &'a Output,
                    pub min_numeric: &'a Output,
                    pub min_special: &'a Output,
                    pub min_upper: &'a Output,
                    pub number: &'a Output,
                    pub numeric: &'a Output,
                    pub override_special: &'a Output,
                    pub special: &'a Output,
                    pub upper: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("lower", &self.lower)
                            .field("min-lower", &self.min_lower)
                            .field("min-numeric", &self.min_numeric)
                            .field("min-special", &self.min_special)
                            .field("min-upper", &self.min_upper)
                            .field("number", &self.number)
                            .field("numeric", &self.numeric)
                            .field("override-special", &self.override_special)
                            .field("special", &self.special)
                            .field("upper", &self.upper)
                            .finish()
                    }
                }
                pub struct Res {
                    pub bcrypt_hash: Output,
                    pub keepers: Output,
                    pub length: Output,
                    pub lower: Output,
                    pub min_lower: Output,
                    pub min_numeric: Output,
                    pub min_special: Output,
                    pub min_upper: Output,
                    pub number: Output,
                    pub numeric: Output,
                    pub override_special: Output,
                    pub result: Output,
                    pub special: Output,
                    pub upper: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("bcrypt-hash", &self.bcrypt_hash)
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("lower", &self.lower)
                            .field("min-lower", &self.min_lower)
                            .field("min-numeric", &self.min_numeric)
                            .field("min-special", &self.min_special)
                            .field("min-upper", &self.min_upper)
                            .field("number", &self.number)
                            .field("numeric", &self.numeric)
                            .field("override-special", &self.override_special)
                            .field("result", &self.result)
                            .field("special", &self.special)
                            .field("upper", &self.upper)
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
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            length: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            lower: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            min_lower: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            min_numeric: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            min_special: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            min_upper: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                            number: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg9 as u32,
                                );
                                &handle8
                            },
                            numeric: {
                                handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg10 as u32,
                                );
                                &handle9
                            },
                            override_special: {
                                handle10 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg11 as u32,
                                );
                                &handle10
                            },
                            special: {
                                handle11 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg12 as u32,
                                );
                                &handle11
                            },
                            upper: {
                                handle12 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg13 as u32,
                                );
                                &handle12
                            },
                        },
                    );
                    let ptr14 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        bcrypt_hash: bcrypt_hash15,
                        keepers: keepers15,
                        length: length15,
                        lower: lower15,
                        min_lower: min_lower15,
                        min_numeric: min_numeric15,
                        min_special: min_special15,
                        min_upper: min_upper15,
                        number: number15,
                        numeric: numeric15,
                        override_special: override_special15,
                        result: result15,
                        special: special15,
                        upper: upper15,
                    } = result13;
                    *ptr14.add(0).cast::<i32>() = (bcrypt_hash15).take_handle() as i32;
                    *ptr14.add(4).cast::<i32>() = (keepers15).take_handle() as i32;
                    *ptr14.add(8).cast::<i32>() = (length15).take_handle() as i32;
                    *ptr14.add(12).cast::<i32>() = (lower15).take_handle() as i32;
                    *ptr14.add(16).cast::<i32>() = (min_lower15).take_handle() as i32;
                    *ptr14.add(20).cast::<i32>() = (min_numeric15).take_handle() as i32;
                    *ptr14.add(24).cast::<i32>() = (min_special15).take_handle() as i32;
                    *ptr14.add(28).cast::<i32>() = (min_upper15).take_handle() as i32;
                    *ptr14.add(32).cast::<i32>() = (number15).take_handle() as i32;
                    *ptr14.add(36).cast::<i32>() = (numeric15).take_handle() as i32;
                    *ptr14.add(40).cast::<i32>() = (override_special15).take_handle()
                        as i32;
                    *ptr14.add(44).cast::<i32>() = (result15).take_handle() as i32;
                    *ptr14.add(48).cast::<i32>() = (special15).take_handle() as i32;
                    *ptr14.add(52).cast::<i32>() = (upper15).take_handle() as i32;
                    ptr14
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_password_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-password@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32,
                        arg8 : i32, arg9 : i32, arg10 : i32, arg11 : i32, arg12 : i32,
                        arg13 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_password_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 56]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 56],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_pet {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                    pub length: &'a Output,
                    pub prefix: &'a Output,
                    pub separator: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("prefix", &self.prefix)
                            .field("separator", &self.separator)
                            .finish()
                    }
                }
                pub struct Res {
                    pub keepers: Output,
                    pub length: Output,
                    pub prefix: Output,
                    pub separator: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("prefix", &self.prefix)
                            .field("separator", &self.separator)
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
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            length: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            prefix: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            separator: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                        },
                    );
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        keepers: keepers7,
                        length: length7,
                        prefix: prefix7,
                        separator: separator7,
                    } = result5;
                    *ptr6.add(0).cast::<i32>() = (keepers7).take_handle() as i32;
                    *ptr6.add(4).cast::<i32>() = (length7).take_handle() as i32;
                    *ptr6.add(8).cast::<i32>() = (prefix7).take_handle() as i32;
                    *ptr6.add(12).cast::<i32>() = (separator7).take_handle() as i32;
                    ptr6
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_pet_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-pet@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_pet_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_shuffle {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub inputs: &'a Output,
                    pub keepers: &'a Output,
                    pub result_count: &'a Output,
                    pub seed: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("inputs", &self.inputs)
                            .field("keepers", &self.keepers)
                            .field("result-count", &self.result_count)
                            .field("seed", &self.seed)
                            .finish()
                    }
                }
                pub struct Res {
                    pub inputs: Output,
                    pub keepers: Output,
                    pub result_count: Output,
                    pub results: Output,
                    pub seed: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("inputs", &self.inputs)
                            .field("keepers", &self.keepers)
                            .field("result-count", &self.result_count)
                            .field("results", &self.results)
                            .field("seed", &self.seed)
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
                            inputs: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            keepers: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            result_count: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            seed: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                        },
                    );
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        inputs: inputs7,
                        keepers: keepers7,
                        result_count: result_count7,
                        results: results7,
                        seed: seed7,
                    } = result5;
                    *ptr6.add(0).cast::<i32>() = (inputs7).take_handle() as i32;
                    *ptr6.add(4).cast::<i32>() = (keepers7).take_handle() as i32;
                    *ptr6.add(8).cast::<i32>() = (result_count7).take_handle() as i32;
                    *ptr6.add(12).cast::<i32>() = (results7).take_handle() as i32;
                    *ptr6.add(16).cast::<i32>() = (seed7).take_handle() as i32;
                    ptr6
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_shuffle_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-shuffle@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_invoke_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_shuffle_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_string {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                    pub length: &'a Output,
                    pub lower: &'a Output,
                    pub min_lower: &'a Output,
                    pub min_numeric: &'a Output,
                    pub min_special: &'a Output,
                    pub min_upper: &'a Output,
                    pub number: &'a Output,
                    pub numeric: &'a Output,
                    pub override_special: &'a Output,
                    pub special: &'a Output,
                    pub upper: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("lower", &self.lower)
                            .field("min-lower", &self.min_lower)
                            .field("min-numeric", &self.min_numeric)
                            .field("min-special", &self.min_special)
                            .field("min-upper", &self.min_upper)
                            .field("number", &self.number)
                            .field("numeric", &self.numeric)
                            .field("override-special", &self.override_special)
                            .field("special", &self.special)
                            .field("upper", &self.upper)
                            .finish()
                    }
                }
                pub struct Res {
                    pub keepers: Output,
                    pub length: Output,
                    pub lower: Output,
                    pub min_lower: Output,
                    pub min_numeric: Output,
                    pub min_special: Output,
                    pub min_upper: Output,
                    pub number: Output,
                    pub numeric: Output,
                    pub override_special: Output,
                    pub result: Output,
                    pub special: Output,
                    pub upper: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("keepers", &self.keepers)
                            .field("length", &self.length)
                            .field("lower", &self.lower)
                            .field("min-lower", &self.min_lower)
                            .field("min-numeric", &self.min_numeric)
                            .field("min-special", &self.min_special)
                            .field("min-upper", &self.min_upper)
                            .field("number", &self.number)
                            .field("numeric", &self.numeric)
                            .field("override-special", &self.override_special)
                            .field("result", &self.result)
                            .field("special", &self.special)
                            .field("upper", &self.upper)
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
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                            length: {
                                handle2 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg3 as u32,
                                );
                                &handle2
                            },
                            lower: {
                                handle3 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg4 as u32,
                                );
                                &handle3
                            },
                            min_lower: {
                                handle4 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg5 as u32,
                                );
                                &handle4
                            },
                            min_numeric: {
                                handle5 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg6 as u32,
                                );
                                &handle5
                            },
                            min_special: {
                                handle6 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg7 as u32,
                                );
                                &handle6
                            },
                            min_upper: {
                                handle7 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg8 as u32,
                                );
                                &handle7
                            },
                            number: {
                                handle8 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg9 as u32,
                                );
                                &handle8
                            },
                            numeric: {
                                handle9 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg10 as u32,
                                );
                                &handle9
                            },
                            override_special: {
                                handle10 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg11 as u32,
                                );
                                &handle10
                            },
                            special: {
                                handle11 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg12 as u32,
                                );
                                &handle11
                            },
                            upper: {
                                handle12 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg13 as u32,
                                );
                                &handle12
                            },
                        },
                    );
                    let ptr14 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res {
                        keepers: keepers15,
                        length: length15,
                        lower: lower15,
                        min_lower: min_lower15,
                        min_numeric: min_numeric15,
                        min_special: min_special15,
                        min_upper: min_upper15,
                        number: number15,
                        numeric: numeric15,
                        override_special: override_special15,
                        result: result15,
                        special: special15,
                        upper: upper15,
                    } = result13;
                    *ptr14.add(0).cast::<i32>() = (keepers15).take_handle() as i32;
                    *ptr14.add(4).cast::<i32>() = (length15).take_handle() as i32;
                    *ptr14.add(8).cast::<i32>() = (lower15).take_handle() as i32;
                    *ptr14.add(12).cast::<i32>() = (min_lower15).take_handle() as i32;
                    *ptr14.add(16).cast::<i32>() = (min_numeric15).take_handle() as i32;
                    *ptr14.add(20).cast::<i32>() = (min_special15).take_handle() as i32;
                    *ptr14.add(24).cast::<i32>() = (min_upper15).take_handle() as i32;
                    *ptr14.add(28).cast::<i32>() = (number15).take_handle() as i32;
                    *ptr14.add(32).cast::<i32>() = (numeric15).take_handle() as i32;
                    *ptr14.add(36).cast::<i32>() = (override_special15).take_handle()
                        as i32;
                    *ptr14.add(40).cast::<i32>() = (result15).take_handle() as i32;
                    *ptr14.add(44).cast::<i32>() = (special15).take_handle() as i32;
                    *ptr14.add(48).cast::<i32>() = (upper15).take_handle() as i32;
                    ptr14
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_string_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-string@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32, arg3 : i32, arg4 : i32, arg5 : i32, arg6 : i32, arg7 : i32,
                        arg8 : i32, arg9 : i32, arg10 : i32, arg11 : i32, arg12 : i32,
                        arg13 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_string_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 52]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 52],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod random_uuid {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::component::pulumi_wasm::output_interface::Output;
                pub struct Args<'a> {
                    pub keepers: &'a Output,
                }
                impl<'a> ::core::fmt::Debug for Args<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Args").field("keepers", &self.keepers).finish()
                    }
                }
                pub struct Res {
                    pub keepers: Output,
                    pub result: Output,
                }
                impl ::core::fmt::Debug for Res {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Res")
                            .field("keepers", &self.keepers)
                            .field("result", &self.result)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle1;
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result2 = T::invoke(
                        _rt::string_lift(bytes0),
                        Args {
                            keepers: {
                                handle1 = super::super::super::super::component::pulumi_wasm::output_interface::Output::from_handle(
                                    arg2 as u32,
                                );
                                &handle1
                            },
                        },
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Res { keepers: keepers4, result: result4 } = result2;
                    *ptr3.add(0).cast::<i32>() = (keepers4).take_handle() as i32;
                    *ptr3.add(4).cast::<i32>() = (result4).take_handle() as i32;
                    ptr3
                }
                pub trait Guest {
                    fn invoke(name: _rt::String, args: Args<'_>) -> Res;
                }
                #[doc(hidden)]
                macro_rules! __export_pulumi_random_random_uuid_4_15_0_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "pulumi:random/random-uuid@4.15.0--0.0.0-DEV#invoke"] unsafe
                        extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize, arg2 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_invoke_cabi::<$ty > (arg0, arg1, arg2) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_pulumi_random_random_uuid_4_15_0_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
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
macro_rules! __export_random_pulumi_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::pulumi::random::random_bytes::__export_pulumi_random_random_bytes_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_bytes);
        $($path_to_types_root)*::
        exports::pulumi::random::random_id::__export_pulumi_random_random_id_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_id);
        $($path_to_types_root)*::
        exports::pulumi::random::random_integer::__export_pulumi_random_random_integer_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_integer);
        $($path_to_types_root)*::
        exports::pulumi::random::random_password::__export_pulumi_random_random_password_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::pulumi::random::random_password); $($path_to_types_root)*::
        exports::pulumi::random::random_pet::__export_pulumi_random_random_pet_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_pet);
        $($path_to_types_root)*::
        exports::pulumi::random::random_shuffle::__export_pulumi_random_random_shuffle_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_shuffle);
        $($path_to_types_root)*::
        exports::pulumi::random::random_string::__export_pulumi_random_random_string_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_string);
        $($path_to_types_root)*::
        exports::pulumi::random::random_uuid::__export_pulumi_random_random_uuid_4_15_0_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::pulumi::random::random_uuid);
    };
}
#[doc(inline)]
pub(crate) use __export_random_pulumi_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:pulumi:random@4.15.0--0.0.0-DEV:random-pulumi:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2816] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xfc\x14\x01A\x02\x01\
A\x15\x01B\x0a\x04\0\x06output\x03\x01\x01i\0\x01@\x01\x05values\0\x01\x04\0\x13\
[constructor]output\x01\x02\x01h\0\x01@\x02\x04self\x03\x0dfunction-names\0\x01\x04\
\0\x12[method]output.map\x01\x04\x01p\x03\x01@\x01\x07outputs\x05\0\x01\x04\0\x07\
combine\x01\x06\x03\00component:pulumi-wasm/output-interface@0.0.0-DEV\x05\0\x02\
\x03\0\0\x06output\x01B\x1c\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\
\x01r\x02\x04names\x05value\x02\x04\0\x0cobject-field\x03\0\x03\x01r\x01\x04name\
s\x04\0\x0cresult-field\x03\0\x05\x01i\x01\x01r\x02\x04names\x06output\x07\x04\0\
\x1eregister-resource-result-field\x03\0\x08\x01p\x04\x01p\x06\x01r\x04\x04types\
\x04names\x06object\x0a\x07results\x0b\x04\0\x19register-resource-request\x03\0\x0c\
\x01p\x09\x01r\x01\x06fields\x0e\x04\0\x18register-resource-result\x03\0\x0f\x01\
r\x02\x04names\x06output\x07\x04\0\x1cresource-invoke-result-field\x03\0\x11\x01\
r\x03\x05tokens\x06object\x0a\x07results\x0b\x04\0\x17resource-invoke-request\x03\
\0\x13\x01p\x12\x01r\x01\x06fields\x15\x04\0\x16resource-invoke-result\x03\0\x16\
\x01@\x01\x07request\x0d\0\x10\x04\0\x08register\x01\x18\x01@\x01\x07request\x14\
\0\x17\x04\0\x06invoke\x01\x19\x03\02component:pulumi-wasm/register-interface@0.\
0.0-DEV\x05\x02\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01\
r\x02\x07keepers\x02\x06length\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x04\x06\
base64\x05\x03hex\x05\x07keepers\x05\x06length\x05\x04\0\x03res\x03\0\x06\x01@\x02\
\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0,pulumi:random/random-b\
ytes@4.15.0--0.0.0-DEV\x05\x03\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\
\0\x01h\x01\x01r\x03\x0bbyte-length\x02\x07keepers\x02\x06prefix\x02\x04\0\x04ar\
gs\x03\0\x03\x01i\x01\x01r\x07\x07b64-std\x05\x07b64-url\x05\x0bbyte-length\x05\x03\
dec\x05\x03hex\x05\x07keepers\x05\x06prefix\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04\
names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0)pulumi:random/random-id@4.\
15.0--0.0.0-DEV\x05\x04\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01\
h\x01\x01r\x04\x07keepers\x02\x03max\x02\x03min\x02\x04seed\x02\x04\0\x04args\x03\
\0\x03\x01i\x01\x01r\x05\x07keepers\x05\x03max\x05\x03min\x05\x06result\x05\x04s\
eed\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invo\
ke\x01\x08\x04\0.pulumi:random/random-integer@4.15.0--0.0.0-DEV\x05\x05\x01B\x0a\
\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x0c\x07keepers\x02\x06\
length\x02\x05lower\x02\x09min-lower\x02\x0bmin-numeric\x02\x0bmin-special\x02\x09\
min-upper\x02\x06number\x02\x07numeric\x02\x10override-special\x02\x07special\x02\
\x05upper\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x0e\x0bbcrypt-hash\x05\x07ke\
epers\x05\x06length\x05\x05lower\x05\x09min-lower\x05\x0bmin-numeric\x05\x0bmin-\
special\x05\x09min-upper\x05\x06number\x05\x07numeric\x05\x10override-special\x05\
\x06result\x05\x07special\x05\x05upper\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04na\
mes\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\x04\0/pulumi:random/random-passwor\
d@4.15.0--0.0.0-DEV\x05\x06\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\
\x01h\x01\x01r\x04\x07keepers\x02\x06length\x02\x06prefix\x02\x09separator\x02\x04\
\0\x04args\x03\0\x03\x01i\x01\x01r\x04\x07keepers\x05\x06length\x05\x06prefix\x05\
\x09separator\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\
\0\x06invoke\x01\x08\x04\0*pulumi:random/random-pet@4.15.0--0.0.0-DEV\x05\x07\x01\
B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x04\x06inputs\x02\
\x07keepers\x02\x0cresult-count\x02\x04seed\x02\x04\0\x04args\x03\0\x03\x01i\x01\
\x01r\x05\x06inputs\x05\x07keepers\x05\x0cresult-count\x05\x07results\x05\x04see\
d\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\
\x01\x08\x04\0.pulumi:random/random-shuffle@4.15.0--0.0.0-DEV\x05\x08\x01B\x0a\x02\
\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x0c\x07keepers\x02\x06len\
gth\x02\x05lower\x02\x09min-lower\x02\x0bmin-numeric\x02\x0bmin-special\x02\x09m\
in-upper\x02\x06number\x02\x07numeric\x02\x10override-special\x02\x07special\x02\
\x05upper\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x0d\x07keepers\x05\x06length\
\x05\x05lower\x05\x09min-lower\x05\x0bmin-numeric\x05\x0bmin-special\x05\x09min-\
upper\x05\x06number\x05\x07numeric\x05\x10override-special\x05\x06result\x05\x07\
special\x05\x05upper\x05\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\
\x04\0\x06invoke\x01\x08\x04\0-pulumi:random/random-string@4.15.0--0.0.0-DEV\x05\
\x09\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01h\x01\x01r\x01\x07k\
eepers\x02\x04\0\x04args\x03\0\x03\x01i\x01\x01r\x02\x07keepers\x05\x06result\x05\
\x04\0\x03res\x03\0\x06\x01@\x02\x04names\x04args\x04\0\x07\x04\0\x06invoke\x01\x08\
\x04\0+pulumi:random/random-uuid@4.15.0--0.0.0-DEV\x05\x0a\x04\0-pulumi:random/r\
andom-pulumi@4.15.0--0.0.0-DEV\x04\0\x0b\x13\x01\0\x0drandom-pulumi\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rus\
t\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
