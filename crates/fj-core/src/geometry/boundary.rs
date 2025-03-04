use std::{
    cmp::{self, Ordering},
    hash::{Hash, Hasher},
};

use fj_math::Point;

use crate::{objects::Vertex, storage::HandleWrapper};

/// A boundary on a curve
///
/// This struct is generic, because different situations require different
/// representations of a boundary. In some cases, curve coordinates are enough,
/// in other cases, vertices are required, and sometimes you need both.
#[derive(Clone, Copy, Debug)]
pub struct CurveBoundary<T: CurveBoundaryElement> {
    /// The raw representation of the boundary
    pub inner: [T::Repr; 2],
}

impl<T: CurveBoundaryElement> CurveBoundary<T> {
    /// Indicate whether the boundary is normalized
    ///
    /// If the boundary is normalized, its bounding elements are in a defined
    /// order, and calling `normalize` will return an identical instance.
    pub fn is_normalized(&self) -> bool {
        let [a, b] = &self.inner;
        a <= b
    }

    /// Reverse the direction of the boundary
    ///
    /// Returns a new instance of this struct, which has its direction reversed.
    #[must_use]
    pub fn reverse(self) -> Self {
        let [a, b] = self.inner;
        Self { inner: [b, a] }
    }

    /// Normalize the boundary
    ///
    /// Returns a new instance of this struct, which has the bounding elements
    /// in a defined order. This can be used to compare boundaries while
    /// disregarding their direction.
    #[must_use]
    pub fn normalize(self) -> Self {
        if self.is_normalized() {
            self
        } else {
            self.reverse()
        }
    }
}

// Technically, these methods could be implemented for all
// `CurveBoundaryElement`s, but that would be misleading. While
// `HandleWrapper<Vertex>` implements `Ord`, which is useful for putting it (and
// by extension, `CurveBoundary<Vertex>`) into `BTreeMap`s, this `Ord`
// implementation doesn't actually define the geometrically meaningful ordering
// that the following methods rely on.
impl CurveBoundary<Point<1>> {
    /// Indicate whether the boundary is empty
    pub fn is_empty(&self) -> bool {
        let [min, max] = &self.inner;
        min >= max
    }

    /// Indicate whether the boundary contains the given element
    pub fn contains(&self, point: Point<1>) -> bool {
        let [min, max] = self.inner;
        point > min && point < max
    }

    /// Indicate whether the boundary overlaps another
    ///
    /// Boundaries that touch (i.e. their closest boundary elements are equal)
    /// count as overlapping.
    pub fn overlaps(&self, other: &Self) -> bool {
        let [a_low, a_high] = self.normalize().inner;
        let [b_low, b_high] = other.normalize().inner;

        a_low <= b_high && a_high >= b_low
    }

    /// Create the subset of this boundary and another
    ///
    /// The result will be normalized.
    #[must_use]
    pub fn subset(self, other: Self) -> Self {
        let self_ = self.normalize();
        let other = other.normalize();

        let [self_min, self_max] = self_.inner;
        let [other_min, other_max] = other.inner;

        let min = cmp::max(self_min, other_min);
        let max = cmp::min(self_max, other_max);

        Self { inner: [min, max] }
    }

    /// Create the union of this boundary and another
    ///
    /// The result will be normalized.
    ///
    /// # Panics
    ///
    /// Panics, if the two boundaries don't overlap (touching counts as
    /// overlapping).
    pub fn union(self, other: Self) -> Self {
        let self_ = self.normalize();
        let other = other.normalize();

        assert!(
            self.overlaps(&other),
            "Can't merge boundaries that don't at least touch"
        );

        let [self_min, self_max] = self_.inner;
        let [other_min, other_max] = other.inner;

        let min = cmp::min(self_min, other_min);
        let max = cmp::max(self_max, other_max);

        Self { inner: [min, max] }
    }
}

impl<S, T: CurveBoundaryElement> From<[S; 2]> for CurveBoundary<T>
where
    S: Into<T::Repr>,
{
    fn from(boundary: [S; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}

impl<T: CurveBoundaryElement> Eq for CurveBoundary<T> {}

impl<T: CurveBoundaryElement> PartialEq for CurveBoundary<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: CurveBoundaryElement> Hash for CurveBoundary<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T: CurveBoundaryElement> Ord for CurveBoundary<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: CurveBoundaryElement> PartialOrd for CurveBoundary<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

/// An element of a curve boundary
///
/// Used for the type parameter of [`CurveBoundary`].
pub trait CurveBoundaryElement {
    /// The representation the curve boundary element
    ///
    /// This is the actual data stored in [`CurveBoundary`].
    type Repr: Eq + Hash + Ord;
}

impl CurveBoundaryElement for Point<1> {
    type Repr = Self;
}

impl CurveBoundaryElement for Vertex {
    type Repr = HandleWrapper<Vertex>;
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::geometry::CurveBoundary;

    #[test]
    fn overlaps() {
        assert!(overlap([0., 2.], [1., 3.])); // regular overlap
        assert!(overlap([0., 1.], [1., 2.])); // just touching
        assert!(overlap([2., 0.], [3., 1.])); // not normalized
        assert!(overlap([1., 3.], [0., 2.])); // lower boundary comes second

        assert!(!overlap([0., 1.], [2., 3.])); // regular non-overlap
        assert!(!overlap([2., 3.], [0., 1.])); // lower boundary comes second

        fn overlap(a: [f64; 2], b: [f64; 2]) -> bool {
            let a = array_to_boundary(a);
            let b = array_to_boundary(b);

            a.overlaps(&b)
        }

        fn array_to_boundary(array: [f64; 2]) -> CurveBoundary<Point<1>> {
            CurveBoundary::from(array.map(|element| [element]))
        }
    }
}
