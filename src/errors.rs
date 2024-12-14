/// Custom error wrapper for bridge operations
#[derive(Debug, Clone)]
pub struct CustomError(pub String);

impl warp::reject::Reject for CustomError {}
