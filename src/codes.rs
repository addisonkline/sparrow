pub enum ReturnCodeType {
    // a successful operation is always return code 0
    Success = 0,
    // client-side errors have negative return codes
    InvalidRequest = -1,
    Unauthorized = -2,
    AlreadyConnected = -3,
    NotConnected = -4,
    // server-side errors have positive return codes
    ServerError = 1,
}