use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double};
use crate::backend::vm::VM;
use crate::frontend::{Lexer, Parser, Compiler};
use crate::shared::LumaError;

// C API for embedding Luma in other applications
#[repr(C)]
pub struct LumaVM {
    vm: Box<VM>,
}

#[repr(C)]
pub struct LumaResult {
    success: bool,
    error_message: *mut c_char,
}

#[no_mangle]
pub extern "C" fn luma_vm_new() -> *mut LumaVM {
    let vm = Box::new(LumaVM {
        vm: Box::new(VM::new()),
    });
    Box::into_raw(vm)
}

#[no_mangle]
pub extern "C" fn luma_vm_free(vm: *mut LumaVM) {
    if !vm.is_null() {
        unsafe {
            let _ = Box::from_raw(vm);
        }
    }
}

#[no_mangle]
pub extern "C" fn luma_execute_source(vm: *mut LumaVM, source: *const c_char) -> LumaResult {
    if vm.is_null() || source.is_null() {
        return LumaResult {
            success: false,
            error_message: create_error_string("Invalid VM or source pointer"),
        };
    }

    let source_str = unsafe {
        match CStr::from_ptr(source).to_str() {
            Ok(s) => s,
            Err(_) => {
                return LumaResult {
                    success: false,
                    error_message: create_error_string("Invalid UTF-8 in source"),
                };
            }
        }
    };

    let vm_ref = unsafe { &mut *vm };
    
    match execute_luma_source(&mut vm_ref.vm, source_str) {
        Ok(_) => LumaResult {
            success: true,
            error_message: std::ptr::null_mut(),
        },
        Err(e) => LumaResult {
            success: false,
            error_message: create_error_string(&e.to_string()),
        },
    }
}

#[no_mangle]
pub extern "C" fn luma_set_global_number(vm: *mut LumaVM, name: *const c_char, _value: c_double) -> LumaResult {
    if vm.is_null() || name.is_null() {
        return LumaResult {
            success: false,
            error_message: create_error_string("Invalid VM or name pointer"),
        };
    }

    let _name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => {
                return LumaResult {
                    success: false,
                    error_message: create_error_string("Invalid UTF-8 in name"),
                };
            }
        }
    };

    // This would set a global variable in the VM
    // Implementation depends on VM internals
    LumaResult {
        success: true,
        error_message: std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn luma_get_global_number(vm: *mut LumaVM, name: *const c_char, value: *mut c_double) -> LumaResult {
    if vm.is_null() || name.is_null() || value.is_null() {
        return LumaResult {
            success: false,
            error_message: create_error_string("Invalid pointer"),
        };
    }

    // Implementation would get global variable from VM
    unsafe {
        *value = 0.0; // Placeholder
    }

    LumaResult {
        success: true,
        error_message: std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn luma_free_error_message(message: *mut c_char) {
    if !message.is_null() {
        unsafe {
            let _ = CString::from_raw(message);
        }
    }
}

// Helper functions
fn create_error_string(message: &str) -> *mut c_char {
    match CString::new(message) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => {
            // If we can't create a CString, return a generic error
            CString::new("Error creating error message").unwrap().into_raw()
        }
    }
}

fn execute_luma_source(vm: &mut VM, source: &str) -> Result<(), LumaError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| LumaError::LexError(e.to_string()))?;
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().map_err(|e| LumaError::ParseError(e.to_string()))?;
    
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements)?;
    
    vm.interpret(chunk)?;
    
    Ok(())
}

// Example C header that would be generated:
/*
// luma.h
#ifndef LUMA_H
#define LUMA_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct LumaVM LumaVM;

typedef struct {
    bool success;
    char* error_message;
} LumaResult;

// VM management
LumaVM* luma_vm_new(void);
void luma_vm_free(LumaVM* vm);

// Code execution
LumaResult luma_execute_source(LumaVM* vm, const char* source);

// Variable access
LumaResult luma_set_global_number(LumaVM* vm, const char* name, double value);
LumaResult luma_get_global_number(LumaVM* vm, const char* name, double* value);

// Memory management
void luma_free_error_message(char* message);

#ifdef __cplusplus
}
#endif

#endif // LUMA_H
*/