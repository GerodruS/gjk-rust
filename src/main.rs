use std::ops;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

impl ops::Neg for &Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

fn dot_product(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y
}

fn triple_cross_product(a: &Point, b: &Point, c: &Point) -> Point {
    let ab = (a.x * b.y) - (a.y * b.x);
    Point {
        x: -ab * c.y,
        y: ab * c.x
    }
}

fn furthest_point(points: &[Point], vector: &Point) -> Point {
    let mut result_point = &points[0];
    let mut max_value = dot_product(result_point, &vector);

    for i in 1..points.len() {
        let point = &points[i];
        let value = dot_product(point, &vector);
        if max_value < value {
            result_point = point;
            max_value = value;
        }
    }

    result_point.clone()
}

fn support(a: &[Point], b: &[Point], vector: &Point) -> Point {
    let furthest_a = furthest_point(a, vector);
    let furthest_b = furthest_point(b, &-vector);

    furthest_a - furthest_b
}

//    if AB
//        [A,B] ABxAOxAB
//    else
//        [A] AO

fn gjk2d(a: &[Point], b: &[Point]) -> bool {
    let vector = Point { x: 0.0, y: 1.0 };
    let support_point = support(a, b, &vector);

    let mut simplex = Vec::with_capacity(3);
    simplex.push(support_point.clone());

    let mut vector = -support_point;

    loop {
        let support_point = support(a, b, &vector);
        if dot_product(&support_point, &vector) < 0.0 { return false; }
        simplex.push(support_point.clone());
        if simplex.len() == 2 {
            let a = &simplex[1];
            let b = &simplex[0];
            let ab = b - a;
            let ao = -a;
            if 0.0 < dot_product(&ab, &ao) {
                vector = triple_cross_product(&ab, &ao, &ab);
                let a = a.clone();
                let b = b.clone();
                simplex.clear();
                simplex.push(a);
                simplex.push(b);
            } else {
                vector = ao.clone();
                let a = a.clone();
                simplex.clear();
                simplex.push(a);
            }
        } else {
            let a = &simplex[2];
            let b = &simplex[1];
            let c = &simplex[0];
            let ab = b - a;
            let ac = c - a;
            let ao = -a;
            let d = triple_cross_product(&ab, &ac, &ac);
            if 0.0 < dot_product(&d, &ao) {
                vector = d.clone();
                let aa = c.clone();
                let bb = a.clone();
                simplex.clear();
                simplex.push(aa);
                simplex.push(bb);
            } else {
                let d = triple_cross_product(&ac, &ab, &ab);
                if 0.0 < dot_product(&d, &ao) {
                    vector = d.clone();
                    let aa = b.clone();
                    let bb = a.clone();
                    simplex.clear();
                    simplex.push(aa);
                    simplex.push(bb);
                } else {
                    return true;
                }
            }
        }
    }
}

//    S = Support(?)
//    [] = S
//    D = -S
//    Loop:
//        A = Support(D)
//        if dot(A,D) < 0 then no intersection
//        [] += A
//        if DoSimplex([], D) `then intersection

fn main() {
    let a = [
        Point { x: 4.0, y: 11.0 },
        Point { x: 4.0, y: 5.0 },
        Point { x: 9.0, y: 9.0 },
    ];

    let b = [
        Point { x: 7.0, y: 7.0 },
        Point { x: 122.0, y: 7.0 },
        Point { x: 7.0, y: 3.0 },
        Point { x: 10.0, y: 2.0 },
    ];

    for i in 0..10000 {
        let has_collision = gjk2d(&a, &b);

        print!("has_collision={}", has_collision);
    }
}
