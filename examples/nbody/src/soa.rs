//use rayon::prelude::*;
use std::ops::{Add, Mul, AddAssign, Index, IndexMut};
use std::default::Default;
use std::marker::PhantomData;
use std::fmt;
use crate::consts::*;

#[derive(Copy, Clone)]
pub struct Array<T>([T; N_PARTICLES_SOA]);

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "( ")?;
		for i in 0..N_PARTICLES_SOA {
			write!(f, "{:?} ", self.0[i])?;
		}
		write!(f, ")")
    }
}

impl<T: Copy + Default> Array<T> {
	pub fn new() -> Self {
		Array::<T>([Default::default(); N_PARTICLES_SOA])
	}
}

impl<T: Copy> Index<usize> for Array<T> {
	type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.0[idx]
    }
}

impl<T: Copy> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.0[idx]
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Array<T> {
    type Output = Array<T>;

    fn mul(self, scalar: T) -> Self {
		let mut array = Array::<T>(unsafe { std::mem::uninitialized() });

		for i in 0..self.0.len() {
			array.0[i] = self.0[i] * scalar;
		}

		array
    }
}

impl<T: Add<Output=T> + AddAssign + Copy> AddAssign for Array<T> {
	fn add_assign(&mut self, other: Array<T>) {
		for i in 0..other.0.len() {
			self.0[i] += other.0[i];
    	}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct StructOfArrays<T> {
	pub x: Array<T>,
	pub y: Array<T>,
	pub z: Array<T>
}

impl PartialEq for StructOfArrays<PrecisionSoA> {
	fn eq(&self, other: &StructOfArrays<PrecisionSoA>) -> bool {
		for i in 0..N_PARTICLES_SOA {
			let dx = self.x[i] - other.x[i];
			let dy = self.y[i] - other.y[i];
			let dz = self.z[i] - other.z[i];
 			let norm = dx*dx + dy*dy + dz*dz;

			for i in 0..PrecisionSoA::lanes() {
				if norm.extract(i) > 0.001 {
					return false;
				}
			}
		}

		true
	}

	fn ne(&self, other: &StructOfArrays<PrecisionSoA>) -> bool {
		!self.eq(other)
	}
}

impl<T: Copy + Default> StructOfArrays<T> {
	pub fn new() -> Self {
		StructOfArrays::<T> {
			x: Array::new(),
			y: Array::new(),
			z: Array::new()
		}
	}
}

impl<T> StructOfArrays<T> {
	pub fn iter<'a>(&'a self) -> StructOfArraysIter<'a, T> {
		StructOfArraysIter {
			inner: self,
			pos: 0,
			len: N_PARTICLES_SOA
		}
	}

	pub fn iter_mut(&mut self) -> StructOfArraysIterMut<T> {
		StructOfArraysIterMut {
			inner: self,
			pos: 0,
			len: N_PARTICLES_SOA,
			phantom: PhantomData
		}
	}
}

#[derive(Copy, Clone)]
pub struct StructOfArraysIter<'a, T: 'a> {
	inner: &'a StructOfArrays<T>,
	pos: usize,
	len: usize
}

impl<'a, T> Iterator for StructOfArraysIter<'a, T> {
	type Item = (&'a T, &'a T, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		if self.pos < self.len {
			let result = (&self.inner.x.0[self.pos],
				&self.inner.y.0[self.pos],
				&self.inner.z.0[self.pos]);
			self.pos += 1;
	        Some(result)
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len - self.pos, Some(self.len))
	}
}

#[derive(Clone)]
pub struct StructOfArraysIterMut<'a, T> {
	inner: *mut StructOfArrays<T>,
	pos: usize,
	len: usize,
	phantom: PhantomData<&'a T>,
}

impl<'a, T: Copy> Iterator for StructOfArraysIterMut<'a, T>  {
	type Item = (&'a mut T, &'a mut T, &'a mut T);

	fn next<'b>(&'b mut self) -> Option<Self::Item> {
		if self.pos < self.len {
			let result = unsafe { (&mut (*self.inner).x[self.pos],
				&mut (*self.inner).y[self.pos],
				&mut (*self.inner).z[self.pos]) };
			self.pos += 1;
	        Some(result)
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len - self.pos, Some(self.len))
	}
}

unsafe impl<'a, T: Send> Send for StructOfArraysIterMut<'a, T> {}
