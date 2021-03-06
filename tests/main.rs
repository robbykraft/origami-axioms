extern crate origami_axioms;

use origami_axioms::math::Vector;
use origami_axioms::math::Rect;
use origami_axioms::math::make_square;
use origami_axioms::origami;
// use origami_axioms::GridVec;
use origami_axioms::QuadTree;
// use origami_axioms::make_grid;
use origami_axioms::make_tree;
use origami_axioms::LineContainer;
use origami_axioms::make_line_container;

// use make_line_container;
// use make_grid;
// use fold;
// use GridVec;
// use LineContainer;

// const EPSILON: f64 = f64::EPSILON * 10.0;

// macro_rules! assert_delta {
// 	($x:expr, $y:expr, $d:expr) => {
// 		assert_eq!(true, $x - $y < $d && $y - $x < $d);
// 	}
// }

#[test]
fn make_axiom_tests () {
	let boundary: Rect = make_square();
	// let mut point_quadtree: GridVec = make_grid();
	let mut point_quadtree: QuadTree = make_tree();
	let mut line_container: LineContainer = make_line_container();
	point_quadtree.push(Vector { x: 0.0, y: 0.0 });
	point_quadtree.push(Vector { x: 1.0, y: 0.0 });
	point_quadtree.push(Vector { x: 1.0, y: 1.0 });
	point_quadtree.push(Vector { x: 0.0, y: 1.0 });
	boundary.sides.iter().for_each(|side| line_container.push(side));
	let points = point_quadtree.flatten();
	let lines = line_container.flatten();
	// let _points1 = point_quadtree.flatten_filter(0);
	// let _points2 = point_quadtree.filter_by_count(0);
	// let _lines1 = line_container.flatten_filter(0);
	let mut new_line_container: LineContainer = make_line_container();
	origami::make_axiom1(&points, &mut line_container, &mut new_line_container);
	origami::make_axiom2(&points, &mut line_container, &mut new_line_container);
	origami::make_axiom3(&points, &lines, &mut line_container, &mut new_line_container, boundary);
	origami::make_axiom4(&points, &lines, &mut line_container, &mut new_line_container, boundary);
	origami::make_axiom5(&points, &lines, &mut line_container, &mut new_line_container, boundary);
	origami::make_axiom6(&points, &lines, &mut line_container, &mut new_line_container, boundary);
	origami::make_axiom7(&points, &lines, &mut line_container, &mut new_line_container, boundary);
	let new_lines = new_line_container.flatten();
	let old_lines = line_container.flatten();
	// let _new_points: GridVec = origami::make_intersections(
	// 	&mut point_quadtree, &old_lines, &new_lines, boundary);
	let _new_points: QuadTree = origami::make_intersections(
		&mut point_quadtree, &old_lines, &new_lines, boundary);
}
