use na::{Pnt2, Vec2, BaseFloat};
use na;
use math::Scalar;
use entities::shape::Capsule2;
use procedural::{Polyline, Polyline2};
use procedural::utils;
use super::ToPolyline;

impl<N: Scalar> ToPolyline<N, Pnt2<N>, Vec2<N>, u32> for Capsule2<N> {
    fn to_polyline(&self, nsubdiv: u32) -> Polyline2<N> {
        let pi: N = BaseFloat::pi();
        let dtheta     = pi / na::cast(nsubdiv as f64);

        let mut points: Vec<Pnt2<N>> = Vec::with_capacity(nsubdiv as usize);

        utils::push_xy_arc(self.radius(), nsubdiv, dtheta, &mut points);

        let npoints = points.len();

        for i in range(0, npoints) {
            let new_point = points[i] + Vec2::new(na::zero(), self.half_height());

            points.push(-new_point);
            points[i] = new_point;
        }

        Polyline::new(points, None)
    }
}
