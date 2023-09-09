use auth_rust::AuthCredentials;

fn main() {
    let credentials = AuthCredentials {
        username: "Daniel".to_owned(),
        password: "Babalola".to_owned(),
    };
    auth_rust::authenticate(credentials)
}
