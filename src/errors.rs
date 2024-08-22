#[derive(Debug)]
pub enum SqlBuilderError {
    EmptyTableName,
    EmptyColumnName,
    EmptyColumnAndValue,
    EmptyValue,
    NoColumnsSpecified,
    InvalidColumnType,
    InvalidQuery,
    EmptyCondition,
}

impl core::fmt::Display for SqlBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SqlBuilderError::EmptyTableName => write!(f, "Table name cannot be empty."),
            SqlBuilderError::EmptyColumnName => write!(f, "Column name cannot be empty."),
            SqlBuilderError::EmptyColumnAndValue => write!(
                f,
                "The column and the value to be inserted cannot be empty."
            ),
            SqlBuilderError::EmptyValue => write!(f, "The value cannot be empty."),
            SqlBuilderError::NoColumnsSpecified => write!(f, "No columns specified for table."),
            SqlBuilderError::InvalidColumnType => {
                write!(f, "The specified column type is invalid.")
            }
            SqlBuilderError::InvalidQuery => write!(f, "The query is invalid."),
            SqlBuilderError::EmptyCondition => write!(f, "The conditions cannot be empty."),
        }
    }
}

impl std::error::Error for SqlBuilderError {}
