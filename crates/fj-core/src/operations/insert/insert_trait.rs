use crate::{
    objects::{
        Curve, Cycle, Edge, Face, Region, Shell, Sketch, Solid, Surface, Vertex,
    },
    operations::{Polygon, TetrahedronShell},
    services::Services,
    storage::Handle,
};

use super::{IsInsertedNo, IsInsertedYes};

/// Insert an object into its respective store
///
/// This is the only primitive operation that is directly understood by
/// `Service<Objects>`. All other operations are built on top of it.
pub trait Insert: Sized {
    /// The type of `Self`, once it has been inserted
    ///
    /// Usually this is just `Handle<Self>`, but there are some more complex
    /// cases where this type needs to be customized.
    type Inserted;

    /// Insert the object into its respective store
    #[must_use]
    fn insert(self, services: &mut Services) -> Self::Inserted;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                type Inserted = Handle<Self>;

                fn insert(self, services: &mut Services) -> Self::Inserted {
                    let handle = services.objects.$store.reserve();
                    let object = (handle.clone(), self).into();
                    services.insert_object(object);
                    handle
                }
            }
        )*
    };
}

impl_insert!(
    Curve, curves;
    Cycle, cycles;
    Face, faces;
    Edge, edges;
    Region, regions;
    Shell, shells;
    Sketch, sketches;
    Solid, solids;
    Surface, surfaces;
    Vertex, vertices;
);

impl<const D: usize> Insert for Polygon<D, IsInsertedNo> {
    type Inserted = Polygon<D, IsInsertedYes>;

    fn insert(self, services: &mut Services) -> Self::Inserted {
        Polygon {
            face: self.face.insert(services),
            edges: self.edges,
            vertices: self.vertices,
        }
    }
}

impl Insert for TetrahedronShell<IsInsertedNo> {
    type Inserted = TetrahedronShell<IsInsertedYes>;

    fn insert(self, services: &mut Services) -> Self::Inserted {
        TetrahedronShell {
            shell: self.shell.insert(services),
            abc: self.abc,
            bad: self.bad,
            dac: self.dac,
            cbd: self.cbd,
        }
    }
}
