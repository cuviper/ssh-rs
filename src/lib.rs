#![allow(non_camel_case_types)]

pub use libc::size_t;

use std::os::raw::{c_char, c_int, c_uchar, c_void};

// Error return codes
pub const SSH_OK: c_int = 0;
pub const SSH_ERROR: c_int = -1;
pub const SSH_AGAIN: c_int = -2;
pub const SSH_EOF: c_int = -127;

pub enum ssh_session_struct {}
pub type ssh_session = *mut ssh_session_struct;

pub type ssh_auth_e = c_int;
pub const SSH_AUTH_SUCCESS: ssh_auth_e = 0;
pub const SSH_AUTH_DENIED: ssh_auth_e = 1;
pub const SSH_AUTH_PARTIAL: ssh_auth_e = 2;
pub const SSH_AUTH_INFO: ssh_auth_e = 3;
pub const SSH_AUTH_AGAIN: ssh_auth_e = 4;
pub const SSH_AUTH_ERROR: ssh_auth_e = -1;

pub type ssh_known_hosts_e = c_int;
pub const SSH_KNOWN_HOSTS_ERROR: ssh_known_hosts_e = -2;
pub const SSH_KNOWN_HOSTS_NOT_FOUND: ssh_known_hosts_e = -1;
pub const SSH_KNOWN_HOSTS_UNKNOWN: ssh_known_hosts_e = 0;
pub const SSH_KNOWN_HOSTS_OK: ssh_known_hosts_e = 1;
pub const SSH_KNOWN_HOSTS_CHANGED: ssh_known_hosts_e = 2;
pub const SSH_KNOWN_HOSTS_OTHER: ssh_known_hosts_e = 3;

extern "C" {
    pub fn ssh_new() -> ssh_session;
    pub fn ssh_connect(session: ssh_session) -> c_int;
    pub fn ssh_disconnect(session: ssh_session) -> c_int;
    pub fn ssh_free(session: ssh_session);

    pub fn ssh_get_error(error: *mut c_void) -> *const c_char;
    pub fn ssh_is_blocking(session: ssh_session) -> c_int;
    pub fn ssh_session_has_known_hosts_entry(session: ssh_session) -> ssh_known_hosts_e;
    pub fn ssh_set_blocking(session: ssh_session, blocking: c_int);
    pub fn ssh_userauth_password(
        session: ssh_session,
        username: *const c_char,
        password: *const c_char,
    ) -> c_int;
}

