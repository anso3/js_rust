use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use js_sandbox::{Script};

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
struct Exercise {
    function: String,
    func_name: String,
    data: String,

}

#[tokio::main]
async fn main () -> Result<(), LambdaError> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}


async fn handler(e: Exercise, _: Context, ) -> Result<Value, LambdaError>{
    // let message = format!("The function is named {}",&e.func_name);
    let js_code = format!("const ds = {}
    function get(){{ return ds }}
    {}",&e.data, &e.function);
	let mut script = Script::from_string(&js_code).expect("Failed comp");
    let ds: js_sandbox::JsValue = script.call("get",&()).expect("Calling function");
	let result: js_sandbox::JsValue = script.call(&e.func_name, &ds).expect("Not a single int");
    Ok(json!(result))
}