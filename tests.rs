use Vector;
use Line;
use axioms;

const EPSILON: f64 = f64::EPSILON * 10.0;

macro_rules! assert_delta {
	($x:expr, $y:expr, $d:expr) => {
		assert_eq!(true, $x - $y < $d && $y - $x < $d);
	}
}

fn vector_tests () {
	let sqrt2 = (2.0_f64).sqrt();
	let v: Vector = Vector { x: 1.2, y: -0.8 };
	let u: Vector = Vector { x: 2.0, y: 2.0 };
	let l = Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
	let m = Line { u: Vector { x: -sqrt2, y: sqrt2 }, d: 1.0 };
	// let m = Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
	let mag1: f64 = u.magnitude();
	let mag2: f64 = u.normalize().magnitude();
	let norm: Vector = u.normalize();
	let rot90: Vector = u.normalize().rotate90();
	let rot270: Vector = u.normalize().rotate270();
	let flip: Vector = v.flip();
	let dot: f64 = u.dot(&v);
	let determ: f64 = v.determinant(&u);
	let degenerate: bool = u.is_degenerate();
	let parallel: bool = u.is_parallel(&v);
	let (success, intersect) = l.intersect(&m);
	let equivalent: bool = u.equivalent(&v);
	assert_delta!(mag1, 2.8284271247461903, EPSILON);
	assert_delta!(mag2, 1.0, EPSILON);
	assert_delta!(norm.x, (2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(norm.y, (2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(rot90.x, -(2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(rot90.y, (2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(rot270.x, (2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(rot270.y, -(2.0_f64).sqrt() / 2.0, EPSILON);
	assert_delta!(flip.x, -1.2, EPSILON);
	assert_delta!(flip.y, 0.8, EPSILON);
	assert_delta!(dot, 0.8, EPSILON);
	assert_delta!(determ, 4.0, EPSILON);
	assert_eq!(degenerate, false);
	assert_eq!(parallel, false);
	assert_eq!(success, true);
	assert_eq!(equivalent, false);
	assert_delta!(intersect.x, 1.0, EPSILON);
	assert_delta!(intersect.y, 1.0_f64 + 2.0_f64.sqrt() / 2.0, EPSILON);
}

fn line_tests () {
	let a = Line {
		u: Vector { x: 0.7071067811865475, y: 0.7071067811865475},
		d: 0.7071067811865475
	};
	let b = Line {
		u: Vector { x: 1.0, y: 0.0},
		d: 0.5
	};
	let equivalent_a: bool = a.equivalent(&b);
	let equivalent_b: bool = b.equivalent(&a);
	assert_eq!(equivalent_a, false);
	assert_eq!(equivalent_b, false);

	// make sure these should be duplicate
	// test if they are duplicate
	// duplicate test Line { x: -1.0, y: 0.0, d: -0.5 } Line { x: 1.0, y: 0.0, d: 0.5 }
}

fn axiom_tests () {
	let u: &Vector = &Vector { x: 2.0, y: 2.0 };
	let v: &Vector = &Vector { x: 1.2, y: -0.8 };
	let l: &Line = &Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
	let m: &Line = &Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
	let ax1 = axioms::axiom1(u, v);
	let ax2 = axioms::axiom2(u, v);
	let (ax3a, ax3b) = axioms::axiom3(l, m);
	assert_delta!(ax1.u.x, 0.9615239476408233, EPSILON);
	assert_delta!(ax1.u.y, -0.2747211278973781, EPSILON);
	assert_delta!(ax1.d, 1.3736056394868903, EPSILON);
	assert_delta!(ax2.u.x, -0.2747211278973781, EPSILON);
	assert_delta!(ax2.u.y, -0.9615239476408233, EPSILON);
	assert_delta!(ax2.d, -1.016468173220299, EPSILON);
	assert_delta!(ax3a.u.x, 1.0, EPSILON);
	assert_delta!(ax3a.u.y, 0.0, EPSILON);
	assert_delta!(ax3a.d, 1.0, EPSILON);
	assert_delta!(ax3b.u.x, 0.0, EPSILON);
	assert_delta!(ax3b.u.y, 1.0, EPSILON);
	assert_delta!(ax3b.d, 1.0, EPSILON);
}

pub fn run_tests () {
	vector_tests();
	axiom_tests();
	line_tests();
}
