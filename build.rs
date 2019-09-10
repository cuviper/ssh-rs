fn main() {
    // Dynamic libssh doesn't need initialization since 0.8,
    // so we require that to keep things easy.
    pkg_config::Config::new()
        .atleast_version("0.8")
        .statik(false)
        .probe("libssh")
        .expect("dynamically linked libssh >= 0.8 is required");
}
