use super::database_check::{Check, DatabaseCheckResults};

pub(super) enum SqlMigrationWarning {
    // format!("You are about to drop the column `{column_name}` on the `{table_name}` table, which still contains {values_count} non-null values.", column_name = todo!(), table_name = todo!(), values_count = todo!())
    NonEmptyColumnDrop { table: String, column: String },
    NonEmptyTableDrop { table: String },
    // "You are about to alter the column `{column_name}` on the `{table_name}` table, which still contains {values_count} non-null values. The data in that column will be lost.",
    AlterColumn { table: String, column: String },
    // "The migration is about to remove a default value on the foreign key field `{}.{}`.",
    ForeignKeyDefaultValueRemoved { table: String, column: String },
}

impl Check for SqlMigrationWarning {
    fn check_row_count(&self) -> Option<&str> {
        None
    }

    fn check_existing_values(&self) -> Option<(&str, &str)> {
        match self {
            SqlMigrationWarning::NonEmptyColumnDrop { table, column }
            | SqlMigrationWarning::AlterColumn { table, column } => Some((table, column)),
            SqlMigrationWarning::ForeignKeyDefaultValueRemoved { .. }
            | SqlMigrationWarning::NonEmptyTableDrop { .. } => None,
        }
    }

    fn render(&self, database_check_results: &DatabaseCheckResults) -> Option<String> {
        match self {
            SqlMigrationWarning::NonEmptyTableDrop { table } => match database_check_results.rows_in_table(table) {
                Some(0) => None, // dropping the table is safe if it's empty
                Some(rows_count) => Some(format!("You are about to drop the table `{table_name}`, which is not empty ({rows_count} rows).", table_name = table, rows_count = rows_count)),
                None => Some(format!("You are about to drop the `{}` table. If the table is not empty, all the data it contains will be lost.", table)),
            },
            SqlMigrationWarning::NonEmptyColumnDrop { table, column } => todo!(),
            SqlMigrationWarning::AlterColumn { table, column } => todo!(),
            SqlMigrationWarning::ForeignKeyDefaultValueRemoved { table, column } => todo!(),
        }
    }
}
