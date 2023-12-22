use std::future::Future;

use futures_util::TryFutureExt;
use js_sys::Promise;
use wasm_bindgen::{JsError, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;

pub(crate) fn future_to_promise<F, T>(future: F) -> Promise
where
    F: Future<Output = Result<T, anyhow::Error>> + 'static,
    T: Into<JsValue>,
{
    future_to_promise_with_custom_error(future.map_err(|e| JsError::new(&e.to_string())))
}

pub(crate) fn future_to_promise_with_custom_error<F, T, E>(future: F) -> Promise
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Into<JsValue>,
    E: Into<JsValue>,
{
    let mut future = Some(future);

    Promise::new(&mut |resolve, reject| {
        let future = future.take().unwrap_throw();

        spawn_local(async move {
            match future.await {
                Ok(value) => resolve.call1(&JsValue::UNDEFINED, &value.into()).unwrap_throw(),
                Err(value) => reject.call1(&JsValue::UNDEFINED, &value.into()).unwrap_throw(),
            };
        });
    })
}