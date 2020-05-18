// Copyright (C) 2017 Felix Obenhuber
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate hexdump;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nix;
#[macro_use]
mod utils;
mod binder;
pub mod errors;
pub mod service;
mod types;
