#include <stdio.h>
#include <math.h>

#define N_PARTICLES	1024
#define N_STEPS		20
#define DELTA_T		0.002f

typedef float Scalar;

typedef struct {
	Scalar x[N_PARTICLES] __attribute__ ((aligned (64)));
	Scalar y[N_PARTICLES] __attribute__ ((aligned (64)));
	Scalar z[N_PARTICLES] __attribute__ ((aligned (64)));
	Scalar vx[N_PARTICLES] __attribute__ ((aligned (64)));
	Scalar vy[N_PARTICLES] __attribute__ ((aligned (64)));
	Scalar vz[N_PARTICLES] __attribute__ ((aligned (64)));
} NBody;

static NBody nbody;

void c_init(void) {
	for(size_t i=0; i<N_PARTICLES; i++) {
		nbody.x[i] = (Scalar) i / (Scalar) N_PARTICLES;
		nbody.y[i] = nbody.x[i];
		nbody.z[i] = nbody.x[i];
		nbody.vx[i] = (Scalar) i / (Scalar) N_PARTICLES;;
		nbody.vy[i] = nbody.vx[i];
		nbody.vz[i] = nbody.vx[i];
	}
}

static void nbody_velocity_update(void) {
	#pragma omp parallel for
	for (size_t i = 0; i < N_PARTICLES; i++)  {
		float Fx = 0.0f;
		float Fy = 0.0f;
		float Fz = 0.0f;

		#pragma omp simd
		for (size_t j = 0; j < N_PARTICLES; j++)  {
			// Newton’s law of universal gravity calculation.
			const float dx = nbody.x[j] - nbody.x[i];
			const float dy = nbody.y[j] - nbody.y[i];
			const float dz = nbody.z[j] - nbody.z[i];
			const float drSquared = dx*dx + dy*dy + dz*dz;
			const float drPowerN32 = 1.0f/(drSquared*sqrtf(drSquared));

			// Reduction to calculate the net force
			Fx += dx * drPowerN32;
			Fy += dy * drPowerN32;
			Fz += dz * drPowerN32;
		}

		nbody.vx[i] += DELTA_T*Fx;
		nbody.vy[i] += DELTA_T*Fy;
		nbody.vz[i] += DELTA_T*Fz;
	}
}

static void nbody_position_update(void) {
	for (size_t i = 0; i < N_PARTICLES; i++)  {
		nbody.x[i] += nbody.vx[i]*DELTA_T;
		nbody.y[i] += nbody.vy[i]*DELTA_T;
		nbody.z[i] += nbody.vz[i]*DELTA_T;
	}
}

void c_parallel(void) {
	for(size_t i=0; i<N_STEPS; i++) {
		nbody_velocity_update();
		nbody_position_update();
	}
}
