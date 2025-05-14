trait Shape{
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

struct Triangle {
    sides_lens: [f64; 3],
}

impl Shape for Triangle{
    fn get_area(&self) -> f64{
        let p:f64 = self.get_perimeter()/2.0;
        (p * (p - self.sides_lens[0]) * (p - self.sides_lens[1]) * (p - self.sides_lens[2])).sqrt()
    }

    fn get_perimeter(&self) -> f64{
        self.sides_lens.iter().sum()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle{
    fn get_area(&self) -> f64{
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64{
        (self.width + self.height)*2.0
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle{
    fn get_area(&self) -> f64{
        2.0 * self.radius * std::f64::consts::PI
    }

    fn get_perimeter(&self) -> f64{
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// отношение  периметра к площади (P/A) фигуры
fn perimeter_by_area(figure:  Box<dyn Shape>) -> f64 {
    figure.get_perimeter()/figure.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test_triangle() {
        assert!(relative_eq!(
            perimeter_by_area(Box::new(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            })),
            2.0
        ));
    }

    #[test]
    fn test_circle() {
        assert!(relative_eq!(perimeter_by_area(Box::new(Circle { radius: 2.0 })), 1.0));
    }

    #[test]
    fn test_rectangle() {
        assert!(relative_eq!(
            perimeter_by_area(Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            })),
            1.6666666666666667
        ));
    }

}