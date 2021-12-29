use crate::backends::core::private::crypto::lwe::LweList;
use crate::prelude::{
    CoreEngine, LweCiphertextVector32, LweCiphertextVector64,
    LweCiphertextVectorTrivialEncryptionEngine, LweCiphertextVectorTrivialEncryptionError,
    PlaintextVector32, PlaintextVector64,
};
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweSize;

impl LweCiphertextVectorTrivialEncryptionEngine<PlaintextVector32, LweCiphertextVector32>
    for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext_vector(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector32,
        noise: Variance,
    ) -> Result<LweCiphertextVector32, LweCiphertextVectorTrivialEncryptionError<Self::EngineError>>
    {
        unsafe {
            Ok(self.trivially_encrypt_lwe_ciphertext_vector_unchecked(lwe_size, input, noise))
        }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_vector_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector32,
        noise: Variance,
    ) -> LweCiphertextVector32 {
        let ciphertexts = LweList::new_trivial_encryption(
            lwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );

        LweCiphertextVector32(ciphertexts)
    }
}

impl LweCiphertextVectorTrivialEncryptionEngine<PlaintextVector64, LweCiphertextVector64>
    for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext_vector(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector64,
        noise: Variance,
    ) -> Result<LweCiphertextVector64, LweCiphertextVectorTrivialEncryptionError<Self::EngineError>>
    {
        unsafe {
            Ok(self.trivially_encrypt_lwe_ciphertext_vector_unchecked(lwe_size, input, noise))
        }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_vector_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &PlaintextVector64,
        noise: Variance,
    ) -> LweCiphertextVector64 {
        let ciphertexts = LweList::new_trivial_encryption(
            lwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );

        LweCiphertextVector64(ciphertexts)
    }
}
