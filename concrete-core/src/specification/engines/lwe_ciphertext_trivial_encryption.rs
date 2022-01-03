use super::engine_error;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweSize;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::{LweCiphertextEntity, PlaintextEntity};

engine_error! {
    LweCiphertextTrivialEncryptionError for LweCiphertextTrivialEncryptionEngine @
}

/// A trait for engines creating trivial encryptions of LWE ciphertexts
///
/// # Semantics
///
/// This [pure](super#operation-semantics) operation creates a LWE ciphertext
/// containing the trivial encryption of the plaintext.
pub trait LweCiphertextTrivialEncryptionEngine<Plaintext, Ciphertext>: AbstractEngine
where
    Plaintext: PlaintextEntity,
    Ciphertext: LweCiphertextEntity,
{
    /// Creates the trivial LWE encryption of the plaintext.
    ///
    /// The output ciphertext will have:
    /// - It's [lwe_dimension] equal to `lwe_size - 1`
    ///
    /// [lwe_dimension]: LweCiphertextEntity::lwe_dimension
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext,
        noise: Variance,
    ) -> Result<Ciphertext, LweCiphertextTrivialEncryptionError<Self::EngineError>>;

    /// Unsafely creates the trivial LWE encryption of the plaintext.
    ///
    /// # Safety
    /// For the _general_ safety concerns regarding this operation, refer to the different variants
    /// of [`LweCiphertextTrivialEncryptionError ]. For safety concerns _specific_ to an engine, refer
    /// to the implementer safety section.
    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext,
        noise: Variance,
    ) -> Ciphertext;
}
