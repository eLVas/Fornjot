use std::f64::consts::PI;

use fj::{
    core::{
        algorithms::sweep::Sweep,
        objects::{Cycle, Region, Sketch, Solid},
        operations::{
            BuildCycle, BuildRegion, BuildSketch, Insert, Reverse,
            UpdateRegion, UpdateSketch,
        },
        services::Services,
        storage::Handle,
    },
    math::Vector,
};

pub fn model(
    num_points: u64,
    r1: f64,
    r2: f64,
    h: f64,
    services: &mut Services,
) -> Handle<Solid> {
    let num_vertices = num_points * 2;
    let vertex_iter = (0..num_vertices).map(|i| {
        let angle_rad = 2. * PI / num_vertices as f64 * i as f64;
        let radius = if i % 2 == 0 { r1 } else { r2 };
        (angle_rad, radius)
    });

    let mut outer_points = Vec::new();
    let mut inner_points = Vec::new();

    for (angle_rad, radius) in vertex_iter {
        let (sin, cos) = angle_rad.sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        outer_points.push([x, y]);
        inner_points.push([x / 2., y / 2.]);
    }

    let sketch = Sketch::empty()
        .add_region(
            Region::polygon(outer_points, services)
                .add_interiors([Cycle::polygon(inner_points, services)
                    .reverse(services)
                    .insert(services)])
                .insert(services),
        )
        .insert(services);

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., h]);
    (sketch, surface).sweep(path, services)
}
