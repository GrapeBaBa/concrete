use super::engine_error;
use crate::prelude::PlaintextVectorEntity;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::GlweSize;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::GlweCiphertextEntity;

engine_error! {
    GlweCiphertextTrivialEncryptionError for GlweCiphertextTrivialEncryptionEngine @
}

/// A trait for engines encrypting LWE ciphertexts.
///
/// # Semantics
///
/// This [pure](super#operation-semantics) operation generates a GLWE ciphertext containing the
/// trivial encryption of the `input` plaintext vector.
///
/// # Formal Definition
pub trait GlweCiphertextTrivialEncryptionEngine<PlaintextVector, Ciphertext>:
    AbstractEngine
where
    PlaintextVector: PlaintextVectorEntity,
    Ciphertext: GlweCiphertextEntity,
{
    /// Creates the trivial GLWE encryption of the plaintext vector.
    ///
    /// The output ciphertext will have:
    /// - It's [glwe_dimension] equal to the requested `glwe_size`
    /// - It's [polynomial_size] equal to the `input` plaintext [plaintext_count]
    ///
    /// [glwe_dimension]: GlweCiphertextEntity::glwe_dimension
    /// [polynomial_size]: GlweCiphertextEntity::polynomial_size
    /// [plaintext_count]: PlaintextVectorEntity::plaintext_count
    fn trivially_encrypt_glwe_ciphertext(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Result<Ciphertext, GlweCiphertextTrivialEncryptionError<Self::EngineError>>;

    /// Unsafely creates the trivial GLWE encryption of the plaintext vector.
    ///
    /// # Safety
    /// For the _general_ safety concerns regarding this operation, refer to the different variants
    /// of [`GlweCiphertextTrivialEncryptionError`]. For safety concerns _specific_ to an engine, refer
    /// to the implementer safety section.
    unsafe fn trivially_encrypt_glwe_ciphertext_unchecked(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector,
        noise: Variance,
    ) -> Ciphertext;
}
