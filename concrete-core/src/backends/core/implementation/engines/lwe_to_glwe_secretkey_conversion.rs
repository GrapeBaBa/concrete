use crate::specification::engines::LweToGlweSecretKeyConversionEngine;
use concrete_commons::parameters::{GlweDimension, PolynomialSize};

use crate::backends::core::entities::{
    GlweSecretKey32, GlweSecretKey64, LweSecretKey32, LweSecretKey64,
};
use crate::prelude::{
    CoreEngine, GlweToLweSecretKeyTransmutationEngine, LweToGlweSecretKeyConversionError,
};

impl LweToGlweSecretKeyConversionEngine<LweSecretKey32, GlweSecretKey32> for CoreEngine {
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use concrete_commons::parameters::{GlweDimension, LweDimension, PolynomialSize};
    /// use concrete_core::prelude::*;
    ///
    /// let mut core_engine = CoreEngine::new()?;
    ///
    /// let lwe_secret_key: LweSecretKey32 = core_engine.create_lwe_secret_key(LweDimension(20))?;
    ///
    /// let glwe_secret_key = core_engine.convert_lwe_to_glwe_secret_key(lwe_secret_key)?;
    ///
    /// assert_eq!(glwe_secret_key.glwe_dimension(), GlweDimension(20));
    /// assert_eq!(glwe_secret_key.polynomial_size(), PolynomialSize(1));
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn convert_lwe_to_glwe_secret_key(
        &mut self,
        input: LweSecretKey32,
        glwe_dimension: GlweDimension,
        polynomial_size: PolynomialSize,
    ) -> Result<GlweSecretKey32, LweToGlweSecretKeyConversionError<Self::EngineError>> {
        LweToGlweSecretKeyConversionError::perform_generic_checks(
            &input,
            glwe_dimension,
            polynomial_size,
        )?;
        Ok(unsafe { self.convert_lwe_to_glwe_secret_key_unchecked(input) })
    }

    unsafe fn convert_lwe_to_glwe_secret_key_unchecked(
        &mut self,
        input: LweSecretKey32,
    ) -> GlweSecretKey32 {
        GlweSecretKey32(input.0.into_glwe_secret_key())
    }
}

impl LweToGlweSecretKeyConversionEngine<LweSecretKey64, GlweSecretKey64> for CoreEngine {
    fn convert_lwe_to_glwe_secret_key(
        &mut self,
        input: LweSecretKey64,
        glwe_dimension: GlweDimension,
        polynomial_size: PolynomialSize,
    ) -> Result<GlweSecretKey64, LweToGlweSecretKeyConversionError<Self::EngineError>> {
        LweToGlweSecretKeyConversionError::perform_generic_checks(
            &input,
            glwe_dimension,
            polynomial_size,
        )?;
        Ok(unsafe { self.convert_lwe_to_glwe_secret_key_unchecked(input) })
    }

    unsafe fn convert_lwe_to_glwe_secret_key_unchecked(
        &mut self,
        input: LweSecretKey64,
    ) -> GlweSecretKey64 {
        GlweSecretKey64(input.0.into_glwe_secret_key())
    }
}
