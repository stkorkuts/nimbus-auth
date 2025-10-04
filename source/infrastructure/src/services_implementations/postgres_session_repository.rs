use std::sync::Arc;

use nimbus_auth_application::services::session_repository::{
    SessionRepository, SessionRepositoryWithTransaction, errors::SessionRepositoryError,
};
use nimbus_auth_domain::{entities::session::SomeSession, value_objects::identifier::Identifier};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_future, pin_static_future};
use sqlx::PgConnection;
use ulid::Ulid;

use crate::{
    postgres_db::{PostgresDatabase, PostgresTransaction},
    services_implementations::postgres_session_repository::schema::GetSessionDb,
};

mod queries;
mod schema;

pub struct PostgresSessionRepository {
    database: Arc<PostgresDatabase>,
}

enum SessionRepositoryWithTransactionQueryRequest {
    GetById { id: String },
    Save { session: GetSessionDb },
}

enum SessionRepositoryWithTransactionQueryResponse {
    OptionalSession { session: Option<GetSessionDb> },
    SessionSaved,
}

pub struct PostgresSessionRepositoryWithTransaction {
    transaction: PostgresTransaction<
        SessionRepositoryWithTransactionQueryRequest,
        SessionRepositoryWithTransactionQueryResponse,
        SessionRepositoryError,
    >,
}

impl PostgresSessionRepository {
    pub fn new(database: Arc<PostgresDatabase>) -> Self {
        Self { database }
    }
}

impl SessionRepository for PostgresSessionRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn SessionRepositoryWithTransaction>, SessionRepositoryError> {
        let db_cloned = self.database.clone();
        pin_static_future(async move {
            let transactional_repo =
                PostgresSessionRepositoryWithTransaction::init(db_cloned).await?;
            Ok(Box::new(transactional_repo) as Box<dyn SessionRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<Option<SomeSession<'static>>, SessionRepositoryError> {
        todo!()
    }

    fn save(&self, session: SomeSession) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }
}

impl PostgresSessionRepositoryWithTransaction {
    pub async fn init(database: Arc<PostgresDatabase>) -> Result<Self, SessionRepositoryError> {
        let transaction = database
            .start_transaction(|conn, req| pin_future(Self::handle_request(conn, req)))
            .await?;
        Ok(Self { transaction })
    }

    async fn handle_request(
        connection: &mut PgConnection,
        request: SessionRepositoryWithTransactionQueryRequest,
    ) -> Result<SessionRepositoryWithTransactionQueryResponse, SessionRepositoryError> {
        match request {
            SessionRepositoryWithTransactionQueryRequest::GetById { id } => todo!(),
            SessionRepositoryWithTransactionQueryRequest::Save { session } => todo!(),
        }
    }
}

impl SessionRepositoryWithTransaction for PostgresSessionRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }

    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn SessionRepositoryWithTransaction>,
            Option<SomeSession<'static>>,
        ),
        SessionRepositoryError,
    > {
        todo!()
    }

    fn save(
        self: Box<Self>,
        session: SomeSession,
    ) -> StaticPinnedFuture<(Box<dyn SessionRepositoryWithTransaction>, ()), SessionRepositoryError>
    {
        todo!()
    }
}
