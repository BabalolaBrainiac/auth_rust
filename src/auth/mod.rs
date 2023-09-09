pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}

pub enum Status {
    Connected,
    Terminated,
}

pub fn login(creds: AuthCredentials) {
    if creds.username == String::from("Daniel") && creds.password == String::from("Babalola") {
        let connected = Status::Connected;
        get_user()
    } else {
        println!("Could not login user");
    }
}

pub fn get_user() {
    print!("User has been logged in and is connected")
}
