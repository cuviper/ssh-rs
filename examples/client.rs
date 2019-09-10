use libssh_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};

fn main() -> Result<(), String> {
    unsafe {
        let verbosity = SSH_LOG_PROTOCOL;
        let host = CString::new("localhost").unwrap();
        let port: c_int = 22;

        let my_ssh_session = ssh_new();
        if my_ssh_session.is_null() {
            return Err("creating a session".to_string());
        }

        ssh_options_set(
            my_ssh_session,
            SSH_OPTIONS_LOG_VERBOSITY,
            &verbosity as *const c_int as *const c_void,
        );
        ssh_options_set(
            my_ssh_session,
            SSH_OPTIONS_HOST,
            host.as_ptr() as *const c_void,
        );
        ssh_options_set(
            my_ssh_session,
            SSH_OPTIONS_PORT,
            &port as *const c_int as *const c_void,
        );

        if ssh_connect(my_ssh_session) != SSH_OK {
            let error = CStr::from_ptr(ssh_get_error(my_ssh_session as *mut c_void));
            return Err(format!("connecting to localhost: {:?}", error));
        }

        // TODO
        // - authenticate the server
        // - authenticate the user
        // - do something

        ssh_disconnect(my_ssh_session);
        ssh_free(my_ssh_session);
    }
    Ok(())
}
