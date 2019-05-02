use sabre_sdk::ApplyError;
use sabre_sdk::TransactionContext;
use sabre_sdk::TransactionHandler;
use sabre_sdk::TpProcessRequest;
use sabre_sdk::{WasmPtr, execute_entrypoint};
   
       
// Sabre apply must return a bool
fn apply(
    request: &TpProcessRequest,
    context: &mut dyn TransactionContext,
) -> Result<bool, ApplyError> {
   //TO_DO
   Ok(true)
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe fn entrypoint(payload: WasmPtr, signer: WasmPtr, signature: WasmPtr) -> i32 {
    execute_entrypoint(payload, signer, signature, apply)
}
