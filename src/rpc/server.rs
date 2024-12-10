// File: src/rpc/server.rs

use crate::api::{governance_api::GovernanceAPI, bridge::BridgeAPI};
use warp::Filter;

pub struct RpcServer {
    governance_api: GovernanceAPI,
    bridge_api: BridgeAPI,
}

impl RpcServer {
    pub fn new(governance_api: GovernanceAPI, bridge_api: BridgeAPI) -> Self {
        Self {
            governance_api,
            bridge_api,
        }
    }

    pub async fn start(self) {
        let governance_routes = self.governance_api.routes();
        let bridge_routes = self.bridge_api.routes();

        let routes = governance_routes.or(bridge_routes);

        println!("RPC server running on http://localhost:3030");
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    }
}
