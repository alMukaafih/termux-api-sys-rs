#![allow(non_snake_case)]

extern "C" {
    pub fn exec_am_broadcast(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn contact_plugin(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn exec_callback(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn generate_uuid(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn transmit_stdin_to_socket(
        arg1: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn transmit_socket_to_stdout(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn run_api_command(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}