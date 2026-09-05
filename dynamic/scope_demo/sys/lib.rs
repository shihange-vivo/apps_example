// Copyright (c) 2026 vivo Mobile Communication Co., Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! C31-a scope corpus (§17.2): the second system DSO. The application root
//! defines a same-named `sys_target = 42`, but this image's own relocation
//! must bind its own definition (`sys_target = 777`) through the system
//! scope — an application symbol must never interpose a system DSO's own
//! relocation (§8.1 non-interpose).

#![no_std]

#[no_mangle]
pub static sys_target: i32 = 777;

#[no_mangle]
pub extern "C" fn sys_report() -> i32 {
    unsafe { core::ptr::read_volatile(&sys_target) }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
