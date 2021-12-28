use crate::prelude::AbstractEngine;
use super::engine_error;

use crate::specification::entities::{GlweSecretKeyEntity, LweSecretKeyEntity};


engine_error! {
    GlweToLweSecretKeyTransmutationEngineError for GlweToLweSecretKeyConversionEngine @
}

/// A trait for engines transmuting GLWE secret keys into LWE secret keys.
///
/// # Semantics
///
/// This [pure](super#operation-semantics) operation moves the existing GLWE into a fresh LWE secret key.
///
/// # Formal Definition
pub trait GlweToLweSecretKeyTransmutationEngine<InputKey, OutputKey>: AbstractEngine
    where InputKey: GlweSecretKeyEntity,
          OutputKey: LweSecretKeyEntity<KeyDistribution=InputKey::KeyDistribution>
{
    /// Does the transmutation of the GLWE secret key into an LWE secret Key
    fn transmute_glwe_secret_key_to_lwe_secret_key(
        &mut self, glwe_secret_key: InputKey,
    ) -> Result<OutputKey, GlweToLweSecretKeyTransmutationEngineError<Self::EngineError>>;

    unsafe fn transmute_glwe_secret_key_to_lwe_secret_key_unchecked(
        &mut self, glwe_secret_key: InputKey,
    ) -> OutputKey;
}