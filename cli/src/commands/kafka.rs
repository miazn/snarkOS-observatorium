// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use clap::Parser;
use anyhow::Result;
mod snarkvm_integration {
    use snarkvm::prelude::store::helpers::kafka::config::KAFKA_PRODUCER;
    use snarkvm::prelude::store::helpers::kafka::KafkaProducerTrait;

    pub fn call_emit_event_from_snarkvm(event_data: &std::option::Option<std::string::String>, topic: &std::option::Option<std::string::String>) {
        // Call the emit_event function from snarkVM
        KAFKA_PRODUCER.emit_event(event_data.as_ref().unwrap().as_str(), "test", topic.as_ref().unwrap().as_str());
    }
}

#[derive(Clone,Debug, Parser)]
pub struct Kafka {
    // Add any command-specific options here if needed
    #[clap(long = "message")]
    pub message: Option<String>,

    #[clap(long = "topic")]
    pub topic: Option<String>,
}

impl Kafka {
    pub fn execute(&self) -> Result<String> {
        // Call the emit_event function from snarkvm_integration
        snarkvm_integration::call_emit_event_from_snarkvm(&self.message, &self.topic);
        println!("Event emitted!");
        Ok(String::new())
    }
    
}
