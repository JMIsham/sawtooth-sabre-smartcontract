extern crate hex;
extern crate crypto;
extern crate sabre_sdk;

use self::sabre_sdk::TransactionContext;
use self::sabre_sdk::ApplyError;
use self::sabre_sdk::TransactionHandler;
use self::sabre_sdk::TpProcessRequest;
use self::sabre_sdk::{WasmPtr, execute_entrypoint};

use self::crypto::sha2::Sha512;
use handler::crypto::digest::Digest;
use std::collections::BTreeMap;
use std::collections::HashMap;

use self::hex::{decode, encode_upper};

   
const INTKEY_PREFIX: &'static str = "1cf126";
const MAX_VALUE: u32 = 4294967295;
const MAX_NAME_LEN: usize = 20;


fn encode_intkey(map: BTreeMap<String, u32>) -> Result<String, ApplyError> {
    // First two characters should be A followed by the number of elements.
    // Only check for A as this will be a map with 15 or less elements
    // It is unlikely that an address will have that many hash collisions
    let mut hex_string = "A".to_string();
    let map_length = map.len() as u32;
    hex_string = hex_string + &format!("{:X}", map_length);

    let keys: Vec<_> = map.keys().cloned().collect();
    for key in keys {
        // Keys need to have a length between 1 and 20
        let key_length = key.len();
        if key_length < 1 || key_length > 20 {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Key must be at least 1 character and no more than 20",
            )));
        }

        // 96 is equal to 60 hex and is the starting byte for strings.
        let length = 96 + key_length;

        // If value is less then 23, the hex of that number is used as the value.
        // If the value is more then 23 the first two bytes start at hex 18 and increment
        // for more bytes. 18 = 2, 19 = 4, 1A = 8. Should not exeed 8 bytes.
        let encoded_key = encode_upper(key.clone());
        let raw_value = map
            .get(&key)
            .ok_or_else(|| ApplyError::InvalidTransaction("Value from map".into()))?;
        if raw_value > &(23 as u32) {
            let mut value = format!("{:02X}", raw_value);
            if value.len() % 2 == 1 {
                value = "0".to_string() + &value.clone();
            }

            let value_length = match value.len() {
                2 => "18",
                4 => "19",
                8 => "1A",
                _ => {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Value is too large",
                    )));
                }
            };
            hex_string =
                hex_string + &format!("{:X}", length) + &encoded_key + &value_length + &value;
        } else {
            hex_string = hex_string
                + &format!("{:X}", length)
                + &encoded_key
                + &format!("{:02X}", raw_value);
        }
    }
    Ok(hex_string)
}

struct HelloWorldPayload {
    //payload fof key value pair
    key: String,
    val: u32,
}

impl HelloWorldPayload {
    pub fn new(payloadData: &[u8]) -> Result <Option<HelloWorldPayload>, ApplyError> {

        let payload = String::from_utf8(payloadData.to_vec())
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        let payload_vec = payload.split(",").collect::<Vec<&str>>();

        let key_raw: String = match payload_vec.get(0) {
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "key should be a string",
                )));
            }
            Some(key_raw) => key_raw.clone().into(),
        };

        if key_raw.len() > MAX_NAME_LEN {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Key be equal to or less than 20 characters",
            )));
        }

        let val_raw: String = match payload_vec.get(1) {
            None => {
                 return Err(ApplyError::InvalidTransaction(String::from(
                    "value should be an integer",
                )));
            }
            Some(val_raw) => val_raw.clone().into(),
        };
        let hello_world_payload = HelloWorldPayload {
            key: key_raw,
            val: val_raw.parse().unwrap(),
        };
        Ok(Some(hello_world_payload))
        
    }
    pub fn get_key(&self) -> &String {
            &self.key
    }

    pub fn get_val(&self) -> u32 {
            self.val
    }

}

pub struct IntkeyState<'a> {
    context: &'a mut TransactionContext,
    get_cache: HashMap<String, BTreeMap<String, u32>>,
}

impl<'a> IntkeyState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> IntkeyState {
        IntkeyState {
            context: context,
            get_cache: HashMap::new(),
        }
    }

    fn calculate_address(name: &str) -> String {
        let mut sha = Sha512::new();
        sha.input(name.as_bytes());
        INTKEY_PREFIX.to_string() + &sha.result_str()[64..].to_string()
    }

    pub fn set(&mut self, name: &str, value: u32) -> Result<(), ApplyError> {
        let mut map: BTreeMap<String, u32> = match self
            .get_cache
            .get_mut(&IntkeyState::calculate_address(name))
        {
            Some(m) => m.clone(),
            None => BTreeMap::new(),
        };
        map.insert(name.into(), value);

        let encoded = encode_intkey(map)?;
        let packed =
            decode(encoded).map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;

        self.context
            .set_state_entry(IntkeyState::calculate_address(name), packed)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        Ok(())
    }
}

pub struct HelloWolrdTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl HelloWolrdTransactionHandler {
    pub fn new() -> HelloWolrdTransactionHandler {
        HelloWolrdTransactionHandler {
            family_name: "hellow_world".to_string(),
            family_versions: vec!["1.0".to_string()],
            namespaces: vec![INTKEY_PREFIX.to_string()],
        }
    }
}

impl TransactionHandler for HelloWolrdTransactionHandler {
    fn family_name(&self) -> String {
        return self.family_name.clone();
    }

    fn family_versions(&self) -> Vec<String> {
        return self.family_versions.clone();
    }

    fn namespaces(&self) -> Vec<String> {
        return self.namespaces.clone();
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        let payload = HelloWorldPayload::new(request.get_payload());
        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };
        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )));
            }
        };
        let mut state = IntkeyState::new(context);
        state.set(&payload.get_key(), payload.get_val())
    }
}

// Sabre apply must return a bool
fn apply(
    request: &TpProcessRequest,
    context: &mut dyn TransactionContext,
) -> Result<bool, ApplyError> {
   let handler = HelloWolrdTransactionHandler::new();
   match handler.apply(request, context) {
       Ok(_) => Ok(true),
       Err(e) => Err(e),
        
   }
}
#[no_mangle]
pub unsafe fn entrypoint(payload: WasmPtr, signer: WasmPtr, signature: WasmPtr) -> i32 {
    execute_entrypoint(payload, signer, signature, apply)
}
