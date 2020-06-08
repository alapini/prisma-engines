use super::{
    database_check::DatabaseCheck, sql_migration_warning::SqlMigrationWarning,
    sql_unexecutable_migration::SqlUnexecutableMigration,
};
use crate::SqlResult;
use migration_connector::DestructiveChangeDiagnostics;

///
pub(super) struct DestructiveCheckPlan {
    warnings: Vec<SqlMigrationWarning>,
    unexecutable_migrations: Vec<SqlUnexecutableMigration>,
}

impl DestructiveCheckPlan {
    pub(super) fn new() -> Self {
        DestructiveCheckPlan {
            warnings: Vec::new(),
            unexecutable_migrations: Vec::new(),
        }
    }

    pub(super) fn push_warning(&mut self, warning: SqlMigrationWarning) {
        self.warnings.push(warning)
    }

    pub(super) fn push_unexecutable(&mut self, unexecutable_migration: SqlUnexecutableMigration) {
        self.unexecutable_migrations.push(unexecutable_migration)
    }

    pub(super) async fn execute(&self) -> SqlResult<DestructiveChangeDiagnostics> {
        todo!()
    }
}
