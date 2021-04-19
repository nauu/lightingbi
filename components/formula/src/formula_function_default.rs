use evalexpr::*;
use std::collections::HashMap;
pub struct FormulaFunctionDefault {}

impl FormulaFunctionDefault {
    pub async fn get_fn_context_map() -> HashMapContext {
        let context = context_map! {
            "avg" => Function::new(Box::new(|argument| {
                let arguments = argument.as_tuple()?;

                let arguments = argument.as_tuple().unwrap();
                let length = arguments.len();
                let mut sum:f64 = 0.0;
                for j in 0..length {

                     sum = sum + arguments[j].as_number().unwrap();
                };

               Ok(Value::Float(sum / length as f64))
            }))
        }
        .unwrap();
        context
    }
}
