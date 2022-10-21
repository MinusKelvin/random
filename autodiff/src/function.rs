use crate::composition::{Chain, Add, Mul};
use crate::elementary::{Exp, Reciprocol, Neg, Elementary, Cos, Sin};
use crate::{AutoDiff, Constant, Identity};

pub(crate) const X: Function<Identity> = Function(Identity);

#[derive(Default)]
pub(crate) struct Function<F>(F);

impl<F> Function<F> {
    pub fn y<I>(&self, x: &I) -> F::Output
    where
        F: AutoDiff<I>,
    {
        self.0.y(x)
    }

    pub fn forward<I>(&mut self, x: &I) -> F::Output
    where
        F: AutoDiff<I>,
    {
        self.0.forward(x)
    }

    pub fn backward<I>(&mut self, x: &F::Output) -> I
    where
        F: AutoDiff<I>,
    {
        self.0.backward(x)
    }

    pub fn zero_grad<I>(&mut self)
    where
        F: AutoDiff<I>,
    {
        self.0.zero_grad()
    }

    pub fn of<G>(self, g: Function<G>) -> Function<Chain<G, F>> {
        Function(Chain::new(g.0, self.0))
    }

    pub fn exp(self) -> Function<Chain<F, Elementary<Exp>>> {
        Function(Default::default()).of(self)
    }

    pub fn recip(self) -> Function<Chain<F, Elementary<Reciprocol>>> {
        Function(Default::default()).of(self)
    }

    pub fn cos(self) -> Function<Chain<F, Elementary<Cos>>> {
        Function(Default::default()).of(self)
    }

    pub fn sin(self) -> Function<Chain<F, Elementary<Sin>>> {
        Function(Default::default()).of(self)
    }
}

impl<F1, F2> std::ops::Add<Function<F2>> for Function<F1>
where
    F1: AutoDiff<f32, Output = f32>,
    F2: AutoDiff<f32, Output = f32>,
{
    type Output = Function<Add<F1, F2>>;

    fn add(self, rhs: Function<F2>) -> Self::Output {
        Function(Add::new(self.0, rhs.0))
    }
}

impl<F: AutoDiff<f32, Output = f32>> std::ops::Add<f32> for Function<F> {
    type Output = Function<Add<F, Constant<f32>>>;

    fn add(self, rhs: f32) -> Self::Output {
        self + Function(Constant(rhs))
    }
}

impl<F: AutoDiff<f32, Output = f32>> std::ops::Add<Function<F>> for f32 {
    type Output = Function<Add<F, Constant<f32>>>;

    fn add(self, rhs: Function<F>) -> Self::Output {
        rhs + Function(Constant(self))
    }
}

impl<F: AutoDiff<f32, Output=f32>> std::ops::Neg for Function<F> {
    type Output = Function<Chain<F, Neg>>;

    fn neg(self) -> Self::Output {
        Function(Neg::default()).of(self)
    }
}

impl<F1, F2> std::ops::Mul<Function<F2>> for Function<F1>
where
    F1: AutoDiff<f32, Output = f32>,
    F2: AutoDiff<f32, Output = f32>,
{
    type Output = Function<Mul<F1, F2>>;

    fn mul(self, rhs: Function<F2>) -> Self::Output {
        Function(Mul::new(self.0, rhs.0))
    }
}

impl<F: AutoDiff<f32, Output = f32>> std::ops::Mul<f32> for Function<F> {
    type Output = Function<Mul<F, Constant<f32>>>;

    fn mul(self, rhs: f32) -> Self::Output {
        self * Function(Constant(rhs))
    }
}

impl<F: AutoDiff<f32, Output = f32>> std::ops::Mul<Function<F>> for f32 {
    type Output = Function<Mul<F, Constant<f32>>>;

    fn mul(self, rhs: Function<F>) -> Self::Output {
        rhs * Function(Constant(self))
    }
}
