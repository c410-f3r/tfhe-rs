use crate::high_level_api::internal_traits::ParameterType;

/// Meant to be implemented on the inner server key
/// eg the crate::integer::ServerKey
pub trait EvaluationIntegerKey<ClientKey> {
    fn new(client_key: &ClientKey) -> Self;

    fn new_wopbs_key(
        client_key: &ClientKey,
        server_key: &Self,
        wopbs_block_parameters: crate::shortint::Parameters,
    ) -> crate::integer::wopbs::WopbsKey;
}

/// Trait to mark parameters type for integers
pub trait IntegerParameter: ParameterType {
    fn num_blocks() -> usize;
}
