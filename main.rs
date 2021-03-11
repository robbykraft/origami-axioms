mod primitives;
mod square;
mod axioms;
mod tree;
mod tests;
mod draw;
use primitives::Vector;
use primitives::Line;
use primitives::Segment;
use tree::Tree;
use tree::make_tree;
use square::Square;
use tests::run_tests;
use draw::draw;

const UNIT_SQUARE: Square = Square {
	a: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
	b: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
	c: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 },
	d: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }
};

fn duplicate_line_check (line: &Line, lines: &mut Vec<(Line, u64)>) -> bool {
	for k in 0..lines.len() {
		if line.equivalent(&lines[k].0) {
			lines[k].1 += 1;
			return true;
		}
	}
	return false;
}

fn duplicate_point_check (point: &Vector, points: &mut Vec<(Vector, u64)>) -> bool {
	for k in 0..points.len() {
		if point.equivalent(&points[k].0) {
			points[k].1 += 1;
			return true;
		}
	}
	return false;
}


fn compute_axiom1 (
	// points: &mut Vec<(Vector, u64)>, // the previous round of points (build from this)
	point_tree: &mut Tree, // the previous round of points (build from this)
	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
	new_lines: &mut Vec<(Line, u64)> // the current round (check for duplicates only)
) {
	let points: Vec<&Vector> = point_tree.flatten();
	for i in 0..points.len() - 1 {
		println!("{}/{}: {} axiom 1", i, points.len(), new_lines.len());
		for j in (i + 1)..points.len() {
			let line: Line = axioms::axiom1(&points[i], &points[j]);
			if duplicate_line_check(&line, lines) { continue }
			if duplicate_line_check(&line, new_lines) { continue }
			new_lines.push((line, 1));
		}
	}
}

fn compute_axiom2 (
	// points: &mut Vec<(Vector, u64)>, // the previous round of points (build from this)
	point_tree: &mut Tree, // the previous round of points (build from this)
	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
	new_lines: &mut Vec<(Line, u64)> // the current round (check for duplicates only)
) {
	let points: Vec<&Vector> = point_tree.flatten();
	for i in 0..points.len() - 1 {
		println!("{}/{}: {} axiom 2", i, points.len(), new_lines.len());
		for j in (i + 1)..points.len() {
			let line: Line = axioms::axiom2(&points[i], &points[j]);
			if duplicate_line_check(&line, lines) { continue }
			if duplicate_line_check(&line, new_lines) { continue }
			new_lines.push((line, 1));
		}
	}
}

// fn compute_axiom3 (
// 	points: &mut Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)> // the current round (check for duplicates only)
// ) {
// 	for i in 0..lines.len() - 1 {
// 		for j in (i + 1)..lines.len() {
// 			let (a, b) = axioms::axiom3(&lines[i].0, &lines[j].0);
// 			if duplicate_line_check(&a, lines) { continue }
// 			if duplicate_line_check(&a, new_lines) { continue }
// 			new_lines.push((a, 1));
// 			if duplicate_line_check(&b, lines) { continue }
// 			if duplicate_line_check(&b, new_lines) { continue }
// 			new_lines.push((b, 1));
// 		}
// 	}
// }

fn compute_intersections (
	// points: &mut Vec<(Vector, u64)>, // already existing intersection points
	points: &mut Tree,
	new_lines: &mut Vec<(Line, u64)> // the newest set of lines
// ) -> Vec<(Vector, u64)> {
) -> Tree {
    let mut round: Tree = make_tree();
	// let mut round: Vec<(Vector, u64)> = Vec::new();
	if new_lines.len() == 0 { return round }
	for i in 0..new_lines.len() - 1 {
		if i > 300 { return round; }
		println!("{}/{}: {} new points", i, new_lines.len(), round.len());
		for j in (i + 1)..new_lines.len() {
			let (success, point) = new_lines[i].0.intersect(&new_lines[j].0);
			if !success { continue }
			if !UNIT_SQUARE.contains(&point) { continue }
			// if duplicate_point_check(&point, points) { continue }
			// if duplicate_point_check(&point, &mut round) { continue }
            if points.duplicate_check(&point) { continue }
            if round.duplicate_check(&point) { continue }
			// round.push((point, 1));
            round.push(&point);
		}
	}
	return round;
}

fn compute_round (
	// points: &mut Vec<(Vector, u64)>,
	points: &mut Tree,
	lines: &mut Vec<(Line, u64)>,
	compute_pts: bool
) {
	// new lines is all the lines made in THIS round
	let mut new_lines: Vec<(Line, u64)> = Vec::new();
	// 1. compute all axioms for this round
	// let mut new_lines: Vec<(Line, u64)> = compute_axiom2(points, lines, new_lines);
	compute_axiom1(points, lines, &mut new_lines);
	compute_axiom2(points, lines, &mut new_lines);
	// todo: list more axioms
	// 2. compute new intersection points
	// let mut new_points: Vec<(Vector, u64)> = if compute_pts { compute_intersections(
	// 	points, &mut new_lines) } else { Vec::new() };
	// let mut new_points: Vec<(Vector, u64)> = compute_intersections(
	// 	points, &mut new_lines);
	let mut new_points: Tree = if compute_pts { compute_intersections(points, &mut new_lines) }
		else { make_tree() };
	// 3. merge points and lines from this new round
	// points.append(&mut new_points);
	points.merge(&mut new_points);
	lines.append(&mut new_lines);
}

fn main () {
	run_tests();

    let tree = make_tree();
    
    // fixed resolution. 100x100
    // let mut tree: Tree = make_tree();
    // let test: Vector = Vector { x: 0.99999, y: 0.99999999};
    // let test2: Vector = Vector { x: 0.99999, y: 0.99999999};
    // tree.push(&test);
    // tree.push(&test2);
    // let res = test.index();
    // println!("{}, {}", res.0, res.1);
    // tree[res.0][res.1].push(test);
    // println!("{:?}", tree.buckets);

	// let mut points: Vec<(Vector, u64)> = Vec::new();
    let mut points = make_tree();
	let mut lines: Vec<(Line, u64)> = Vec::new();
	// points.push((Vector { x: 0.0, y: 0.0 }, 1));
	// points.push((Vector { x: 1.0, y: 0.0 }, 1));
	// points.push((Vector { x: 1.0, y: 1.0 }, 1));
	// points.push((Vector { x: 0.0, y: 1.0 }, 1));
	points.push(&Vector { x: 0.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 1.0 });
	points.push(&Vector { x: 0.0, y: 1.0 });

	for i in 0..3 {
		compute_round(&mut points, &mut lines, if i > 2 { false } else { true });
		// compute_round(&mut points, &mut lines);
		// println!("..finished axioms round {} ({} lines {} points)",
		// 	i, lines.len(), points.len());
	}

	let mut segments: Vec<Segment> = Vec::new();
	for i in 0..lines.len() {
		let (success, segment) = UNIT_SQUARE.clip(&lines[i].0);
		if success { segments.push(segment) }
	}

	// let marks: Vec<Vector> = points.iter()
	// 	.map(|el| el.0)
	// 	.collect::<Vec<Vector>>();

	let marks: Vec<Vector> = Vec::new();

	// println!("{} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	println!("{} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	draw(&segments, &marks);
}
