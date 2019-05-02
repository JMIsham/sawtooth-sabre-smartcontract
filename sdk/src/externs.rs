// Copyright 2018 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub type WasmPtr = i32;
pub type WasmPtrList = i32;

#[no_mangle]
extern "C" {
    pub fn get_state(addresses: WasmPtrList) -> WasmPtrList;
    pub fn set_state(addr_data: WasmPtrList) -> i32;
    pub fn delete_state(addresses: WasmPtrList) -> WasmPtrList;
    pub fn get_ptr_len(ptr: WasmPtr) -> isize;
    pub fn alloc(len: usize) -> WasmPtr;
    pub fn read_byte(offset: isize) -> u8;
    pub fn write_byte(ptr: WasmPtr, offset: u32, byte: u8) -> i32;
    pub fn get_ptr_collection_len(ptr: WasmPtrList) -> isize;
    pub fn get_ptr_from_collection(ptr: WasmPtrList, index: u32) -> WasmPtr;
    pub fn add_to_collection(head: WasmPtr, ptr: WasmPtr) -> WasmPtr;
    pub fn create_collection(head: WasmPtr) -> WasmPtr;
    pub fn invoke_smart_permission(
        contract_addr: WasmPtr,
        name: WasmPtr,
        roles: WasmPtrList,
        org_id: WasmPtr,
        public_key: WasmPtr,
        payload: WasmPtr,
    ) -> i32;
}
