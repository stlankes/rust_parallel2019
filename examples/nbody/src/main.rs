#![feature(test)]
#![feature(link_args)]

extern crate test;
extern crate rayon;
extern crate packed_simd;

mod vector;
mod nbody;
mod nbody_soa;
mod consts;
mod soa;

use nbody::*;
use nbody_soa::*;

#[link_args = "-fopenmp"]
extern {
    fn c_init();
	fn c_parallel();
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn naive_parallel_works() {
		let mut particle_golden = NBody::new();
		particle_golden.sequential();

		let mut particle = NBody::new();
		particle.parallel();

		assert_eq!(particle_golden, particle);
	}

	#[test]
	fn soa_parallel_works() {
		let mut particle_golden = NBodySoA::new();
		particle_golden.sequential();

		let mut particle = NBodySoA::new();
		particle.parallel();

		assert_eq!(particle_golden, particle);
	}

	#[test]
	fn compare_both_solutions() {
		let mut particle_golden = NBody::new();
		particle_golden.sequential();

		let mut particle = NBodySoA::new();
		particle.sequential();

		particle_golden.position.iter().enumerate().for_each(|(i, pos_golden)| {
			let pos_soa = particle.get_position(i);

			assert_eq!(*pos_golden, pos_soa);
		});
	}

	#[bench]
	fn bench_c(b: &mut Bencher) {
		unsafe {
			c_init();
		}

		b.iter(|| unsafe { c_parallel() });
	}

	#[bench]
	fn bench_naive_sequential(b: &mut Bencher) {
		let mut particle = NBody::new();
		b.iter(|| particle.sequential());
	}

	#[bench]
	fn bench_naive_parallel(b: &mut Bencher) {
		let mut particle = NBody::new();
		b.iter(|| particle.parallel());
	}

	#[bench]
	fn bench_SoA_sequential(b: &mut Bencher) {
		let mut particle = NBodySoA::new();
		b.iter(|| particle.sequential());
	}

	#[bench]
	fn bench_SoA_parallel(b: &mut Bencher) {
		let mut particle = NBodySoA::new();
		b.iter(|| particle.parallel());
	}

}

fn main() {
	println!("Please run 'cargo bench' to get performance results.");
}
