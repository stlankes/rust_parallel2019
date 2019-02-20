use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use crate::soa::*;
use crate::consts::*;
use crate::vector::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NBodySoA {
	position: StructOfArrays<PrecisionSoA>,
	velocity: StructOfArrays<PrecisionSoA>
}

impl NBodySoA {
	pub fn new() -> Self {
		let mut nbody = NBodySoA {
			position: StructOfArrays::<PrecisionSoA>::new(),
			velocity: StructOfArrays::<PrecisionSoA>::new()
		};

		for i in 0..N_PARTICLES_SOA {
			nbody.position.x[i] = PrecisionSoA::new((PrecisionSoA::lanes()*i) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+1) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+2) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+3) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+4) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+5) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+6) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+7) as Precision / N_PARTICLES as Precision);
			nbody.position.y[i] = nbody.position.x[i];
			nbody.position.z[i] = nbody.position.x[i];
			nbody.velocity.x[i] = PrecisionSoA::new((PrecisionSoA::lanes()*i) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+1) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+2) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+3) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+4) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+5) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+6) as Precision / N_PARTICLES as Precision,
									(PrecisionSoA::lanes()*i+7) as Precision / N_PARTICLES as Precision);
			nbody.velocity.y[i] = nbody.velocity.x[i];
			nbody.velocity.z[i] = nbody.velocity.x[i];
		}

		nbody
	}

	pub fn get_position(&self, idx: usize) -> Vector<Precision> {
			let i = idx / PrecisionSoA::lanes();
			let j = idx % PrecisionSoA::lanes();

			Vector::new(self.position.x[i].extract(j),
				self.position.y[i].extract(j),
				self.position.z[i].extract(j))
	}

	fn parallel_velocity_update(&mut self) {
		let position = &self.position;
		let velocity = &mut self.velocity;
		let dt = PrecisionSoA::splat(DELTA_T);
		let one = PrecisionSoA::splat(1.0);

		position.iter().zip(velocity.iter_mut()).par_bridge().for_each(|((pix, piy, piz), (vix, viy, viz))| {
			let mut fx: PrecisionSoA = PrecisionSoA::splat(0.0);
			let mut fy: PrecisionSoA = PrecisionSoA::splat(0.0);
			let mut fz: PrecisionSoA = PrecisionSoA::splat(0.0);

			position.iter().for_each(|(pjx, pjy, pjz)| {
				// Newton’s law of universal gravity calculation.
				let mut dx: PrecisionSoA = PrecisionSoA::splat(0.0);
				let mut dy: PrecisionSoA = PrecisionSoA::splat(0.0);
				let mut dz: PrecisionSoA = PrecisionSoA::splat(0.0);

				for lane in 0..PrecisionSoA::lanes() {
					dx +=  *pjx - PrecisionSoA::splat(pix.extract(lane));
					dy +=  *pjy - PrecisionSoA::splat(piy.extract(lane));
					dz +=  *pjz - PrecisionSoA::splat(piz.extract(lane));
				}

				let n2 = dx*dx + dy*dy + dz*dz;
				let power = one / (n2.sqrt() * n2);

				fx += dx*power;
				fy += dy*power;
				fz += dz*power;
			});

			*vix += fx * dt;
			*viy += fy * dt;
			*viz += fz * dt;
		});
	}


	fn velocity_update(&mut self) {
		let position = &self.position;
		let velocity = &mut self.velocity;
		let dt = PrecisionSoA::splat(DELTA_T);
		let one = PrecisionSoA::splat(1.0);

		position.iter().zip(velocity.iter_mut()).for_each(|((pix, piy, piz), (vix, viy, viz))| {
			let mut fx: PrecisionSoA = PrecisionSoA::splat(0.0);
			let mut fy: PrecisionSoA = PrecisionSoA::splat(0.0);
			let mut fz: PrecisionSoA = PrecisionSoA::splat(0.0);

			position.iter().for_each(|(pjx, pjy, pjz)| {
				// Newton’s law of universal gravity calculation.
				let mut dx: PrecisionSoA = PrecisionSoA::splat(0.0);
				let mut dy: PrecisionSoA = PrecisionSoA::splat(0.0);
				let mut dz: PrecisionSoA = PrecisionSoA::splat(0.0);

				for lane in 0..PrecisionSoA::lanes() {
					dx +=  *pjx - PrecisionSoA::splat(pix.extract(lane));
					dy +=  *pjy - PrecisionSoA::splat(piy.extract(lane));
					dz +=  *pjz - PrecisionSoA::splat(piz.extract(lane));
				}

				let n2 = dx*dx + dy*dy + dz*dz;
				let power = one / (n2.sqrt() * n2);

				fx += dx*power;
				fy += dy*power;
				fz += dz*power;
			});

			*vix += fx * dt;
			*viy += fy * dt;
			*viz += fz * dt;
		});
	}

	fn position_update(&mut self) {
		for i in 0..N_PARTICLES_SOA {
			self.position.x[i] += self.velocity.x[i] * DELTA_T;
			self.position.y[i] += self.velocity.y[i] * DELTA_T;
			self.position.z[i] += self.velocity.z[i] * DELTA_T;
		}
	}

	pub fn sequential(&mut self) {
		// sequential version
		for _ in 0..N_STEPS {
			self.velocity_update();
			self.position_update();
		}
	}

	pub fn parallel(&mut self) {
		// parallel version
		for _ in 0..N_STEPS {
			self.parallel_velocity_update();
			self.position_update();
		}
	}
}
