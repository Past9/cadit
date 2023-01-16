use crate::math::{bezier::decasteljau, FloatRange, Homogeneous, Vec2, Vec2H, Vector};

#[derive(Debug, Clone)]
pub struct Line2D {
    a: f64,
    b: f64,
    c: f64,
}
impl Line2D {
    pub fn from_pos_and_dir(pos: Vec2, dir: Vec2) -> Self {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self { a, b, c }
    }

    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn normalize(&self) -> Self {
        let vec_mag = (self.a.powi(2) + self.b.powi(2)).sqrt();
        Self {
            a: self.a / vec_mag,
            b: self.b / vec_mag,
            c: self.c,
        }
    }
}

#[derive(Debug)]
pub struct BezierCurve<H: Homogeneous> {
    control_points: Vec<H>,
}
impl<H: Homogeneous> BezierCurve<H> {
    pub fn new(control_points: Vec<H>) -> Self {
        Self { control_points }
    }

    pub fn point(&self, u: f64) -> H::Projected {
        let p = decasteljau(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            u,
        );

        H::cast_from_weighted(p).project()
    }
}
impl BezierCurve<Vec2H> {
    pub fn line_intersection_curve(&self, line: Line2D) -> Self {
        //let line = line.normalize();
        let coefficients = self
            .control_points
            .iter()
            .map(|pt| pt.x * line.a + pt.y * line.b + line.c)
            .collect::<Vec<_>>();

        let mut control_points = Vec::new();

        for x in FloatRange::new(0.0, 1.0, 50) {
            println!("({} {})", x, decasteljau(&coefficients, x));
        }

        for (i, coefficient) in coefficients.iter().enumerate() {
            let u = i as f64 / (coefficients.len() - 1) as f64;
            control_points.push(Vec2H {
                x: u,
                y: decasteljau(&coefficients, u),
                h: 1.0,
            });
            println!("U {} {}", u, decasteljau(&coefficients, u));
        }

        Self { control_points }
    }

    /*
    pub fn intersect_line(&self, line: Line2D) -> Vec<f64> {
        //
    }
    */
}
