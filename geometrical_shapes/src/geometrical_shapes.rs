use raster::{Color, Image};
use rand::random_range;


pub trait Drawable {
    fn draw(&self, image: &mut Image);
    
    fn color(&self) -> Color {
        let r: u8 = random_range(0..=255);
        let g: u8 = random_range(0..=255);
        let b: u8 = random_range(0..=255);
        Color { r, g, b, a: 255 }
    }
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Debug)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self(random_range(0..width), random_range(0..height))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }
}

#[derive(Clone, Debug)]
pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Self(p1, p2)
    }

    pub fn draw_color(&self, image: &mut Image, color: &Color) {
        let (x1, y1) = (self.0.0, self.0.1);
        let (x2, y2) = (self.1.0, self.1.1);

        let dx = x2 - x1;
        let dy = y2 - y1;
        let steps = dx.abs().max(dy.abs());

        let x_inc = dx as f64 / steps as f64;
        let y_inc = dy as f64 / steps as f64;

        let mut x = x1 as f64;
        let mut y = y1 as f64;

        for _ in 0..=steps {
            image.display(x.round() as i32, y.round() as i32, color.clone());
            x += x_inc;
            y += y_inc;
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        self.draw_color(image, &color);
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle(Point, Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();

        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;

        Line::new(&Point(x1, y1), &Point(x2, y1)).draw_color(image, &color);
        Line::new(&Point(x2, y1), &Point(x2, y2)).draw_color(image, &color);
        Line::new(&Point(x2, y2), &Point(x1, y2)).draw_color(image, &color);
        Line::new(&Point(x1, y2), &Point(x1, y1)).draw_color(image, &color);
    }
}

#[derive(Clone, Debug)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(p1.clone(), p2.clone(), p3.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();

        Line::new(&self.0, &self.1).draw_color(image, &color);
        Line::new(&self.1, &self.2).draw_color(image, &color);
        Line::new(&self.2, &self.0).draw_color(image, &color);
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = random_range(1..height);
        Self::new(center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let (cx, cy) = (self.center.0, self.center.1);
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(cx + x, cy + y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx - y, cy - x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx + x, cy - y, color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }
}
