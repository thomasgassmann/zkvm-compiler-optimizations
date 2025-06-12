#![allow(unused)]

#[link(name = "spec-619", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

static mut IT: i32 = 1;
static mut ACTION: i32 = 0;
static mut SIM_TYPE: i32 = 0;
static mut CALL_COUNT: i32 = 0;

#[no_mangle]
pub fn read_int() -> i32 {
    unsafe {
        let res = match CALL_COUNT {
            0 => IT,
            1 => ACTION,
            2 => SIM_TYPE,
            _ => panic!("read_int called too many times"),
        };
        CALL_COUNT += 1;
        res
    }
}

#[no_mangle]
pub extern "C" fn main_core(it: i32, action: i32, sim_type: i32) -> () {
    unsafe {
        IT = it;
        ACTION = action;
        SIM_TYPE = sim_type;
        CALL_COUNT = 0;
        cmain();
    }
}
