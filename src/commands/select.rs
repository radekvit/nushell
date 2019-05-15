use crate::errors::ShellError;
use crate::object::base::select;
use crate::object::Value;
use crate::prelude::*;
use derive_new::new;

#[derive(new)]
pub struct SelectBlueprint;

impl crate::CommandBlueprint for SelectBlueprint {
    fn create(
        &self,
        args: Vec<Value>,
        _host: &dyn Host,
        _env: &mut Environment,
    ) -> Result<Box<dyn Command>, ShellError> {
        if args.is_empty() {
            return Err(ShellError::string("take requires an integer"));
        }

        let fields: Result<_, _> = args.iter().map(|a| a.as_string()).collect();

        Ok(Box::new(Select { fields: fields? }))
    }
}

#[derive(new)]
pub struct Select {
    fields: Vec<String>,
}

impl crate::Command for Select {
    fn run(&mut self, stream: VecDeque<Value>) -> Result<VecDeque<ReturnValue>, ShellError> {
        let objects = stream
            .iter()
            .map(|item| Value::Object(select(item, &self.fields)))
            .map(|item| ReturnValue::Value(item))
            .collect();

        Ok(objects)
    }
}