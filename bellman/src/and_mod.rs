use ff::PrimeField;

use crate::{Circuit, ConstraintSystem, SynthesisError};

pub struct AndDemo<Scalar: PrimeField> {
    pub a: Option<bool>,
    pub b: Option<bool>,
    pub tt: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for AndDemo<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        print!("a:{:?}  , b:{:?}  ", self.a, self.b);
        let a_var = cs.alloc(
            || "a",
            || {
                if self.a.is_some() {
                    if self.a.unwrap() {
                        Ok(Scalar::one())
                    } else {
                        Ok(Scalar::zero())
                    }
                } else {
                    Err(SynthesisError::AssignmentMissing)
                }
            },
        )?;

        cs.enforce(
            || "a_boolean_constraint",
            |lc| lc + CS::one() - a_var,
            |lc| lc + a_var,
            |lc| lc,
        );

        let b_var = cs.alloc(
            || "b",
            || {
                if self.b.is_some() {
                    if self.b.unwrap() {
                        Ok(Scalar::one())
                    } else {
                        Ok(Scalar::zero())
                    }
                } else {
                    Err(SynthesisError::AssignmentMissing)
                }
            },
        )?;
        let c_var = cs.alloc_input(
            || "c",
            || {
                if self.a.is_some() && self.b.is_some() {
                    if self.a.unwrap() && self.b.unwrap() {
                        Ok(Scalar::one())
                    } else {
                        Ok(Scalar::zero())
                    }
                } else {
                    Err(SynthesisError::AssignmentMissing)
                }
            },
        )?;

        cs.enforce(
            || "c_and_constraint",
            |lc| lc + a_var,
            |lc| lc + b_var,
            |lc| lc + c_var,
        );

        Ok(())
    }
}
