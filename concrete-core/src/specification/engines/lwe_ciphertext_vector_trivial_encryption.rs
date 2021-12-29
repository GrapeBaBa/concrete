use super::engine_error;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweSize;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::{
    LweCiphertextVectorEntity, PlaintextVectorEntity,
};

engine_error! {
    LweCiphertextVectorTrivialEncryptionError for LweCiphertextVectorTrivialEncryptionEngine @
}

pub trait LweCiphertextVectorTrivialEncryptionEngine<PlaintextVector, CiphertextVector>:
    AbstractEngine
where
    PlaintextVector: PlaintextVectorEntity,
    CiphertextVector: LweCiphertextVectorEntity,
{
    fn trivially_encrypt_lwe_ciphertext_vector(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Result<CiphertextVector, LweCiphertextVectorTrivialEncryptionError<Self::EngineError>>;

    unsafe fn trivially_encrypt_lwe_ciphertext_vector_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> CiphertextVector;
}
