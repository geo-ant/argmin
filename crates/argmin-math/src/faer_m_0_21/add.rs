use crate::ArgminAdd;
use faer::{
    mat::{AsMatMut, AsMatRef},
    reborrow::{IntoConst, Reborrow, ReborrowMut},
    unzip, zip, Mat, MatMut, MatRef,
};
use faer_traits::ComplexField;
use std::ops::{Add, AddAssign};

/// MatRef + Scalar -> Mat
impl<E, R, C> ArgminAdd<E, Mat<E, R, C>> for MatRef<'_, E, R, C>
where
    E: ComplexField,
    R: faer::Shape,
    C: faer::Shape,
{
    #[inline]
    fn add(&self, other: &E) -> Mat<E, R, C> {
        zip!(self).map(|unzip!(this)| this.add_by_ref(other))
    }
}

/// Scaler + MatRef-> Mat
impl<'a, E, R, C> ArgminAdd<MatRef<'a, E, R, C>, Mat<E, R, C>> for E
where
    E: ComplexField,
    R: faer::Shape,
    C: faer::Shape,
{
    #[inline]
    fn add(&self, other: &MatRef<'a, E, R, C>) -> Mat<E, R, C> {
        // commutative with MatRef + Scalar so we can fall back on that case
        <_ as ArgminAdd<_, _>>::add(other, self)
    }
}

/// Mat + Scalar -> Mat
impl<E, R, C> ArgminAdd<E, Mat<E, R, C>> for Mat<E, R, C>
where
    E: ComplexField,
    R: faer::Shape,
    C: faer::Shape,
{
    #[inline]
    fn add(&self, other: &E) -> Mat<E, R, C> {
        //@note(geo-ant) because we are taking self by reference we
        // cannot mutate the matrix in place, so we can just as well
        // reuse the reference code
        <_ as ArgminAdd<_, _>>::add(&self.as_mat_ref(), other)
    }
}

/// Scalar + Mat -> Mat
impl<E, R, C> ArgminAdd<Mat<E, R, C>, Mat<E, R, C>> for E
where
    E: ComplexField,
    R: faer::Shape,
    C: faer::Shape,
{
    #[inline]
    fn add(&self, other: &Mat<E, R, C>) -> Mat<E, R, C> {
        // commutative with Mat + Scalar so we can fall back on that case
        <_ as ArgminAdd<_, _>>::add(other, self)
    }
}

/// MatRef + MatRef -> Mat
impl<'a, E> ArgminAdd<MatRef<'a, E>, Mat<E>> for MatRef<'_, E>
where
    E: ComplexField,
{
    #[inline]
    fn add(&self, other: &MatRef<'a, E>) -> Mat<E> {
        self + other
    }
}

/// MatRef + Mat -> Mat
impl<E: ComplexField> ArgminAdd<Mat<E>, Mat<E>> for MatRef<'_, E> {
    #[inline]
    fn add(&self, other: &Mat<E>) -> Mat<E> {
        self + other
    }
}

/// Mat + MatRef -> Mat
impl<E: ComplexField> ArgminAdd<MatRef<'_, E>, Mat<E>> for Mat<E> {
    #[inline]
    fn add(&self, other: &MatRef<'_, E>) -> Mat<E> {
        self + other
    }
}

/// Mat + Mat -> Mat
impl<E: ComplexField> ArgminAdd<Mat<E>, Mat<E>> for Mat<E> {
    #[inline]
    fn add(&self, other: &Mat<E>) -> Mat<E> {
        self + other
    }
}
