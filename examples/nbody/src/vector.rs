use std::ops::{Sub,Add, Mul,AddAssign};
use crate::consts::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T: Copy> Vector<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Vector::<T> {
			x: x,
			y: y,
			z: z
		}
	}
}

impl<T: Mul<Output=T> + Add<Output=T> + Copy> Vector<T> {
	pub fn square(&self) -> T {
		self.x * self.x + self.y * self.y + self.z * self.z
	}
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Self {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
			z: self.z * scalar
        }
    }
}

impl<T: Add<Output=T> + AddAssign> AddAssign for Vector<T> {
	fn add_assign(&mut self, other: Vector<T>) {
		self.x += other.x;
    	self.y += other.y;
		self.z += other.z;
	}
}

impl<T: Add<Output=T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
			z: self.z + other.z
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, other: Vector<T>) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
			z: self.z - other.z
        }
    }
}

impl PartialEq for Vector<Precision> {
	fn eq(&self, other: &Vector<Precision>) -> bool {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;

		if dx*dx + dy*dy + dz*dz > 0.001 {
			false
		} else {
			true
		}
	}

	fn ne(&self, other: &Vector<Precision>) -> bool {
		!self.eq(other)
	}
}
