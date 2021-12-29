use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::GlweSize;

use crate::backends::core::entities::{
    GlweCiphertext32, GlweCiphertext64, PlaintextVector32, PlaintextVector64,
};
use crate::backends::core::private::crypto::glwe::GlweCiphertext as ImplGlweCiphertext;
use crate::specification::engines::{
    GlweCiphertextTrivialEncryptionEngine, GlweCiphertextTrivialEncryptionError,
};

use crate::backends::core::engines::CoreEngine;

impl GlweCiphertextTrivialEncryptionEngine<PlaintextVector32, GlweCiphertext32> for CoreEngine {
    fn trivially_encrypt_glwe_ciphertext(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector32,
        noise: Variance,
    ) -> Result<GlweCiphertext32, GlweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_glwe_ciphertext_unchecked(glwe_size, input, noise)) }
    }

    unsafe fn trivially_encrypt_glwe_ciphertext_unchecked(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector32,
        noise: Variance,
    ) -> GlweCiphertext32 {
        let ciphertext: ImplGlweCiphertext<Vec<u32>> = ImplGlweCiphertext::new_trivial_encryption(
            glwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        GlweCiphertext32(ciphertext)
    }
}

impl GlweCiphertextTrivialEncryptionEngine<PlaintextVector64, GlweCiphertext64> for CoreEngine {
    fn trivially_encrypt_glwe_ciphertext(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector64,
        noise: Variance,
    ) -> Result<GlweCiphertext64, GlweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_glwe_ciphertext_unchecked(glwe_size, input, noise)) }
    }

    unsafe fn trivially_encrypt_glwe_ciphertext_unchecked(
        &mut self,
        glwe_size: GlweSize,
        input: &PlaintextVector64,
        noise: Variance,
    ) -> GlweCiphertext64 {
        let ciphertext: ImplGlweCiphertext<Vec<u64>> = ImplGlweCiphertext::new_trivial_encryption(
            glwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        GlweCiphertext64(ciphertext)
    }
}
