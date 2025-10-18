#[derive(Clone, Copy, Debug)]
pub struct SessionExpirationSeconds(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct AccessTokenExpirationSeconds(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct PostgresDbMaxConnections(pub usize);

#[derive(Clone, Copy, Debug)]
pub enum UserRole {
    Default,
    Admin,
}
