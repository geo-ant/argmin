use crate::ArgminL1Norm;
use faer::{ComplexField, Entity, Mat, MatRef, SimpleEntity};

impl<'a, E: Entity + ComplexField> ArgminL1Norm<E::Real> for MatRef<'a, E> {
    fn l1_norm(&self) -> E::Real {
        self.norm_l1()
    }
}

impl<'a, E: Entity + ComplexField> ArgminL1Norm<E::Real> for Mat<E> {
    fn l1_norm(&self) -> E::Real {
        self.norm_l1()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn more_tests() {
        todo!()
    }
}