use std::collections::HashMap;

/// Checks that depend on the current state of the database and serve to qualify destructive change
/// warnings and errors.
///
/// For example, dropping a table that has 0 rows can be considered safe.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum DatabaseCheck {
    TableNotEmpty { table: String },
    ColumnHasValues { table: String, column: String },
    NoCheck,
}

/// The information about the current state of the database gathered by the destructive change checker.
pub(super) struct DatabaseCheckResults {
    /// HashMap from table name to row count.
    row_counts: HashMap<String, i64>,
    /// HashMap from (table name, column name) to non-null values count.
    value_counts: HashMap<(String, String), i64>,
}

impl DatabaseCheckResults {
    pub(crate) fn rows_in_table(&self, table: &str) -> Option<i64> {
        self.row_counts.get(table).map(|count| *count)
    }

    /// Returns the row count in the table and the non-null value count in the column.
    pub(crate) fn values_in_column(&self, table: &str, column: &str) -> (Option<i64>, Option<i64>) {
        // TODO: avoid cloning
        (
            self.row_counts.get(table).map(|count| *count),
            self.value_counts
                .get(&(table.to_owned(), column.to_owned()))
                .map(|count| *count),
        )
    }
}

/// This trait should be implemented by warning and unexecutable migration types. It lets them
/// describe what data they need from the current state of the database to be as accurate and informative as possible.
pub(super) trait Check {
    /// Fetch the row count for the table with the returned name.
    fn check_row_count(&self) -> Option<&str> {
        None
    }

    /// Fetch the number of non-null values for the returned table and column.
    fn check_existing_values(&self) -> Option<(&str, &str)> {
        None
    }

    /// This function will always be called for every check in a migration. Each change must check
    /// for the data it needs in the database check results. If there is no data, it should assume
    /// the current state of the database could not be checked and warn with a best effort message
    /// explaining under which conditions the migration could not be applied or would cause data
    /// loss.
    ///
    /// The only case where `None` should be returned is when there is data about the current state
    /// of the database, and that data indicates that the migration would be executable and safe.
    fn render(&self, database_check_results: &DatabaseCheckResults) -> Option<String>;
}
