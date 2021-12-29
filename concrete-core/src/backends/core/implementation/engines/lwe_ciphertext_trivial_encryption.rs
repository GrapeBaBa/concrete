use crate::prelude::{
    CoreEngine, LweCiphertext32, LweCiphertext64, Plaintext32, Plaintext64,
};
use crate::specification::engines::{
    LweCiphertextTrivialEncryptionEngine, LweCiphertextTrivialEncryptionError,
};
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweSize;

use crate::backends::core::private::crypto::lwe::LweCiphertext as ImplLweCiphertext;

impl LweCiphertextTrivialEncryptionEngine<Plaintext32, LweCiphertext32>
    for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext32,
        noise: Variance,
    ) -> Result<LweCiphertext32, LweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_lwe_ciphertext_unchecked(lwe_size, input, noise)) }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext32,
        noise: Variance,
    ) -> LweCiphertext32 {
        let ciphertext = ImplLweCiphertext::new_trivial_encryption(
            lwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        LweCiphertext32(ciphertext)
    }
}


impl LweCiphertextTrivialEncryptionEngine<Plaintext64, LweCiphertext64>
for CoreEngine
{
    fn trivially_encrypt_lwe_ciphertext(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext64,
        noise: Variance,
    ) -> Result<LweCiphertext64, LweCiphertextTrivialEncryptionError<Self::EngineError>> {
        unsafe { Ok(self.trivially_encrypt_lwe_ciphertext_unchecked(lwe_size, input, noise)) }
    }

    unsafe fn trivially_encrypt_lwe_ciphertext_unchecked(
        &mut self,
        lwe_size: LweSize,
        input: &Plaintext64,
        noise: Variance,
    ) -> LweCiphertext64 {
        let ciphertext = ImplLweCiphertext::new_trivial_encryption(
            lwe_size,
            &input.0,
            noise,
            &mut self.encryption_generator,
        );
        LweCiphertext64(ciphertext)
    }
}
