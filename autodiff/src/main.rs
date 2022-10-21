use function::X;

mod composition;
mod function;
mod elementary;

fn main() {
    let mut f = X.cos().exp() * (X.sin() + 2.0).recip();

    for x in range(-1.0, 1.0) {
        f.zero_grad();
        let v = f.forward(&x);
        let dv_dx = f.backward(&1.0);
        println!("x={x:.4}, f(x)={v:.4}, f'(x)={dv_dx:.4}")
    }
}

fn range(x1: f32, x2: f32) -> impl Iterator<Item = f32> {
    (0..=10).map(move |t| {
        let t = t as f32 / 10.0;
        x1 * (1.0 - t) + x2 * t
    })
}

trait AutoDiff<Input> {
    type Output;

    fn y(&self, x: &Input) -> Self::Output;

    fn zero_grad(&mut self);
    fn forward(&mut self, x: &Input) -> Self::Output;
    fn backward(&mut self, dl_dy: &Self::Output) -> Input;
}

#[derive(Default)]
struct Constant<T>(T);

impl<I: Default, T: Clone> AutoDiff<I> for Constant<T> {
    type Output = T;

    fn y(&self, _x: &I) -> Self::Output {
        self.0.clone()
    }

    fn zero_grad(&mut self) {}

    fn forward(&mut self, _x: &I) -> Self::Output {
        self.0.clone()
    }

    fn backward(&mut self, _dl_dy: &Self::Output) -> I {
        I::default()
    }
}

#[derive(Default)]
struct Identity;

impl<I: Clone> AutoDiff<I> for Identity {
    type Output = I;

    fn y(&self, x: &I) -> Self::Output {
        x.clone()
    }

    fn zero_grad(&mut self) {}

    fn forward(&mut self, x: &I) -> Self::Output {
        x.clone()
    }

    fn backward(&mut self, dl_dy: &Self::Output) -> I {
        dl_dy.clone()
    }
}
