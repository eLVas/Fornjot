use fj_math::Vector;

use crate::{
    geometry::{GlobalPath, SurfaceGeometry},
    storage::{Handle, Store},
};

use super::{
    Curve, Cycle, Edge, Face, Region, Shell, Sketch, Solid, Surface, Vertex,
};

/// The available object stores
#[derive(Debug, Default)]
pub struct Objects {
    /// Store for [`Curve`]s
    pub curves: Store<Curve>,

    /// Store for [`Cycle`]s
    pub cycles: Store<Cycle>,

    /// Store for [`Edge`]s
    pub edges: Store<Edge>,

    /// Store for [`Face`]s
    pub faces: Store<Face>,

    /// Store for [`Region`]s
    pub regions: Store<Region>,

    /// Store for [`Shell`]s
    pub shells: Store<Shell>,

    /// Store for [`Sketch`]es
    pub sketches: Store<Sketch>,

    /// Store for [`Solid`]s
    pub solids: Store<Solid>,

    /// Store for [`Surface`]s
    pub surfaces: Surfaces,

    /// Store for [`Vertex`] objects
    pub vertices: Store<Vertex>,
}

impl Objects {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for [`Surface`]s
#[derive(Debug)]
pub struct Surfaces {
    store: Store<Surface>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Surfaces {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Surface> {
        self.store.reserve()
    }

    /// Insert an object into the store
    pub fn insert(&mut self, handle: Handle<Surface>, surface: Surface) {
        self.store.insert(handle, surface);
    }

    /// Access the xy-plane
    pub fn xy_plane(&self) -> Handle<Surface> {
        self.xy_plane.clone()
    }

    /// Access the xz-plane
    pub fn xz_plane(&self) -> Handle<Surface> {
        self.xz_plane.clone()
    }

    /// Access the yz-plane
    pub fn yz_plane(&self) -> Handle<Surface> {
        self.yz_plane.clone()
    }
}

impl Default for Surfaces {
    fn default() -> Self {
        let mut store: Store<Surface> = Store::new();

        let xy_plane = store.reserve();
        store.insert(
            xy_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            }),
        );

        let xz_plane = store.reserve();
        store.insert(
            xz_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            }),
        );
        let yz_plane = store.reserve();
        store.insert(
            yz_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::y_axis(),
                v: Vector::unit_z(),
            }),
        );

        Self {
            store,
            xy_plane,
            xz_plane,
            yz_plane,
        }
    }
}
