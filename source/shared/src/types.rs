use crate::define_enum;

#[derive(Clone, Copy, Debug)]
pub struct SessionExpirationSeconds(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct AccessTokenExpirationSeconds(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct PostgresDbMaxConnections(pub usize);

define_enum! {
    pub enum UserRole {
        Default,
        Admin,
    }
}
