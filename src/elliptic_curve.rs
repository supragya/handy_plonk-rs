use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign, Sub, Neg}, hash::Hash};

pub trait Field:
    Sized       // Size should be known compile time
    + Debug     // Should be printable for Debugging
    + Copy      // Making a copy of this is an expensive operation
    + Display   // Should have a display output
    + PartialEq // Elements could implement eq and not eq
    + From<i64> // Should be convertible from an i64 value
    + From<u64> // Should be convertible from an u64 value
    + Add<Output = Self> // Addition is possible for field
    + for<'a> Add<&'a Self, Output = Self>
    + AddAssign
    + for<'a> AddAssign<&'a Self>
    + Div<Output = Option<Self>>
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output=Self>
    + for<'a> MulAssign<&'a Self>
    + Sub<Output = Self>
    + for<'a> SubAssign<&'a Self>
    + Neg<Output = Self>
{
    type Order: Display;

    fn order() -> Self::Order;
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn one() -> Self;
    fn as_u64(&self) -> u64;
    fn in_field(&self) -> bool;
    fn inv(&self) -> Option<Self>;
    fn pow(&self, exp: u64) -> Self;
    // fn as_poly(&self) -> Poly<Self> {
    //     Poly::new(vec![*self])
    // }
    fn carrying_mul(&self, rhs: &Self, carry: &mut Self) -> Self;
}

pub trait G1Point:
    Copy
    + Display
    + PartialEq
    + Hash
    + Ord
    + Neg<Output = Self>
    + Add<Output = Self>
    + Mul<Self::S, Output = Self>
{
    type F: Field;
    type S: Field;

    fn generator() -> Self;
    fn generator_subgroup_size() -> Self::F;
    fn identity() -> Self;
    fn new(x: Self::F, y: Self::F) -> Self;
    fn x(&self) -> &Self::F;
    fn y(&self) -> &Self::F;
    fn in_curve(&self) -> bool;
    fn is_identity(&self) -> bool;
}

pub trait G2Point:
    Display
    + Copy
    + PartialEq
    + Neg<Output = Self>
    + Add
    + Mul<Self::S, Output = Self>
{
    type F: Field;
    type S: Field;

    fn x(&self) -> &Self::F;
    fn y(&self) -> &Self::F;

    fn new(x: Self::F, y: Self::F) -> Self;
    fn generator() -> Self;
    fn embedding_degree() -> u64;
}

pub trait GTPoint:
    Display
    + Copy
    + PartialEq
    + Mul
{
    type S;
    fn pow(&self, n: Self::S) -> Self;
}

pub trait Pairing {
    type G1: G1Point;
    type G2: G2Point;
    type GT: GTPoint;

    fn pairing(p: Self::G1, q: Self::G2) -> Self::GT;
}