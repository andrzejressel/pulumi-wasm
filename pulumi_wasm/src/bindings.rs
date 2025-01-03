#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod pulumi_wasm_external {
        #[allow(dead_code, clippy::all)]
        pub mod log {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Level {
                Trace,
                Debug,
                Info,
                Warn,
                Error,
            }
            impl ::core::fmt::Debug for Level {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Level::Trace => f.debug_tuple("Level::Trace").finish(),
                        Level::Debug => f.debug_tuple("Level::Debug").finish(),
                        Level::Info => f.debug_tuple("Level::Info").finish(),
                        Level::Warn => f.debug_tuple("Level::Warn").finish(),
                        Level::Error => f.debug_tuple("Level::Error").finish(),
                    }
                }
            }
            impl Level {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Level {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Level::Trace,
                        1 => Level::Debug,
                        2 => Level::Info,
                        3 => Level::Warn,
                        4 => Level::Error,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[derive(Clone)]
            pub struct Content {
                pub level: Level,
                pub target: _rt::String,
                pub args: _rt::String,
                pub module_path: Option<_rt::String>,
                pub file: Option<_rt::String>,
                pub line: Option<u32>,
                pub key_values: _rt::Vec<(_rt::String, _rt::String)>,
            }
            impl ::core::fmt::Debug for Content {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Content")
                        .field("level", &self.level)
                        .field("target", &self.target)
                        .field("args", &self.args)
                        .field("module-path", &self.module_path)
                        .field("file", &self.file)
                        .field("line", &self.line)
                        .field("key-values", &self.key_values)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn log(content: &Content) {
                unsafe {
                    let Content {
                        level: level0,
                        target: target0,
                        args: args0,
                        module_path: module_path0,
                        file: file0,
                        line: line0,
                        key_values: key_values0,
                    } = content;
                    let vec1 = target0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = args0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    let (result4_0, result4_1, result4_2) = match module_path0 {
                        Some(e) => {
                            let vec3 = e;
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            (1i32, ptr3.cast_mut(), len3)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    let (result6_0, result6_1, result6_2) = match file0 {
                        Some(e) => {
                            let vec5 = e;
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            (1i32, ptr5.cast_mut(), len5)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    let (result7_0, result7_1) = match line0 {
                        Some(e) => (1i32, _rt::as_i32(e)),
                        None => (0i32, 0i32),
                    };
                    let vec11 = key_values0;
                    let len11 = vec11.len();
                    let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec11.len() * 16,
                        4,
                    );
                    let result11 = if layout11.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout11);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec11.into_iter().enumerate() {
                        let base = result11.add(i * 16);
                        {
                            let (t8_0, t8_1) = e;
                            let vec9 = t8_0;
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            *base.add(4).cast::<usize>() = len9;
                            *base.add(0).cast::<*mut u8>() = ptr9.cast_mut();
                            let vec10 = t8_1;
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            *base.add(12).cast::<usize>() = len10;
                            *base.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/log@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "log"]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        level0.clone() as i32,
                        ptr1.cast_mut(),
                        len1,
                        ptr2.cast_mut(),
                        len2,
                        result4_0,
                        result4_1,
                        result4_2,
                        result6_0,
                        result6_1,
                        result6_2,
                        result7_0,
                        result7_1,
                        result11,
                        len11,
                    );
                    if layout11.size() != 0 {
                        _rt::alloc::dealloc(result11.cast(), layout11);
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod external_world {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct ResourceInvokeRequest {
                pub output_id: _rt::String,
                pub body: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for ResourceInvokeRequest {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ResourceInvokeRequest")
                        .field("output-id", &self.output_id)
                        .field("body", &self.body)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct RegisterResourceRequest {
                pub output_id: _rt::String,
                pub body: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for RegisterResourceRequest {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RegisterResourceRequest")
                        .field("output-id", &self.output_id)
                        .field("body", &self.body)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct RegisteredResource {
                pub output_id: _rt::String,
                pub body: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for RegisteredResource {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RegisteredResource")
                        .field("output-id", &self.output_id)
                        .field("body", &self.body)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn is_in_preview() -> bool {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "is-in-preview"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    _rt::bool_lift(ret as u8)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_root_resource() -> _rt::String {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "get-root-resource"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                    _rt::string_lift(bytes3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn register_resource_outputs(request: &[u8]) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = request;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "register-resource-outputs"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resource_invoke(request: &ResourceInvokeRequest) {
                unsafe {
                    let ResourceInvokeRequest { output_id: output_id0, body: body0 } = request;
                    let vec1 = output_id0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = body0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "resource-invoke"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr1.cast_mut(), len1, ptr2.cast_mut(), len2);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn register_resource(request: &RegisterResourceRequest) {
                unsafe {
                    let RegisterResourceRequest { output_id: output_id0, body: body0 } = request;
                    let vec1 = output_id0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = body0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "register-resource"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr1.cast_mut(), len1, ptr2.cast_mut(), len2);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn wait_for_resource_operations() -> _rt::Vec<RegisteredResource> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV"
                    )]
                    extern "C" {
                        #[link_name = "wait-for-resource-operations"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base9 = l1;
                    let len9 = l2;
                    let mut result9 = _rt::Vec::with_capacity(len9);
                    for i in 0..len9 {
                        let base = base9.add(i * 16);
                        let e9 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            let l6 = *base.add(8).cast::<*mut u8>();
                            let l7 = *base.add(12).cast::<usize>();
                            let len8 = l7;
                            RegisteredResource {
                                output_id: _rt::string_lift(bytes5),
                                body: _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                            }
                        };
                        result9.push(e9);
                    }
                    _rt::cabi_dealloc(base9, len9 * 16, 4);
                    result9
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod pulumi_wasm {
            #[allow(dead_code, clippy::all)]
            pub mod output_interface {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Output {
                    handle: _rt::Resource<Output>,
                }
                type _OutputRep<T> = Option<T>;
                impl Output {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Output`.
                    pub fn new<T: GuestOutput>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _OutputRep<T> = Some(val);
                        let ptr: *mut _OutputRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestOutput>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestOutput>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestOutput>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _OutputRep<T>);
                    }
                    fn as_ptr<T: GuestOutput>(&self) -> *mut _OutputRep<T> {
                        Output::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Output`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct OutputBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Output>,
                }
                impl<'a> OutputBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestOutput>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _OutputRep<T> {
                        Output::type_guard::<T>();
                        self.rep.cast()
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
                                wasm_import_module = "[export]component:pulumi-wasm/output-interface@0.0.0-DEV"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]output"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_output_cabi<T: GuestOutput>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = Output::new(T::new(_rt::string_lift(bytes0)));
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_output_map_cabi<T: GuestOutput>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::map(
                        OutputBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_combine_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base1 = arg0;
                    let len1 = arg1;
                    let mut result1 = _rt::Vec::with_capacity(len1);
                    for i in 0..len1 {
                        let base = base1.add(i * 4);
                        let e1 = {
                            let l0 = *base.add(0).cast::<i32>();
                            OutputBorrow::lift(l0 as u32 as usize)
                        };
                        result1.push(e1);
                    }
                    _rt::cabi_dealloc(base1, len1 * 4, 4);
                    let result2 = T::combine(result1);
                    (result2).take_handle() as i32
                }
                pub trait Guest {
                    type Output: GuestOutput;
                    fn combine(outputs: _rt::Vec<OutputBorrow<'_>>) -> Output;
                }
                pub trait GuestOutput: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:pulumi-wasm/output-interface@0.0.0-DEV"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]output"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:pulumi-wasm/output-interface@0.0.0-DEV"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]output"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(value: _rt::String) -> Self;
                    fn map(&self, function_name: _rt::String) -> Output;
                }
                #[doc(hidden)]
                macro_rules! __export_component_pulumi_wasm_output_interface_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:pulumi-wasm/output-interface@0.0.0-DEV#[constructor]output"]
                        unsafe extern "C" fn export_constructor_output(arg0 : * mut u8,
                        arg1 : usize,) -> i32 { $($path_to_types)*::
                        _export_constructor_output_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Output > (arg0, arg1) } #[export_name =
                        "component:pulumi-wasm/output-interface@0.0.0-DEV#[method]output.map"]
                        unsafe extern "C" fn export_method_output_map(arg0 : * mut u8,
                        arg1 : * mut u8, arg2 : usize,) -> i32 { $($path_to_types)*::
                        _export_method_output_map_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Output > (arg0, arg1, arg2) } #[export_name =
                        "component:pulumi-wasm/output-interface@0.0.0-DEV#combine"]
                        unsafe extern "C" fn export_combine(arg0 : * mut u8, arg1 :
                        usize,) -> i32 { $($path_to_types)*:: _export_combine_cabi::<$ty
                        > (arg0, arg1) } const _ : () = { #[doc(hidden)] #[export_name =
                        "component:pulumi-wasm/output-interface@0.0.0-DEV#[dtor]output"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Output::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Output > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_pulumi_wasm_output_interface_0_0_0_dev_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod register_interface {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::exports::component::pulumi_wasm::output_interface::Output;
                pub type OutputBorrow<'a> = super::super::super::super::exports::component::pulumi_wasm::output_interface::OutputBorrow<
                    'a,
                >;
                pub struct ObjectField<'a> {
                    pub name: _rt::String,
                    pub value: OutputBorrow<'a>,
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
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_register_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                    arg6: *mut u8,
                    arg7: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let base6 = arg4;
                    let len6 = arg5;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 12);
                        let e6 = {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            let len4 = l3;
                            let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                            let l5 = *base.add(8).cast::<i32>();
                            ObjectField {
                                name: _rt::string_lift(bytes4),
                                value: OutputBorrow::lift(l5 as u32 as usize),
                            }
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 12, 4);
                    let base10 = arg6;
                    let len10 = arg7;
                    let mut result10 = _rt::Vec::with_capacity(len10);
                    for i in 0..len10 {
                        let base = base10.add(i * 8);
                        let e10 = {
                            let l7 = *base.add(0).cast::<*mut u8>();
                            let l8 = *base.add(4).cast::<usize>();
                            let len9 = l8;
                            let bytes9 = _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                            ResultField {
                                name: _rt::string_lift(bytes9),
                            }
                        };
                        result10.push(e10);
                    }
                    _rt::cabi_dealloc(base10, len10 * 8, 4);
                    let result11 = T::register(RegisterResourceRequest {
                        type_: _rt::string_lift(bytes0),
                        name: _rt::string_lift(bytes1),
                        object: result6,
                        results: result10,
                    });
                    let ptr12 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let RegisterResourceResult { fields: fields13 } = result11;
                    let vec16 = fields13;
                    let len16 = vec16.len();
                    let layout16 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec16.len() * 12,
                        4,
                    );
                    let result16 = if layout16.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout16).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout16);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec16.into_iter().enumerate() {
                        let base = result16.add(i * 12);
                        {
                            let RegisterResourceResultField {
                                name: name14,
                                output: output14,
                            } = e;
                            let vec15 = (name14.into_bytes()).into_boxed_slice();
                            let ptr15 = vec15.as_ptr().cast::<u8>();
                            let len15 = vec15.len();
                            ::core::mem::forget(vec15);
                            *base.add(4).cast::<usize>() = len15;
                            *base.add(0).cast::<*mut u8>() = ptr15.cast_mut();
                            *base.add(8).cast::<i32>() = (output14).take_handle() as i32;
                        }
                    }
                    *ptr12.add(4).cast::<usize>() = len16;
                    *ptr12.add(0).cast::<*mut u8>() = result16;
                    ptr12
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_register<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base4 = l0;
                    let len4 = l1;
                    for i in 0..len4 {
                        let base = base4.add(i * 12);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                        }
                    }
                    _rt::cabi_dealloc(base4, len4 * 12, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_invoke_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let base5 = arg2;
                    let len5 = arg3;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<i32>();
                            ObjectField {
                                name: _rt::string_lift(bytes3),
                                value: OutputBorrow::lift(l4 as u32 as usize),
                            }
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    let base9 = arg4;
                    let len9 = arg5;
                    let mut result9 = _rt::Vec::with_capacity(len9);
                    for i in 0..len9 {
                        let base = base9.add(i * 8);
                        let e9 = {
                            let l6 = *base.add(0).cast::<*mut u8>();
                            let l7 = *base.add(4).cast::<usize>();
                            let len8 = l7;
                            let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                            ResultField {
                                name: _rt::string_lift(bytes8),
                            }
                        };
                        result9.push(e9);
                    }
                    _rt::cabi_dealloc(base9, len9 * 8, 4);
                    let result10 = T::invoke(ResourceInvokeRequest {
                        token: _rt::string_lift(bytes0),
                        object: result5,
                        results: result9,
                    });
                    let ptr11 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let ResourceInvokeResult { fields: fields12 } = result10;
                    let vec15 = fields12;
                    let len15 = vec15.len();
                    let layout15 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec15.len() * 12,
                        4,
                    );
                    let result15 = if layout15.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout15).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout15);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec15.into_iter().enumerate() {
                        let base = result15.add(i * 12);
                        {
                            let ResourceInvokeResultField {
                                name: name13,
                                output: output13,
                            } = e;
                            let vec14 = (name13.into_bytes()).into_boxed_slice();
                            let ptr14 = vec14.as_ptr().cast::<u8>();
                            let len14 = vec14.len();
                            ::core::mem::forget(vec14);
                            *base.add(4).cast::<usize>() = len14;
                            *base.add(0).cast::<*mut u8>() = ptr14.cast_mut();
                            *base.add(8).cast::<i32>() = (output13).take_handle() as i32;
                        }
                    }
                    *ptr11.add(4).cast::<usize>() = len15;
                    *ptr11.add(0).cast::<*mut u8>() = result15;
                    ptr11
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_invoke<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base4 = l0;
                    let len4 = l1;
                    for i in 0..len4 {
                        let base = base4.add(i * 12);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                        }
                    }
                    _rt::cabi_dealloc(base4, len4 * 12, 4);
                }
                pub trait Guest {
                    fn register(
                        request: RegisterResourceRequest<'_>,
                    ) -> RegisterResourceResult;
                    fn invoke(
                        request: ResourceInvokeRequest<'_>,
                    ) -> ResourceInvokeResult;
                }
                #[doc(hidden)]
                macro_rules! __export_component_pulumi_wasm_register_interface_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:pulumi-wasm/register-interface@0.0.0-DEV#register"]
                        unsafe extern "C" fn export_register(arg0 : * mut u8, arg1 :
                        usize, arg2 : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 :
                        usize, arg6 : * mut u8, arg7 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_register_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5, arg6, arg7) } #[export_name =
                        "cabi_post_component:pulumi-wasm/register-interface@0.0.0-DEV#register"]
                        unsafe extern "C" fn _post_return_register(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_register::<$ty > (arg0) }
                        #[export_name =
                        "component:pulumi-wasm/register-interface@0.0.0-DEV#invoke"]
                        unsafe extern "C" fn export_invoke(arg0 : * mut u8, arg1 : usize,
                        arg2 : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 : usize,) ->
                        * mut u8 { $($path_to_types)*:: _export_invoke_cabi::<$ty >
                        (arg0, arg1, arg2, arg3, arg4, arg5) } #[export_name =
                        "cabi_post_component:pulumi-wasm/register-interface@0.0.0-DEV#invoke"]
                        unsafe extern "C" fn _post_return_invoke(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_invoke::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_pulumi_wasm_register_interface_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod stack_interface {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Output = super::super::super::super::exports::component::pulumi_wasm::output_interface::Output;
                pub type OutputBorrow<'a> = super::super::super::super::exports::component::pulumi_wasm::output_interface::OutputBorrow<
                    'a,
                >;
                pub struct FunctionInvocationRequest {
                    pub id: Output,
                    pub function_id: _rt::String,
                    pub value: _rt::String,
                }
                impl ::core::fmt::Debug for FunctionInvocationRequest {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("FunctionInvocationRequest")
                            .field("id", &self.id)
                            .field("function-id", &self.function_id)
                            .field("value", &self.value)
                            .finish()
                    }
                }
                pub struct FunctionInvocationResult<'a> {
                    pub id: OutputBorrow<'a>,
                    pub value: _rt::String,
                }
                impl<'a> ::core::fmt::Debug for FunctionInvocationResult<'a> {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("FunctionInvocationResult")
                            .field("id", &self.id)
                            .field("value", &self.value)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_export_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    T::add_export(
                        _rt::string_lift(bytes0),
                        OutputBorrow::lift(arg2 as u32 as usize),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_finish_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base4 = arg0;
                    let len4 = arg1;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 12);
                        let e4 = {
                            let l0 = *base.add(0).cast::<i32>();
                            let l1 = *base.add(4).cast::<*mut u8>();
                            let l2 = *base.add(8).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            FunctionInvocationResult {
                                id: OutputBorrow::lift(l0 as u32 as usize),
                                value: _rt::string_lift(bytes3),
                            }
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 12, 4);
                    let result5 = T::finish(result4);
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec10 = result5;
                    let len10 = vec10.len();
                    let layout10 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec10.len() * 20,
                        4,
                    );
                    let result10 = if layout10.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout10).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout10);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec10.into_iter().enumerate() {
                        let base = result10.add(i * 20);
                        {
                            let FunctionInvocationRequest {
                                id: id7,
                                function_id: function_id7,
                                value: value7,
                            } = e;
                            *base.add(0).cast::<i32>() = (id7).take_handle() as i32;
                            let vec8 = (function_id7.into_bytes()).into_boxed_slice();
                            let ptr8 = vec8.as_ptr().cast::<u8>();
                            let len8 = vec8.len();
                            ::core::mem::forget(vec8);
                            *base.add(8).cast::<usize>() = len8;
                            *base.add(4).cast::<*mut u8>() = ptr8.cast_mut();
                            let vec9 = (value7.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *base.add(16).cast::<usize>() = len9;
                            *base.add(12).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    }
                    *ptr6.add(4).cast::<usize>() = len10;
                    *ptr6.add(0).cast::<*mut u8>() = result10;
                    ptr6
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_finish<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base6 = l0;
                    let len6 = l1;
                    for i in 0..len6 {
                        let base = base6.add(i * 20);
                        {
                            let l2 = *base.add(4).cast::<*mut u8>();
                            let l3 = *base.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                            let l4 = *base.add(12).cast::<*mut u8>();
                            let l5 = *base.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                    _rt::cabi_dealloc(base6, len6 * 20, 4);
                }
                pub trait Guest {
                    fn add_export(name: _rt::String, value: OutputBorrow<'_>);
                    fn finish(
                        functions: _rt::Vec<FunctionInvocationResult<'_>>,
                    ) -> _rt::Vec<FunctionInvocationRequest>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_pulumi_wasm_stack_interface_0_0_0_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:pulumi-wasm/stack-interface@0.0.0-DEV#add-export"]
                        unsafe extern "C" fn export_add_export(arg0 : * mut u8, arg1 :
                        usize, arg2 : i32,) { $($path_to_types)*::
                        _export_add_export_cabi::<$ty > (arg0, arg1, arg2) }
                        #[export_name =
                        "component:pulumi-wasm/stack-interface@0.0.0-DEV#finish"] unsafe
                        extern "C" fn export_finish(arg0 : * mut u8, arg1 : usize,) -> *
                        mut u8 { $($path_to_types)*:: _export_finish_cabi::<$ty > (arg0,
                        arg1) } #[export_name =
                        "cabi_post_component:pulumi-wasm/stack-interface@0.0.0-DEV#finish"]
                        unsafe extern "C" fn _post_return_finish(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_finish::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_pulumi_wasm_stack_interface_0_0_0_dev_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
        #[allow(dead_code)]
        pub mod pulumi_wasm_external {
            #[allow(dead_code, clippy::all)]
            pub mod pulumi_settings {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_set_in_preview_cabi<T: Guest>(arg0: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_in_preview(_rt::bool_lift(arg0 as u8));
                }
                pub trait Guest {
                    fn set_in_preview(in_preview: bool);
                }
                #[doc(hidden)]
                macro_rules! __export_component_pulumi_wasm_external_pulumi_settings_0_0_0_stable_dev_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:pulumi-wasm-external/pulumi-settings@0.0.0-STABLE-DEV#set-in-preview"]
                        unsafe extern "C" fn export_set_in_preview(arg0 : i32,) {
                        $($path_to_types)*:: _export_set_in_preview_cabi::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_pulumi_wasm_external_pulumi_settings_0_0_0_stable_dev_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::alloc;
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
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
    pub use alloc_crate::boxed::Box;
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
macro_rules! __export_pulumi_wasm_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::pulumi_wasm::output_interface::__export_component_pulumi_wasm_output_interface_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::pulumi_wasm::output_interface); $($path_to_types_root)*::
        exports::component::pulumi_wasm::register_interface::__export_component_pulumi_wasm_register_interface_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::pulumi_wasm::register_interface); $($path_to_types_root)*::
        exports::component::pulumi_wasm::stack_interface::__export_component_pulumi_wasm_stack_interface_0_0_0_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::pulumi_wasm::stack_interface); $($path_to_types_root)*::
        exports::component::pulumi_wasm_external::pulumi_settings::__export_component_pulumi_wasm_external_pulumi_settings_0_0_0_stable_dev_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::pulumi_wasm_external::pulumi_settings);
    };
}
#[doc(inline)]
pub(crate) use __export_pulumi_wasm_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:component:pulumi-wasm@0.0.0-DEV:pulumi-wasm:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1882] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd8\x0d\x01A\x02\x01\
A\x0d\x01B\x0a\x01m\x05\x05TRACE\x05DEBUG\x04INFO\x04WARN\x05ERROR\x04\0\x05leve\
l\x03\0\0\x01ks\x01ky\x01o\x02ss\x01p\x04\x01r\x07\x05level\x01\x06targets\x04ar\
gss\x0bmodule-path\x02\x04file\x02\x04line\x03\x0akey-values\x05\x04\0\x07conten\
t\x03\0\x06\x01@\x01\x07content\x07\x01\0\x04\0\x03log\x01\x08\x03\03component:p\
ulumi-wasm-external/log@0.0.0-STABLE-DEV\x05\0\x01B\x14\x01p}\x01r\x02\x09output\
-ids\x04body\0\x04\0\x17resource-invoke-request\x03\0\x01\x01r\x02\x09output-ids\
\x04body\0\x04\0\x19register-resource-request\x03\0\x03\x01r\x02\x09output-ids\x04\
body\0\x04\0\x13registered-resource\x03\0\x05\x01@\0\0\x7f\x04\0\x0dis-in-previe\
w\x01\x07\x01@\0\0s\x04\0\x11get-root-resource\x01\x08\x01@\x01\x07request\0\0\0\
\x04\0\x19register-resource-outputs\x01\x09\x01@\x01\x07request\x02\x01\0\x04\0\x0f\
resource-invoke\x01\x0a\x01@\x01\x07request\x04\x01\0\x04\0\x11register-resource\
\x01\x0b\x01p\x06\x01@\0\0\x0c\x04\0\x1cwait-for-resource-operations\x01\x0d\x03\
\0>component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV\x05\x01\x01B\x0a\
\x04\0\x06output\x03\x01\x01i\0\x01@\x01\x05values\0\x01\x04\0\x13[constructor]o\
utput\x01\x02\x01h\0\x01@\x02\x04self\x03\x0dfunction-names\0\x01\x04\0\x12[meth\
od]output.map\x01\x04\x01p\x03\x01@\x01\x07outputs\x05\0\x01\x04\0\x07combine\x01\
\x06\x04\00component:pulumi-wasm/output-interface@0.0.0-DEV\x05\x02\x02\x03\0\x02\
\x06output\x01B\x1c\x02\x03\x02\x01\x03\x04\0\x06output\x03\0\0\x01h\x01\x01r\x02\
\x04names\x05value\x02\x04\0\x0cobject-field\x03\0\x03\x01r\x01\x04names\x04\0\x0c\
result-field\x03\0\x05\x01i\x01\x01r\x02\x04names\x06output\x07\x04\0\x1eregiste\
r-resource-result-field\x03\0\x08\x01p\x04\x01p\x06\x01r\x04\x04types\x04names\x06\
object\x0a\x07results\x0b\x04\0\x19register-resource-request\x03\0\x0c\x01p\x09\x01\
r\x01\x06fields\x0e\x04\0\x18register-resource-result\x03\0\x0f\x01r\x02\x04name\
s\x06output\x07\x04\0\x1cresource-invoke-result-field\x03\0\x11\x01r\x03\x05toke\
ns\x06object\x0a\x07results\x0b\x04\0\x17resource-invoke-request\x03\0\x13\x01p\x12\
\x01r\x01\x06fields\x15\x04\0\x16resource-invoke-result\x03\0\x16\x01@\x01\x07re\
quest\x0d\0\x10\x04\0\x08register\x01\x18\x01@\x01\x07request\x14\0\x17\x04\0\x06\
invoke\x01\x19\x04\02component:pulumi-wasm/register-interface@0.0.0-DEV\x05\x04\x01\
B\x0e\x02\x03\x02\x01\x03\x04\0\x06output\x03\0\0\x01i\x01\x01r\x03\x02id\x02\x0b\
function-ids\x05values\x04\0\x1bfunction-invocation-request\x03\0\x03\x01h\x01\x01\
r\x02\x02id\x05\x05values\x04\0\x1afunction-invocation-result\x03\0\x06\x01@\x02\
\x04names\x05value\x05\x01\0\x04\0\x0aadd-export\x01\x08\x01p\x07\x01p\x04\x01@\x01\
\x09functions\x09\0\x0a\x04\0\x06finish\x01\x0b\x04\0/component:pulumi-wasm/stac\
k-interface@0.0.0-DEV\x05\x05\x01B\x02\x01@\x01\x0ain-preview\x7f\x01\0\x04\0\x0e\
set-in-preview\x01\0\x04\0?component:pulumi-wasm-external/pulumi-settings@0.0.0-\
STABLE-DEV\x05\x06\x04\0+component:pulumi-wasm/pulumi-wasm@0.0.0-DEV\x04\0\x0b\x11\
\x01\0\x0bpulumi-wasm\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-com\
ponent\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
