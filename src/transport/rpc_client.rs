use anyhow::Result;
use serde_json::Value;


#[derive(Debug, Clone)]
/// RPC client stub
pub struct RpcClient { 
    transport: Box<dyn Transport> 
}

impl RpcClient {
    pub fn new_with_auth(url: impl Into<String>, user: &str, pass: &str) -> Self {
        Self { 
            transport: Box::new(crate::transport::DefaultTransport::new(
                url,
                Some((user.to_string(), pass.to_string()))
            ))
        }
    }
    
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, TransportError> {
        self.transport.send_request(method, params).await
    }
}
