#![cfg(windows)]

extern crate core;

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, UINT};
use winapi::shared::d3d9;
use winapi::um::winnt::{LPCSTR};
use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress};
use winapi::um::consoleapi;
use std::ptr;
use std::ffi::CString;
use std::mem;
use winapi::shared::d3d9::IDirect3D9;

type _D3DCreate9 =  extern "stdcall" fn(UINT) -> *mut d3d9::IDirect3D9;
type _D3DPERF_SetOptions = extern "stdcall" fn(DWORD);

static mut hOriginal: HINSTANCE = ptr::null_mut();
static mut pDirect3DCreate9: Option<_D3DCreate9> = None;
static mut pD3DPERF_SetOptions: Option<_D3DPERF_SetOptions> = None;

#[no_mangle]
pub unsafe extern "system" fn D3DPERF_SetOptions(dwOptions: DWORD) {
    println!("Inside Set Options");
    match pD3DPERF_SetOptions {
        Some(func) => func(dwOptions),
        None => panic!("oh god")
    }
}

//oh god oh fuck
#[no_mangle]
pub unsafe extern "stdcall" fn Direct3DCreate9(SDKVersion: UINT) -> *mut IDirect3D9 {
    println!("Inside Hook");
    println!("SDKVersion: {}", SDKVersion);
    match pDirect3DCreate9 {
        Some(func) => func(SDKVersion),
        None => panic!("How")
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(    //system is cross-platform guaranteed, but this is only windows so its == "stdcall" i think maybe
                               dll_module: HINSTANCE,
                               call_reason: DWORD,
                               reserved: LPVOID)
                               -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => initialize(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    minwindef::TRUE
}

fn initialize() {
    //cstring everything because rust strings are not
    unsafe {
        hOriginal = LoadLibraryA(CString::new("C:\\Windows\\System32\\d3d9.dll").unwrap().as_ptr());
        pDirect3DCreate9 = Some(mem::transmute(GetProcAddress(hOriginal, CString::new("Direct3DCreate9").unwrap().as_ptr())));
        pD3DPERF_SetOptions = Some(mem::transmute(GetProcAddress(hOriginal, CString::new("D3DPERF_SetOptions").unwrap().as_ptr())));
        consoleapi::AllocConsole();
    }
    println!("pink loaded")
}