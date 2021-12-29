use crate::prelude::{
    CoreEngine, LweCiphertext32, LweCiphertext64, LweSecretKey32, LweSecretKey64,
    LweSecretKeyEntity, Plaintext32, Plaintext64,
};
use crate::specification::engines::{
    LweCiphertextTrivialEncryptionEngine, LweCiphertextTrivialEncryptionError,
};
use concrete_commons::dispersion::Variance;

use crate::backends::core::private::crypto::lwe::LweCiphertext as ImplLweCiphertext;

impl LweCiphertextTrivialEncryptionEngine<LweSecretKey32, Plaintext32, LweCiphertext32>
    for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        key: &LweSecretKey32,
        input: &Plaintext32,
        noise: Variance,
    ) -> Result<LweCiphertext32, LweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_lwe_ciphertext_unchecked(key, input, noise)) }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        key: &LweSecretKey32,
        input: &Plaintext32,
        noise: Variance,
    ) -> LweCiphertext32 {
        let ciphertext = ImplLweCiphertext::new_trivial_encryption(
            key.lwe_dimension().to_lwe_size(),
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        LweCiphertext32(ciphertext)
    }
}

impl LweCiphertextTrivialEncryptionEngine<LweSecretKey64, Plaintext64, LweCiphertext64>
    for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        key: &LweSecretKey64,
        input: &Plaintext64,
        noise: Variance,
    ) -> Result<LweCiphertext64, LweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_lwe_ciphertext_unchecked(key, input, noise)) }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        key: &LweSecretKey64,
        input: &Plaintext64,
        noise: Variance,
    ) -> LweCiphertext64 {
        let ciphertext = ImplLweCiphertext::new_trivial_encryption(
            key.lwe_dimension().to_lwe_size(),
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        LweCiphertext64(ciphertext)
    }
}
