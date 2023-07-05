```rust
use llm_chain::LlmChain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LlmChainState {
    pub id: Uuid,
    pub state: String,
}

impl LlmChainState {
    pub fn new(state: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            state,
        }
    }
}

pub fn run_llm_chain(state: &str) -> Result<LlmChainState, &'static str> {
    let llm_chain = LlmChain::new(state);

    match llm_chain.run() {
        Ok(result) => Ok(LlmChainState::new(result)),
        Err(_) => Err("LlmChainError"),
    }
}
```