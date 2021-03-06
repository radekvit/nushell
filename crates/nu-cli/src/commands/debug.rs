use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, UntaggedValue};

pub struct Debug;

#[derive(Deserialize)]
pub struct DebugArgs {
    raw: bool,
}

#[async_trait]
impl WholeStreamCommand for Debug {
    fn name(&self) -> &str {
        "debug"
    }

    fn signature(&self) -> Signature {
        Signature::build("debug").switch("raw", "Prints the raw value representation.", Some('r'))
    }

    fn usage(&self) -> &str {
        "Print the Rust debug representation of the values"
    }

    async fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        debug_value(args, registry).await
    }
}

async fn debug_value(
    args: CommandArgs,
    registry: &CommandRegistry,
) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();
    let (DebugArgs { raw }, input) = args.process(&registry).await?;
    Ok(input
        .map(move |v| {
            if raw {
                ReturnSuccess::value(
                    UntaggedValue::string(format!("{:#?}", v)).into_untagged_value(),
                )
            } else {
                ReturnSuccess::debug_value(v)
            }
        })
        .to_output_stream())
}

#[cfg(test)]
mod tests {
    use super::Debug;

    #[test]
    fn examples_work_as_expected() {
        use crate::examples::test as test_examples;

        test_examples(Debug {})
    }
}
