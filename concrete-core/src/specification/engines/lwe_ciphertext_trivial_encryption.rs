use super::engine_error;
use concrete_commons::dispersion::Variance;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::{LweCiphertextEntity, LweSecretKeyEntity, PlaintextEntity};

engine_error! {
    LweCiphertextTrivialEncryptionError for LweCiphertextTrivialEncryptionEngine @
}

pub trait LweCiphertextTrivialEncryptionEngine<SecretKey, Plaintext, Ciphertext>:
    AbstractEngine
where
    SecretKey: LweSecretKeyEntity,
    Plaintext: PlaintextEntity,
    Ciphertext: LweCiphertextEntity,
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        key: &SecretKey,
        input: &Plaintext,
        noise: Variance,
    ) -> Result<Ciphertext, LweCiphertextTrivialEncryptionError<Self::EngineError>>;

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        key: &SecretKey,
        input: &Plaintext,
        noise: Variance,
    ) -> Ciphertext;
}
