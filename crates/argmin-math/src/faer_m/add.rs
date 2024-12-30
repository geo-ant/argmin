use super::RealEntity;
use crate::ArgminAdd;
use faer::{
    mat::AsMatRef,
    reborrow::{IntoConst, Reborrow, ReborrowMut},
    unzipped, zipped_rw, ComplexField, Conjugate, Entity, Mat, MatMut, MatRef, SimpleEntity,
};
use std::ops::{Add, AddAssign};

/// MatRef + Scalar -> Mat
impl<'a, E> ArgminAdd<E, Mat<E>> for MatRef<'a, E>
where
    E: Entity + Add<E, Output = E>,
{
    #[inline]
    fn add(&self, other: &E) -> Mat<E> {
        let mut sum = Mat::<E>::zeros(self.nrows(), self.ncols());
        zipped_rw!(sum.as_mut()).for_each(|unzipped!(mut sum)| {
            let added = sum.read() + *other;
            sum.write(added)
        });
        sum
    }
}

//@todo(geo) also add scalar + Matrix and matrix + Scalar (and reference variants?)

/// Mat + Scalar -> Mat
impl<E> ArgminAdd<E, Mat<E>> for Mat<E>
where
    E: Entity + Add<E, Output = E>,
{
    #[inline]
    fn add(&self, other: &E) -> Mat<E> {
        <MatRef<E> as ArgminAdd<_, _>>::add(&self.as_mat_ref(), other)
    }
}

/// MatRef + MatRef -> Mat
impl<'a, 'b, E: SimpleEntity + ComplexField> ArgminAdd<MatRef<'a, E>, Mat<E>> for MatRef<'b, E> {
    #[inline]
    fn add(&self, other: &MatRef<'a, E>) -> Mat<E> {
        <_ as Add>::add(self, other)
    }
}

/// MatRef + Mat -> Mat
impl<'a, 'b, E: SimpleEntity + ComplexField> ArgminAdd<Mat<E>, Mat<E>> for MatRef<'b, E> {
    #[inline]
    fn add(&self, other: &Mat<E>) -> Mat<E> {
        self + other
    }
}

/// Mat + Mat -> Mat
impl<'a, 'b, E: SimpleEntity + ComplexField> ArgminAdd<Mat<E>, Mat<E>> for Mat<E> {
    #[inline]
    fn add(&self, other: &Mat<E>) -> Mat<E> {
        self + other
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_scalar() {
        fn add_scalar(scalar: f64, mat: &Mat<f64>) -> Mat<f64> {
            <MatRef<f64> as ArgminAdd<f64, Mat<f64>>>::add(&mat.as_mat_ref(), &scalar)
        }
        let mat = Mat::<f64>::zeros(10, 11);
        let mut expected = Mat::<f64>::zeros(10, 11);
        let mat = add_scalar(1., &mat);
        expected.fill(1.);
        assert_eq!(mat, expected);
    }

    #[test]
    fn more_tests() {
        todo!()
    }
}
