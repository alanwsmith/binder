use security_framework::passwords::get_generic_password;

pub fn pw(name: &str, account: &str) -> String {
    let load_thing = get_generic_password(name, account);
    String::from_utf8(load_thing.unwrap()).unwrap()
}
