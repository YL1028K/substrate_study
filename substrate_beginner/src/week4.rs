// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
trait Light {
    fn time(&self) -> u8;
}

impl Light for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 20,
        }
    }
}

//实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn sum_u32(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut overflow: bool = false;
    for v in arr.iter() {
        match sum.checked_add(*v) {
            Some(res) => {
                sum = res;
            }
            None => {
                overflow = true;
                break;
            }
        }
    }
    if overflow {
        return None;
    }
    return Some(sum);
}

// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

// 定义计算面积函数
trait ShapeCaculate {
    fn area(&self) -> f64;
}

fn cal_area<T: ShapeCaculate>(shape: T) -> f64 {
    return shape.area();
}
// 定义支持的图形，并使用 Enum 保存
struct Square {
    length: f64,
    width: f64,
}
impl ShapeCaculate for Square {
    fn area(&self) -> f64 {
        return self.length * self.width;
    }
}

struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl ShapeCaculate for Triangle {
    fn area(&self) -> f64 {
        let s: f64 = (self.side_a + self.side_b + self.side_c) / 2.0;
        return (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt();
    }
}

struct Circle {
    radius: f64,
}
impl ShapeCaculate for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traffic_light() {
        let l = TrafficLight::Green;
        assert_eq!(l.time(), 20);
        let l = TrafficLight::Red;
        assert_eq!(l.time(), 30);
        let l = TrafficLight::Yellow;
        assert_eq!(l.time(), 3);
    }
    #[test]
    fn test_sum_u32() {
        let arr: [u32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&arr), Some(15));
        let arr: [u32; 5] = [u32::MAX, 2, 3, 4, 5];
        assert_eq!(sum_u32(&arr), None);
    }

    #[test]
    fn test_shape_caculate_area() {
        let c = Circle { radius: 3.0 };
        assert_eq!(cal_area(c), 28.274333882308138);
        let s = Square {
            length: (3.0),
            width: (4.0),
        };
        assert_eq!(cal_area(s), 12.0);
        let t = Triangle {
            side_a: (3.0),
            side_b: (4.0),
            side_c: (5.0),
        };
        assert_eq!(cal_area(t), 6.0);
    }
}
