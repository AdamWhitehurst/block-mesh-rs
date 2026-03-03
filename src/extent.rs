use std::ops::Add;

use glam::{IVec3, UVec3};

/// Axis-aligned extent over a half-open integer interval `[minimum, minimum + shape)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extent<V> {
    /// The least point contained in the extent.
    pub minimum: V,
    /// The length of each dimension.
    pub shape: V,
}

impl<V> Extent<V> {
    #[inline]
    pub const fn from_min_and_shape(minimum: V, shape: V) -> Self {
        Self { minimum, shape }
    }
}

impl Extent<IVec3> {
    /// Construct from inclusive min and max corners.
    /// Shape = max - min + 1.
    #[inline]
    pub fn from_min_and_max(min: IVec3, max: IVec3) -> Self {
        Self {
            minimum: min,
            shape: max - min + IVec3::ONE,
        }
    }

    /// Grow (positive) or shrink (negative) the extent by `amount` on each side.
    #[inline]
    pub fn padded(&self, amount: i32) -> Self {
        Self {
            minimum: self.minimum - IVec3::splat(amount),
            shape: self.shape + IVec3::splat(amount + amount),
        }
    }
}

impl Extent<UVec3> {
    /// Construct from inclusive min and max corners.
    /// Shape = max - min + 1.
    #[inline]
    pub fn from_min_and_max(min: UVec3, max: UVec3) -> Self {
        Self {
            minimum: min,
            shape: max - min + UVec3::ONE,
        }
    }

    /// Returns `Some(())` if all shape components are positive (> 0).
    #[inline]
    pub fn check_positive_shape(&self) -> Option<()> {
        let s = self.shape;
        (s.x > 0 && s.y > 0 && s.z > 0).then_some(())
    }

    /// Returns `true` if `self` is fully contained within `other`.
    #[inline]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        let self_min = self.minimum;
        let self_lub = self.least_upper_bound();
        let other_min = other.minimum;
        let other_lub = other.least_upper_bound();

        self_min.cmpge(other_min).all() && self_lub.cmple(other_lub).all()
    }

    /// The exclusive upper bound: `minimum + shape`.
    #[inline]
    pub fn least_upper_bound(&self) -> UVec3 {
        self.minimum + self.shape
    }

    /// Iterate over all `UVec3` points in `[minimum, minimum + shape)`.
    /// Order: z outermost, then y, then x innermost.
    #[inline]
    pub fn iter3(&self) -> impl Iterator<Item = UVec3> {
        let min = self.minimum;
        let lub = self.least_upper_bound();
        let z_range = min.z..lub.z;
        let y_range = min.y..lub.y;
        let x_range = min.x..lub.x;

        z_range.flat_map(move |z| {
            let x_range = x_range.clone();
            y_range
                .clone()
                .flat_map(move |y| x_range.clone().map(move |x| UVec3::new(x, y, z)))
        })
    }
}

impl Add<UVec3> for Extent<UVec3> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: UVec3) -> Self::Output {
        Self {
            minimum: self.minimum + rhs,
            shape: self.shape,
        }
    }
}