pub type ssh_options_e = c_int;
pub const SSH_OPTIONS_HOST: ssh_options_e = 0;
pub const SSH_OPTIONS_PORT: ssh_options_e = 1;
pub const SSH_OPTIONS_PORT_STR: ssh_options_e = 2;
pub const SSH_OPTIONS_FD: ssh_options_e = 3;
pub const SSH_OPTIONS_USER: ssh_options_e = 4;
pub const SSH_OPTIONS_SSH_DIR: ssh_options_e = 5;
pub const SSH_OPTIONS_IDENTITY: ssh_options_e = 6;
pub const SSH_OPTIONS_ADD_IDENTITY: ssh_options_e = 7;
pub const SSH_OPTIONS_KNOWNHOSTS: ssh_options_e = 8;
pub const SSH_OPTIONS_TIMEOUT: ssh_options_e = 9;
pub const SSH_OPTIONS_TIMEOUT_USEC: ssh_options_e = 10;
pub const SSH_OPTIONS_SSH1: ssh_options_e = 11;
pub const SSH_OPTIONS_SSH2: ssh_options_e = 12;
pub const SSH_OPTIONS_LOG_VERBOSITY: ssh_options_e = 13;
pub const SSH_OPTIONS_LOG_VERBOSITY_STR: ssh_options_e = 14;
pub const SSH_OPTIONS_CIPHERS_C_S: ssh_options_e = 15;
pub const SSH_OPTIONS_CIPHERS_S_C: ssh_options_e = 16;
pub const SSH_OPTIONS_COMPRESSION_C_S: ssh_options_e = 17;
pub const SSH_OPTIONS_COMPRESSION_S_C: ssh_options_e = 18;
pub const SSH_OPTIONS_PROXYCOMMAND: ssh_options_e = 19;
pub const SSH_OPTIONS_BINDADDR: ssh_options_e = 20;
pub const SSH_OPTIONS_STRICTHOSTKEYCHECK: ssh_options_e = 21;
pub const SSH_OPTIONS_COMPRESSION: ssh_options_e = 22;
pub const SSH_OPTIONS_COMPRESSION_LEVEL: ssh_options_e = 23;
pub const SSH_OPTIONS_KEY_EXCHANGE: ssh_options_e = 24;
pub const SSH_OPTIONS_HOSTKEYS: ssh_options_e = 25;
pub const SSH_OPTIONS_GSSAPI_SERVER_IDENTITY: ssh_options_e = 26;
pub const SSH_OPTIONS_GSSAPI_CLIENT_IDENTITY: ssh_options_e = 27;
pub const SSH_OPTIONS_GSSAPI_DELEGATE_CREDENTIALS: ssh_options_e = 28;
pub const SSH_OPTIONS_HMAC_C_S: ssh_options_e = 29;
pub const SSH_OPTIONS_HMAC_S_C: ssh_options_e = 30;
pub const SSH_OPTIONS_PASSWORD_AUTH: ssh_options_e = 31;
pub const SSH_OPTIONS_PUBKEY_AUTH: ssh_options_e = 32;
pub const SSH_OPTIONS_KBDINT_AUTH: ssh_options_e = 33;
pub const SSH_OPTIONS_GSSAPI_AUTH: ssh_options_e = 34;
pub const SSH_OPTIONS_GLOBAL_KNOWNHOSTS: ssh_options_e = 35;
pub const SSH_OPTIONS_NODELAY: ssh_options_e = 36;
pub const SSH_OPTIONS_PUBLICKEY_ACCEPTED_TYPES: ssh_options_e = 37;
pub const SSH_OPTIONS_PROCESS_CONFIG: ssh_options_e = 38;
pub const SSH_OPTIONS_REKEY_DATA: ssh_options_e = 39;
pub const SSH_OPTIONS_REKEY_TIME: ssh_options_e = 40;

pub const SSH_LOG_NOLOG: c_int = 0;
pub const SSH_LOG_WARNING: c_int = 1;
pub const SSH_LOG_PROTOCOL: c_int = 2;
pub const SSH_LOG_PACKET: c_int = 3;
pub const SSH_LOG_FUNCTIONS: c_int = 4;

extern "C" {
    pub fn ssh_options_get(
        session: ssh_session,
        type_: ssh_options_e,
        value: *mut *mut c_char,
    ) -> c_int;
    pub fn ssh_options_set(
        session: ssh_session,
        type_: ssh_options_e,
        value: *const c_void,
    ) -> c_int;
}

pub enum ssh_key_struct {}
pub type ssh_key = *mut ssh_key_struct;

pub type ssh_publickey_hash_type_e = c_int;
pub const SSH_PUBLICKEY_HASH_SHA1: ssh_publickey_hash_type_e = 0;
pub const SSH_PUBLICKEY_HASH_MD5: ssh_publickey_hash_type_e = 1;
pub const SSH_PUBLICKEY_HASH_SHA256: ssh_publickey_hash_type_e = 2;

// Public key related functions
extern "C" {
    pub fn ssh_clean_pubkey_hash(hash: *mut *mut c_uchar);
    pub fn ssh_key_free(key: ssh_key);
    pub fn ssh_get_publickey_hash(
        key: ssh_key,
        type_: ssh_publickey_hash_type_e,
        hash: *mut *mut c_uchar,
        hlen: *mut size_t,
    ) -> c_int;
    pub fn ssh_get_server_publickey(session: ssh_session, key: *mut ssh_key) -> c_int;
}
