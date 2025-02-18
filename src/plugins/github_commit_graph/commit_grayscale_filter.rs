use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Runtime, Value, ValueView,
};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "git_commit_grayscale",
    description = "Limits a number to a maximum value.",
    parsed(GitCommitGreyscaleFilter)
)]
pub struct GitCommitGreyscale;

#[derive(Debug, Default, Display_filter)]
#[name = "git_commit_grayscale"]
struct GitCommitGreyscaleFilter;

impl Filter for GitCommitGreyscaleFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> liquid_core::Result<Value> {
        let count = input.as_scalar().and_then(|s| s.to_integer()).unwrap_or(0);
        let shade = match count {
            0 => "bg-white",
            1 => "bg--gray-5",
            2 => "bg--gray-4",
            3 => "bg--gray-3",
            4 => "bg--gray-2",
            5 => "bg--gray-1",
            _ => "bg-black",
        };
        Ok(Value::Scalar(shade.to_string().into()))
    }
}
