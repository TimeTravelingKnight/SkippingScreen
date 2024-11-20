use std::{ffi::c_int, io::{Error, ErrorKind}};

use retour::static_detour;




static_detour! {
  pub static mmbnlc_enable_noesis: unsafe extern "system" fn(
   *mut  u64
  );
 
}
const enablechange:usize=0x145117ab2;
const offset:usize=0x1530;
const music:u32=0x150dd371+1;
//static mut count:u8=0;
static mut addr:usize=0;
static mut init:u8=0;
const getAddrStruct:usize=0x142fa93e4;
const getAddrStructSettings:usize=0x142f1d7e0;
const ResourceDicGet:usize=0x1431d6050;
const funcTitle:usize=0x142f47910;
//const maxCount:u8=2;

pub fn currGame(game:u8) ->Result<&'static str,Error >{
match game {
  0 => Ok("EXE1\0"),
  2 => Ok("EXE2\0"),
  3 => Ok("EXE3\0"),
  4 => Ok("EXE3B\0"),
  5 => Ok("EXE4\0"),
  6 => Ok("EXE4B\0"),
  7 => Ok("EXE5\0"),
  8 => Ok("EXE5K\0"),
  9 => Ok("EXE6\0"),
  10 => Ok("EXE6F\0"),
  _ => Err(Error::new(ErrorKind::Other,"NonValid LC Game\0")),
}

}


pub fn noesis(structofnoesis :*mut u64) {
 unsafe{ 

  if init ==0   {
 
    
    let func: fn()->u64=std::mem::transmute(getAddrStruct as *const ()); //set music for menu 
    let x=func() as  u64;
     *((x+0xC) as *mut u32)=music;

    let func: fn()->u64=std::mem::transmute(getAddrStructSettings as *const ()); //read launcher details
    let y =func() as u64;

    let currgame=*((y+0x48)  as *mut u8);
    let loadGame=currGame(currgame).unwrap();

    let getValue:fn(u64 ,&str)->*mut u8 =std::mem::transmute( (*(ResourceDicGet as *mut usize) )as *const ());

    let address=getValue( *((structofnoesis as usize+0x15d0 )as *mut u64),loadGame);
    let funcToSetTitle: fn(usize,usize) =  std::mem::transmute(funcTitle as *const ());
    funcToSetTitle(structofnoesis as usize,address as usize);     
     
  
    
  * (enablechange as *mut u8) =0x1;
  *((structofnoesis as usize +offset) as *mut u32)=0x6;

  init =1;
  }

 }
 unsafe{mmbnlc_enable_noesis.call(structofnoesis)}
}
  pub fn enableGameMode(){
unsafe {
  * (enablechange as *mut u8) =0x1;
  *((addr) as *mut u32)=0x6;


}

  }

  
 
  
  
const switchGameMod:u64=0x142f425a0;  


pub fn install_hooks_mapCheck(){
 unsafe{ mmbnlc_enable_noesis.initialize(   std::mem::transmute(switchGameMod as *mut u8 ), 
  move |a| noesis(a)).unwrap();
 }
 
      
 }
