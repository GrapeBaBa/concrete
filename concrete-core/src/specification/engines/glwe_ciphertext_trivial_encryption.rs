use super::engine_error;
use crate::prelude::PlaintextVectorEntity;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::GlweSize;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::GlweCiphertextEntity;

engine_error! {
    GlweCiphertextTrivialEncryptionError for GLweCiphertextTrivialEncryptionEngine @
}

pub trait GlweCiphertextTrivialEncryptionEngine<PlaintextVector, Ciphertext>:
    AbstractEngine
where
    PlaintextVector: PlaintextVectorEntity,
    Ciphertext: GlweCiphertextEntity,
{
    fn trivially_encrypt_glwe_ciphertext(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Result<Ciphertext, GlweCiphertextTrivialEncryptionError<Self::EngineError>>;

    unsafe fn trivially_encrypt_glwe_ciphertext_unchecked(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Ciphertext;
}
