// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
use crate::trusted_worker::EchoWorker;
use crate::trusted_worker::RegisterWorker;
use crate::worker::WorkerInfoQueue;
#[cfg(feature = "mesalock_sgx")]
use std::prelude::v1::*;

pub fn register_trusted_worker_statically() {
    for _i in 0..10 {
        let worker = Box::new(EchoWorker::new());
        let _ = WorkerInfoQueue::register(worker);

        let worker = Box::new(RegisterWorker::new());
        let _ = WorkerInfoQueue::register(worker);
    }
}
