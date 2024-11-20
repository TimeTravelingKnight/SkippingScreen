#![feature(lazy_cell)]

use std::ffi::{c_int, c_void};
use std::{slice};
use hook_module::hooks::GM_HOOK;
use DetourMap::detourMaps::{enableGameMode, install_hooks_mapCheck, mmbnlc_enable_noesis};



mod DetourMap;
mod helpermacros;
mod hook_module;




#[no_mangle]
pub  extern "C" fn luaopen_SkipTitleLC2(_:c_void)-> c_int{
    install_hooks_mapCheck();
    
    unsafe{mmbnlc_enable_noesis.enable().unwrap()};
    //GM_HOOK!(0x140d1ca28,softReset,12);
    1

}