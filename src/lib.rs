use std::convert::{TryFrom, TryInto};
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;


struct SocketHandle(*mut c_void);

impl SocketHandle {

    pub unsafe fn as_socket(&self) -> &'static mut SmartSocket {
        let ptr = self.0 as *mut SmartSocket;
        ptr.as_mut().unwrap()
    }

    pub unsafe fn into_socket(self) -> Box<SmartSocket> {
        let ptr = self.0 as *mut SmartSocket;
        Box::from_raw(ptr)
    }

    pub fn from_socket(socket: SmartSocket) -> Self {
        let reference = Box::leak(Box::new(socket));
        let ptr = reference as *mut SmartSocket;
        Self(ptr as _)
    }
}

type CreateSocketFn = unsafe extern "C" fn() -> SocketHandle;
type TurnOnFn = unsafe extern "C" fn();
type TurnOffFn = unsafe extern "C" fn();
type GetStateFn = unsafe extern "C" fn() -> State<'static>;
type DestroySocketFn = unsafe extern "C" fn(SocketHandle);


pub struct FunctionsBlock {
    create_socket: CreateSocketFn,
    turn_on: TurnOnFn,
    turn_off: TurnOffFn,
    get_state: GetStateFn,
    destroy_socket: DestroySocketFn,
}


impl Default for FunctionsBlock {
    fn default() -> Self {
        Self { 
            create_socket: (), 
            turn_on: (), 
            turn_off: (), 
            get_state: (), 
            destroy_socket: () 
        }
    }
}

#[no_mangle]
pub extern "C" fn functions() -> FunctionsBlock {
    FunctionsBlock::default()
}


unsafe extern "C" fn create_socket() -> SocketHandle {
    
}


struct SmartSocket {
    state: String,
    power: f32,
}

#[repr(C)]
struct State<'a > {
    state: &'a str,
    power: f32,
}


impl SmartSocket {
    fn new() -> Self {
        SmartSocket { state: String::from("Off"), power: 0.0 }
    }

    fn turn_on(&mut self) {
        self.state = String::from("On");
        self.power = 100.0;
    }

    fn turn_off(&mut self) {
        self.state = String::from("Off");
        self.power = 0.0;
    }

    fn get_state(&mut self) -> State {
        State { state: &self.state, power: self.power }
    }
}
