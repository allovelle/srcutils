use std::fmt;

impl<T: fmt::Debug> ToDebug for T {}

pub trait ToDebug: fmt::Debug
{
    /// Equivalent to `format!("{:?}", thing);`
    fn to_debug(&self) -> String
    {
        format!("{self:?}")
    }

    /// Equivalent to `format!("{:#?}", thing);`
    fn to_long_debug(&self) -> String
    {
        format!("{self:#?}")
    }

    /// A debug view of a debug view (includes the outer quotes)
    fn to_debug_literal(&self) -> String
    {
        format!("{:?}", format!("{}", self.to_debug()))
    }

    /// Standard format does not allow for width & alignment formatting.
    fn to_debug_left(&self, space: usize) -> String
    {
        format!("{:<space$}", format!("{self:?}"))
    }

    /// Standard format does not allow for width & alignment formatting.
    fn to_debug_right(&self, space: usize) -> String
    {
        format!("{:>space$}", format!("{self:?}"))
    }

    /// Standard format does not allow for width & alignment formatting.
    fn to_debug_center(&self, space: usize) -> String
    {
        format!("{:^space$}", format!("{self:?}"))
    }
}
