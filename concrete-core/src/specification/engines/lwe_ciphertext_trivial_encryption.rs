use super::engine_error;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweSize;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::{LweCiphertextEntity, PlaintextEntity};

engine_error! {
    LweCiphertextTrivialEncryptionError for LweCiphertextTrivialEncryptionEngine @
}

pub trait LweCiphertextTrivialEncryptionEngine<Plaintext, Ciphertext>:
    AbstractEngine
where
    Plaintext: PlaintextEntity,
    Ciphertext: LweCiphertextEntity,
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext,
        noise: Variance,
    ) -> Result<Ciphertext, LweCiphertextTrivialEncryptionError<Self::EngineError>>;

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext,
        noise: Variance,
    ) -> Ciphertext;
}
