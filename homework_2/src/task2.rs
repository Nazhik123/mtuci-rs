/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

fn main() {}

struct Rect {
    top_left1: (f32, f32),
    width1: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        let rect = Rect {
            top_left1 : top_left,
            width1 : width,
        };
        rect
    }

    fn bottom_right(&self) -> (f32, f32) {
            (4.0, 4.0)
    }


    fn area(&self) -> f32 {
        self.width1*self.width1
    }

    fn perimeter(&self) -> f32 {
        self.width1*4.0
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}
