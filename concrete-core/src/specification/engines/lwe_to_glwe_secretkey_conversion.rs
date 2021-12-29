use concrete_commons::parameters::{GlweDimension, LweDimension, PolynomialSize};
use super::engine_error;

use crate::specification::engines::AbstractEngine;
use crate::specification::entities::GlweSecretKeyEntity;
use crate::specification::entities::LweSecretKeyEntity;


engine_error! {
    LweToGlweSecretKeyConversionError for LweToGlweSecretKeyConversionEngine @
    LweSizeMismatch => "The input LweSize does not match the output GlweDimension * PolynomialSize"
}

impl<EngineError: std::error::Error> LweToGlweSecretKeyConversionError<EngineError> {
    pub fn perform_generic_checks<Input>(input: &Input, glwe_dimension: GlweDimension, polynomial_size: PolynomialSize) -> Result<(), Self> where Input: LweSecretKeyEntity {
        if input.lwe_dimension().0 != glwe_dimension.0 * polynomial_size.0 {
            return Err(Self::LweSizeMismatch);
        }
        Ok(())
    }
}


pub trait LweToGlweSecretKeyConversionEngine<Input, Output>: AbstractEngine
    where
        Input: LweSecretKeyEntity,
        Output: GlweSecretKeyEntity<KeyDistribution=Input::KeyDistribution>
{
    fn convert_lwe_to_glwe_secret_key(
        &mut self,
        input: Input,
        glwe_dimension: GlweDimension,
        polynomial_size: PolynomialSize,
    ) -> Result<Output, LweToGlweSecretKeyConversionError<Self::EngineError>>;

    unsafe fn convert_lwe_to_glwe_secret_key_unchecked(
        &mut self,
        input: Input,
    ) -> Output;
}