

// Idea (and most of the code) from https://github.com/ggez/ggez-goodies/blob/master/src/particle.rs

// To make it simple, this is used to get a value by a %.delta-t
// ex: (0,1,2,3,4,5,6,7,8,9) 0.5 would give 4
pub trait Linear
where
    Self: Sized,
{
    /// Interpolate the value.  t should always be a number
    /// between 0.0 and 1.0, normalized for whatever actual
    /// value is the "end" of the interpolation.
    fn interp(&self, t: f64) -> Self;

    fn interp_between(v1: Self, v2: Self, t: f64) -> Self;

    /// A little shortcut that does the normalization for you.
    fn normalize_interp(&self, t: f64, max_t: f64) -> Self {
        let norm_t = t / max_t;
        self.interp(norm_t)
    }

    /// Combines interp_between with normalize_interp()
    fn normalize_interp_between(v1: Self, v2: Self, t: f64, max_t: f64) -> Self {
        let norm_t = t / max_t;
        Self::interp_between(v1, v2, norm_t)
    }
}

impl Linear for f64 {
    fn interp(&self, t: f64) -> Self {
        *self * t
    }

    fn interp_between(v1: Self, v2: Self, t: f64) -> Self {
        let val1 = v1.interp(1.0 - t);
        let val2 = v2.interp(t);
        val1 + val2
    }
}

impl Linear for crate::Point {
    fn interp(&self, t: f64) -> Self {
        crate::Point::new(self.x.interp(t), self.y.interp(t))
    }
    fn interp_between(v1: Self, v2: Self, t: f64) -> Self {
        let val1 = v1.interp(1. - t);
        let val2 = v2.interp(t);
        val1 + val2
    }
}

// From Vupa's code, may be usefull later
// // This function is broken; see ggj2017 code for fix.  :/
// // Is it ?
// impl Linear for render::Color {
//     fn interp(&self, t: f64) -> Self {
//         let rt = self.r() as f64 * t;
//         let gt = self.g() as f64 * t;
//         let bt = self.b() as f64 * t;
//         let at = self.a() as f64 * t;
//         render::Color::from_rgba(rt as u8, gt as u8, bt as u8, at as u8)
//     }

//     fn interp_between(v1: Self, v2: Self, t: f64) -> Self {
//         let val1 = v1.interp(1.0 - t);
//         let val2 = v2.interp(t);
//         let r = val1.r() + val2.r();
//         let g = val1.g() + val2.g();
//         let b = val1.b() + val2.b();
//         let a = val1.a() + val2.a();
//         render::Color::from_rgba(r, g, b, a)
//     }
// }
