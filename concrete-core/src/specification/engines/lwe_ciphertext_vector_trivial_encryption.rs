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

/// A trait for engines creating trivial encryption of LWE ciphertexts
///
/// # Semantics
///
/// This [pure](super#operation-semantics) operation generates a LWE ciphertext
/// containing the trivial encryption of the `input` plaintext with the requested `LweSize`.
pub trait LweCiphertextVectorTrivialEncryptionEngine<PlaintextVector, CiphertextVector>:
    AbstractEngine
where
    PlaintextVector: PlaintextVectorEntity,
    CiphertextVector: LweCiphertextVectorEntity,
{
    /// Creates the trivial LWE encryption of the plaintext vector.
    ///
    /// The output ciphertext will have:
    /// - It's [lwe_dimension] equal to `lwe_size` - 1
    /// - It's [lwe_ciphertext_count] equal to the `input` plaintext [plaintext_count]
    ///
    /// [lwe_dimension]: LweCiphertextVectorEntity::lwe_dimension
    /// [lwe_ciphertext_count]: LweCiphertextVectorEntity::lwe_ciphertext_count
    /// [plaintext_count]: PlaintextVectorEntity::plaintext_count
    fn trivially_encrypt_lwe_ciphertext_vector(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Result<CiphertextVector, LweCiphertextVectorTrivialEncryptionError<Self::EngineError>>;

    /// Unsafely creates the trivial LWE encryption of the plaintext vector.
    ///
    /// # Safety
    /// For the _general_ safety concerns regarding this operation, refer to the different variants
    /// of [`LweCiphertextVectorTrivialEncryptionError`]. For safety concerns _specific_ to an engine, refer
    /// to the implementer safety section.
    unsafe fn trivially_encrypt_lwe_ciphertext_vector_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> CiphertextVector;
}
