use gethostname::gethostname;

pub fn get_hostname() -> String {
    gethostname().to_string_lossy().into()
}
