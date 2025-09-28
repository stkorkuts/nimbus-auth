#[derive(Clone, Copy)]
pub struct SessionExpirationSeconds(pub usize);

#[derive(Clone, Copy)]
pub struct AccessTokenExpirationSeconds(pub usize);

#[derive(Clone, Copy)]
pub struct PostgresDbMaxConnections(pub usize);
