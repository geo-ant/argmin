// Copyright 2018-2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # References:
//!
//! [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
//! Springer. ISBN 0-387-30303-0.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Gauss-Newton method
///
/// [Example](https://github.com/argmin-rs/argmin/blob/master/examples/gaussnewton.rs)
///
/// # References:
///
/// [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
/// Springer. ISBN 0-387-30303-0.
#[derive(Serialize, Deserialize)]
pub struct GaussNewton {
    /// gamma
    gamma: f64,
}

impl GaussNewton {
    /// Constructor
    pub fn new() -> Self {
        GaussNewton { gamma: 1.0 }
    }

    /// set gamma
    pub fn set_gamma(mut self, gamma: f64) -> Result<Self, Error> {
        if gamma <= 0.0 || gamma > 1.0 {
            return Err(ArgminError::InvalidParameter {
                text: "Gauss-Newton: gamma must be in  (0, 1].".to_string(),
            }
            .into());
        }
        self.gamma = gamma;
        Ok(self)
    }
}

impl Default for GaussNewton {
    fn default() -> GaussNewton {
        GaussNewton::new()
    }
}

impl<O> Solver<O> for GaussNewton
where
    O: ArgminOp,
    O::Param: ArgminScaledSub<O::Param, f64, O::Param>
        + ArgminDot<O::Param, O::Param>
        + ArgminMul<f64, O::Param>,
    O::Jacobian: ArgminTranspose
        + ArgminInv<O::Jacobian>
        + ArgminDot<O::Jacobian, O::Jacobian>
        + ArgminDot<O::Output, O::Param>
        + ArgminDot<O::Param, O::Param>,
{
    const NAME: &'static str = "Gauss-Newton method";

    fn next_iter(
        &mut self,
        op: &mut OpWrapper<O>,
        state: &IterState<O>,
    ) -> Result<ArgminIterData<O>, Error> {
        let param = state.get_param();
        let residuals = op.apply(&param)?;
        let grad = op.gradient(&param)?;
        let jacobian = op.jacobian(&param)?;
        let jacobian_t = jacobian.clone().t();

        let p = jacobian_t
            .dot(&jacobian)
            .inv()?
            .dot(&jacobian.t().dot(&residuals))
            .mul(&1.0);

        let new_param = param.scaled_sub(&self.gamma, &p.dot(&grad));

        Ok(ArgminIterData::new().param(new_param))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::send_sync_test;

    send_sync_test!(gauss_newton_method, GaussNewton);
}