use js_sys::{Function, JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@actions/core")]
extern {
    #[wasm_bindgen(js_name = "issue")]
    pub fn issue(name: &JsString, message: Option<&JsString>);

    #[wasm_bindgen(js_name = "issueCommand")]
    pub fn issue_command(command: &JsString, properties: &Object, message: &JsString);
}

/// Interface for getInput options
#[wasm_bindgen]
pub struct InputOptions {
    /// Whether the input is required. If required and not present, will throw. Defaults
    /// to false.
    pub required: Option<bool>,
}

/// The code to exit an action
#[wasm_bindgen]
pub enum ExitCode {
    /// A code indicating that the action was successful.
    Success = 0,
    /// A code indicating that the action was a failure.
    Failure = 1,
}

#[wasm_bindgen(module = "@actions/core")]
extern {
    /// Sets env variable for this action and future actions in the job.
    #[wasm_bindgen(js_name = "exportVariable")]
    pub fn export_variable(name: &JsString, value: &JsString);

    /// Registers a secret which will get masked from logs.
    #[wasm_bindgen(js_name = "setSecret")]
    pub fn set_secret(secret: &JsString);

    /// Prepends inputPath to the PATH (for this action and future actions).
    #[wasm_bindgen(js_name = "addPath")]
    pub fn add_path(path: &JsString);

    /// Gets the value of an input. The value is also trimmed.
    #[wasm_bindgen(js_name = "getInput")]
    pub fn get_input(name: &JsString, options: Option<InputOptions>) -> JsString;

    /// Sets the value of an output.
    #[wasm_bindgen(js_name = "setOutput")]
    pub fn set_output(name: &JsString, value: &JsString);

    /// Sets the action status to failed. When the action exits it will be with an exit code of 1.
    #[wasm_bindgen(js_name = "setFailed")]
    pub fn set_failed(message: &JsString);

    /// Writes debug message to user log.
    #[wasm_bindgen]
    pub fn debug(message: &JsString);

    /// Adds an error issue.
    #[wasm_bindgen]
    pub fn error(message: &JsString);

    /// Adds an warning issue.
    #[wasm_bindgen]
    pub fn warning(message: &JsString);

    /// Writes info to log with console.log.
    #[wasm_bindgen]
    pub fn info(message: &JsString);

    /// Begin an output group.
    #[wasm_bindgen(js_name = "startGroup")]
    pub fn start_group(name: &JsString);

    /// End an output group.
    #[wasm_bindgen(js_name = "endGroup")]
    pub fn end_group();

    /// Wrap an asynchronous function call in a group.
    pub fn group(name: &JsString, fun: &Function) -> Promise;

    // Saves state for current action, the state can only be retrieved by this action's post job
    // execution.
    #[wasm_bindgen(js_name = "saveState")]
    pub fn save_state(name: &JsString, value: &JsString);

    // Gets the value of an state set by this action's main execution.
    #[wasm_bindgen(js_name = "getState")]
    pub fn get_state(name: &JsString) -> JsString;
}
