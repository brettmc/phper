use std::os::raw::{c_char, c_ushort, c_uint, c_uchar};
use std::ffi::CStr;
use crate::sys::{zend_module_entry, c_str, ZEND_MODULE_API_NO, PHP_BUILD_ID, zend_function_entry};
use std::mem::size_of;
use std::ptr::{null, null_mut};
use crate::function::Function;

#[derive(Debug)]
pub struct Module<'a> {
    name: &'a CStr,
    version: &'a CStr,
    functions: Option<Vec<Function<'a>>>,
}

impl<'a> Module<'a> {
    pub fn new(name: &'a CStr) -> Self {
        Self {
            name,
            version: c_str!(env!("CARGO_PKG_VERSION")),
            functions: None,
        }
    }

    pub fn version(mut self, version: &'a CStr) -> Self {
        self.version = version;
        self
    }

    pub fn functions(mut self, functions: Vec<Function<'a>>) -> Self {
        self.functions = Some(functions);
        self
    }

    pub fn into_box_entry(self) -> Box<zend_module_entry> {
        let functions = match self.functions {
            Some(functions) => {
                let mut entries = Vec::with_capacity(functions.len() + 1);
                for function in functions {
                    entries.push(
                        zend_function_entry {
                            fname: function.name.as_ptr(),
                            handler: Some(function.func),
                            arg_info: null(),
                            num_args: 0,
                            flags: 0,
                        }
                    );
                }
                entries.push(
                    zend_function_entry {
                        fname: null(),
                        handler: None,
                        arg_info: null(),
                        num_args: 0,
                        flags: 0,
                    }
                );

                Box::into_raw(entries.into_boxed_slice()) as *const zend_function_entry
            }
            None => null(),
        };

        Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            // TODO Don't support `DEBUG` now.
            zend_debug: 0,
            // TODO Don't support `ZTS` now.
            zts: 0 as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: self.name.as_ptr(),
            functions,
            module_startup_func: None,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: None,
            version: self.version.as_ptr(),
            globals_size: 0usize,
            globals_ptr: null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: PHP_BUILD_ID,
        })
    }
}

impl From<Module<'_>> for *const zend_module_entry {
    fn from(module: Module<'_>) -> Self {
        Box::into_raw(module.into_box_entry())
    }
}
