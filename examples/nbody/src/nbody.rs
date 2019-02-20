use crate::vector::*;
use crate::consts::*;
use crate::consts::Precision;
use std::vec::Vec;
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NBody {
	pub position: Vec<Vector<Precision>>,
	pub velocity: Vec<Vector<Precision>>
}

impl NBody {
	pub fn new() -> Self {
		let position: Vec<_> = (0..N_PARTICLES).map(|i| {
                Vector::<Precision>::new(i as Precision / N_PARTICLES as Precision,
					i as Precision / N_PARTICLES as Precision,
					i as Precision / N_PARTICLES as Precision)
            })
            .collect();

		NBody {
			position: position.clone(),
			velocity: position.clone()
		}
	}

	fn parallel_velocity_update(&mut self)
	{
		let position = &self.position;
		let velocity = &mut self.velocity;

		position.par_iter().zip(velocity.par_iter_mut()).for_each(|(item_pi, item_vi)| {
			let mut f: Vector<Precision> = Vector::new(0.0, 0.0, 0.0);

			position.iter().for_each(|item_pj| {
				// Newton’s law of universal gravity calculation.
				let diff = *item_pj - *item_pi;
				let n2 = diff.square();
				let power = 1.0 / (n2.sqrt() * n2);

				f += diff*power;
			});

			*item_vi += f*DELTA_T;
		});
	}


	fn velocity_update(&mut self)
	{
		let position = &self.position;
		let velocity = &mut self.velocity;

		position.iter().zip(velocity.iter_mut()).for_each(|(item_pi, item_vi)| {
			let mut f: Vector<Precision> = Vector::new(0.0, 0.0, 0.0);

			position.iter().for_each(|item_pj| {
				// Newton’s law of universal gravity calculation.
				let diff = *item_pj - *item_pi;
				let n2 = diff.square();
				let power = 1.0 / (n2.sqrt() * n2);

				f += diff*power;
			});

			*item_vi += f*DELTA_T;
		});
	}

	fn position_update(&mut self) {
		self.position.iter_mut().zip(self.velocity.iter()).for_each(|(item_pi, item_vi)| {
			*item_pi += *item_vi * DELTA_T;
		});
	}

	pub fn sequential(&mut self) {
		// sequential version
		for _t in 0..N_STEPS {
			self.velocity_update();
			self.position_update();
		}
	}

	pub fn parallel(&mut self) {
		// parallel version
		for _t in 0..N_STEPS {
			self.parallel_velocity_update();
			self.position_update();
		}
	}
}
