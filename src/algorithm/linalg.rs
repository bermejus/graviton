use ndarray::{Data, LinalgScalar, prelude::*};
use num::traits::{Signed, Float, zero};

pub fn magnitude<A, S>(x: &ArrayBase<S, Ix1>) -> A
where
    S: Data<Elem = A>,
    A: LinalgScalar
{
    x.dot(x)
}

pub fn l1_norm<A, S>(x: &ArrayBase<S, Ix1>) -> A
where
    S: Data<Elem = A>,
    A: Signed + From<i32>
{
    x.fold(zero::<A>(), |acc, elem| acc + elem.abs())
}

pub fn l2_norm<A, S>(x: &ArrayBase<S, Ix1>) -> A
where
    S: Data<Elem = A>,
    A: LinalgScalar + Float
{
    x.dot(x).sqrt()
}

pub fn normalize<A, S>(x: &ArrayBase<S, Ix1>) -> Array<A, Ix1>
where
    S: Data<Elem = A>,
    A: LinalgScalar + Float
{
    let norm = l2_norm(x);
    x.mapv(|e| e/norm)
}
