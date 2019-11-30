pub mod participant1 {
    pub const NAME: &str = "Alice";
    pub const EMAIL: &str = "alice@example.com";
    pub const PHONE_NUMBER: &str = "+380710000000";
    pub const PASS_CODE: &str = "AA000000";
}

pub mod participant2 {
    pub const NAME: &str = "Bob";
    pub const EMAIL: &str = "bob@example.com";
    pub const PHONE_NUMBER: &str = "+380710000001";
    pub const PASS_CODE: &str = "AA000001";
}

pub mod administration1 {
    pub const NAME: &str = "Administration1";
}

pub mod election1 {
    pub const NAME: &str = "Choose your favorite color";
    pub const OPTIONS: &[&str] = &["red", "green", "blue"];
}
