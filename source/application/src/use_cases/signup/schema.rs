pub struct SignUpRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub e2e_key_hash: &'a str,
    pub encrypted_master_key: &'a str,
}

pub struct SignUpResponse {}
