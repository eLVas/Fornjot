use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    objects::{Cycle, Edge},
    operations::{BuildEdge, Insert, UpdateCycle},
    services::Services,
};

/// Build a [`Cycle`]
pub trait BuildCycle {
    /// Build an empty cycle
    fn empty() -> Cycle {
        Cycle::new([])
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Cycle {
        let circle = Edge::circle(center, radius, services).insert(services);
        Cycle::empty().add_edges([circle])
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, services: &mut Services) -> Cycle
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let edges = points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .map(|(start, end)| {
                Edge::line_segment([start, end], None, services)
                    .insert(services)
            });

        Cycle::new(edges)
    }
}

impl BuildCycle for Cycle {}
