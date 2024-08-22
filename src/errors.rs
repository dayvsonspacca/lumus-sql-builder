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
    EmptyOnClause,
}

impl core::fmt::Display for SqlBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyTableName => write!(f, "Table name cannot be empty."),
            Self::EmptyColumnName => write!(f, "Column name cannot be empty."),
            Self::EmptyColumnAndValue => write!(
                f,
                "The column and the value to be inserted cannot be empty."
            ),
            Self::EmptyValue => write!(f, "The value cannot be empty."),
            Self::NoColumnsSpecified => write!(f, "No columns specified for table."),
            Self::InvalidColumnType => {
                write!(f, "The specified column type is invalid.")
            }
            Self::InvalidQuery => write!(f, "The query is invalid."),
            Self::EmptyCondition => write!(f, "The conditions cannot be empty."),
            Self::EmptyOnClause => write!(f, "The on clause cannot be empty."),
        }
    }
}

impl std::error::Error for SqlBuilderError {}
