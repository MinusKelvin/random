use crate::AutoDiff;

#[derive(Default)]
pub struct Elementary<F> {
    f: F,
    dy_dx: f32,
}

trait ElementaryFunction {
    fn y(&self, x: f32) -> f32;
    fn dy(&self, x: f32) -> f32;
}

impl<F: ElementaryFunction> AutoDiff<f32> for Elementary<F> {
    type Output = f32;

    fn y(&self, &x: &f32) -> Self::Output {
        self.f.y(x)
    }

    fn zero_grad(&mut self) {
        self.dy_dx = 0.0;
    }

    fn forward(&mut self, &x: &f32) -> Self::Output {
        self.dy_dx = self.f.dy(x);
        self.f.y(x)
    }

    fn backward(&mut self, dl_dy: &Self::Output) -> f32 {
        dl_dy * self.dy_dx
    }
}

#[derive(Default)]
pub struct Neg;

impl ElementaryFunction for Neg {
    fn y(&self, x: f32) -> f32 {
        -x
    }

    fn dy(&self, _x: f32) -> f32 {
        -1.0
    }
}

#[derive(Default)]
pub struct Cos;

impl ElementaryFunction for Cos {
    fn y(&self, x: f32) -> f32 {
        x.cos()
    }

    fn dy(&self, x: f32) -> f32 {
        -x.sin()
    }
}

#[derive(Default)]
pub struct Sin;

impl ElementaryFunction for Sin {
    fn y(&self, x: f32) -> f32 {
        x.sin()
    }

    fn dy(&self, x: f32) -> f32 {
        x.cos()
    }
}

#[derive(Default)]
pub struct Exp;

impl ElementaryFunction for Exp {
    fn y(&self, x: f32) -> f32 {
        x.exp()
    }

    fn dy(&self, x: f32) -> f32 {
        x.exp()
    }
}

#[derive(Default)]
pub struct Reciprocol;

impl ElementaryFunction for Reciprocol {
    fn y(&self, x: f32) -> f32 {
        x.recip()
    }

    fn dy(&self, x: f32) -> f32 {
        -(x * x).recip()
    }
}
