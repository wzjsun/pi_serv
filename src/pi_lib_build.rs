use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use std::mem::transmute;
use core;
use core::convert::From;
use pi_lib;



fn call_1549520222(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


    let result = pi_lib::atom::Atom::from(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,1411051473);


    Some(CallResult::Ok)
}


fn call_1469354144(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst0.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst0 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u16();


    let result = pi_lib::guid::GuidGen::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,1706731228);


    Some(CallResult::Ok)
}


fn call_748243846(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1411051473, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


    let result = pi_lib::sinfo::StructInfo::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3099464219);


    Some(CallResult::Ok)
}

fn drop_1411051473(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };
}

fn drop_1706731228(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_lib::guid::GuidGen) };
}

fn drop_3099464219(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_lib::sinfo::StructInfo) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::atom::Atom"), drop_fn: drop_1411051473}, 1411051473);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::guid::GuidGen"), drop_fn: drop_1706731228}, 1706731228);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::sinfo::StructInfo"), drop_fn: drop_3099464219}, 3099464219);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1549520222), 1549520222);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1469354144), 1469354144);
    mgr.regist_fun_meta(FnMeta::CallArg(call_748243846), 748243846);
}