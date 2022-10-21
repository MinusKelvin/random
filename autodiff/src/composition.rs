use crate::AutoDiff;

#[derive(Default)]
pub struct Chain<F, G>(F, G);

impl<I, F, G> AutoDiff<I> for Chain<F, G>
where
    F: AutoDiff<I>,
    G: AutoDiff<F::Output>,
{
    type Output = G::Output;

    fn y(&self, x: &I) -> Self::Output {
        self.1.y(&self.0.y(x))
    }

    fn zero_grad(&mut self) {
        self.0.zero_grad();
        self.1.zero_grad();
    }

    fn forward(&mut self, x: &I) -> Self::Output {
        self.1.forward(&self.0.forward(x))
    }

    fn backward(&mut self, dl_dy: &Self::Output) -> I {
        self.0.backward(&(self.1.backward(dl_dy)))
    }
}

impl<F, G> Chain<F, G> {
    pub fn new(f: F, g: G) -> Self {
        Chain(f, g)
    }
}

#[derive(Default)]
pub struct Add<F, G>(F, G);

impl<F, G> AutoDiff<f32> for Add<F, G>
where
    F: AutoDiff<f32, Output = f32>,
    G: AutoDiff<f32, Output = f32>,
{
    type Output = f32;

    fn y(&self, x: &f32) -> Self::Output {
        self.0.y(x) + self.1.y(x)
    }

    fn zero_grad(&mut self) {
        self.0.zero_grad();
        self.1.zero_grad();
    }

    fn forward(&mut self, x: &f32) -> Self::Output {
        self.0.forward(x) + self.1.forward(x)
    }

    fn backward(&mut self, dl_dy: &Self::Output) -> f32 {
        self.0.backward(dl_dy) + self.1.backward(dl_dy)
    }
}

impl<F, G> Add<F, G> {
    pub fn new(f: F, g: G) -> Self {
        Add(f, g)
    }
}

#[derive(Default)]
pub struct Mul<F1, F2> {
    f: F1,
    g: F2,
    dy_df: f32,
    dy_dg: f32,
}

impl<F, G> AutoDiff<f32> for Mul<F, G>
where
    F: AutoDiff<f32, Output = f32>,
    G: AutoDiff<f32, Output = f32>,
{
    type Output = f32;

    fn y(&self, x: &f32) -> Self::Output {
        self.f.y(x) * self.g.y(x)
    }

    fn zero_grad(&mut self) {
        self.dy_df = 0.0;
        self.dy_dg = 0.0;
    }

    fn forward(&mut self, x: &f32) -> Self::Output {
        let f = self.f.forward(x);
        let g = self.g.forward(x);
        self.dy_df = g;
        self.dy_dg = f;
        f * g
    }

    fn backward(&mut self, dl_dy: &Self::Output) -> f32 {
        self.dy_df * self.f.backward(dl_dy) + self.dy_dg * self.g.backward(dl_dy)
    }
}

impl<F, G> Mul<F, G> {
    pub fn new(f: F, g: G) -> Self {
        Mul {
            f,
            g,
            dy_df: 0.0,
            dy_dg: 0.0,
        }
    }
}
