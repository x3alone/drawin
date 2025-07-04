use rand::Rng;
use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);

    fn color(&self) -> Color {
        let r = rand::thread_rng().gen_range(0..=255);
        let g = rand::thread_rng().gen_range(0..=255);
        let b = rand::thread_rng().gen_range(0..=255);

        Color { r, g, b, a: 255 }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(1..width);
        let y = rand::thread_rng().gen_range(1..height);

        Self::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color());
    }
}

pub struct Line {
    start: Point,
    end: Point,
    color: Color,
}

impl Line {
    pub fn new(start: Point, end: Point, color: Color) -> Self {
        Self { start, end, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = Point::random(width, height);
        let b = Point::random(width, height);

        Self::new(a, b, a.color())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut impl Displayable) {
        let x_start = self.start.x;
        let y_start = self.start.y;
        let x_end = self.end.x;
        let y_end = self.end.y;

        let dx = (x_end - x_start) as f32;
        let dy = (y_end - y_start) as f32;

        let steps = dx.abs().max(dy.abs()) as i32;

        let x_increment = dx / steps as f32;
        let y_increment = dy / steps as f32;

        let mut x = x_start as f32;
        let mut y = y_start as f32;

        for _ in 0..=steps {
            image.display(x.round() as i32, y.round() as i32, self.color.clone());
            x += x_increment;
            y += y_increment;
        }
    }
}

pub struct Rectangle {
    point_a: Point,
    point_b: Point,
    point_c: Point,
    point_d: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let color = p1.color();

        Self {
            point_a: *p1,
            point_b: *p2,
            point_c: Point { x: p1.x, y: p2.y },
            point_d: Point { x: p2.x, y: p1.y },
            color,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut impl Displayable) {
        Line::new(self.point_a, self.point_c, self.color.clone()).draw(image);
        Line::new(self.point_c, self.point_b, self.color.clone()).draw(image);
        Line::new(self.point_b, self.point_d, self.color.clone()).draw(image);
        Line::new(self.point_d, self.point_a, self.color.clone()).draw(image);
    }
}

pub struct Triangle {
    point_a: Point,
    point_b: Point,
    point_c: Point,
}

impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Self {
        Self {
            point_a: *point_a,
            point_b: *point_b,
            point_c: *point_c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut impl Displayable) {
        let color = self.color();
        Line::new(self.point_a, self.point_c, color.clone()).draw(image);
        Line::new(self.point_c, self.point_b, color.clone()).draw(image);
        Line::new(self.point_b, self.point_a, color.clone()).draw(image);
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p = Point::random(width, height);
        let r = rand::thread_rng().gen_range(0..width.min(height) / 2);
        Self::new(p, r)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut impl Displayable) {
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = -r;
        let color = self.color();

        while x < -y {
            let y_midpoint = y as f32 + 0.5;

            if x.pow(2) as f32 + y_midpoint.powf(2.0) > r.pow(2) as f32 {
                y += 1;
            }

            image.display(cx + x, cy + y, color.clone()); // top right
            image.display(cx - x, cy + y, color.clone()); // top left
            image.display(cx - x, cy - y, color.clone()); // bottom left
            image.display(cx + x, cy - y, color.clone()); // bottom right

            image.display(cx - y, cy - x, color.clone()); // right top
            image.display(cx - y, cy + x, color.clone()); // right bottom
            image.display(cx + y, cy - x, color.clone()); // left top
            image.display(cx + y, cy + x, color.clone()); // letf bottom

            x += 1;
        }
    }
}

pub struct Cube {
    point_a: Point,
    point_b: Point,
}

impl Cube {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            point_a: *a,
            point_b: *b,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut impl Displayable) {
        let color = self.color();
        let a = self.point_a;
        let b = self.point_b;

        let dx = (a.x - b.x) / 2;
        let dy = -((a.y - b.y) / 2);

        let mut rec_1 = Rectangle::new(&a, &b);
        let mut rec_2 = Rectangle::new(
            &Point {
                x: (a.x + dx),
                y: (a.y + dy),
            },
            &Point {
                x: (b.x + dx),
                y: (b.y + dy),
            },
        );

        rec_1.color = color.clone();
        rec_1.draw(image);

        rec_2.color = color.clone();
        rec_2.draw(image);

        Line::new(rec_1.point_a, rec_2.point_a, color.clone()).draw(image);
        Line::new(rec_1.point_b, rec_2.point_b, color.clone()).draw(image);
        Line::new(rec_1.point_c, rec_2.point_c, color.clone()).draw(image);
        Line::new(rec_1.point_d, rec_2.point_d, color.clone()).draw(image);
    }
}

pub struct Pentagon {
    lines: Vec<Line>,
}

impl Pentagon {
    pub fn new(start: &Point, side_length: i32) -> Self {
        let mut current_point = *start;
        let mut angle: f32 = 0.;
        let mut pentagon = Pentagon { lines: vec![] };
        let color = start.color();

        for _ in 1..=5 {
            let x_offset = ((side_length as f32) * angle.to_radians().cos()) as i32;
            let y_offset = ((side_length as f32) * angle.to_radians().sin()) as i32;

            let next_point = Point::new(current_point.x + x_offset, current_point.y + y_offset);

            pentagon
                .lines
                .push(Line::new(current_point, next_point, color.clone()));
            current_point = next_point;

            angle += 72.0;
        }
        return pentagon;
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut impl Displayable) {
        for line in &self.lines {
            line.draw(image);
        }
    }
}
