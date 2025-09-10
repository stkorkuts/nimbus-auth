pub struct SignUpRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
}

pub struct SignUpResponse {}
