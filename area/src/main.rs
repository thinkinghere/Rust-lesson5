// 定义一个公共HasArea trait
pub trait HasArea {
    type Output;
    fn get_area(&self) -> Self::Output;
}

// 正方形结构体
struct Square<T> {
    side: T,
}

// 圆形结构体
struct Circle<T> {
    radius: T,
}

// 三角形结构体
struct Triangle<T> {
    base: T,
    hight: T,
}

// 定义正方形Output类型  get_area 方法
impl<T: std::ops::Mul<Output = T> + Copy> HasArea for Square<T> {
    type Output = T;
    
    fn get_area(&self) -> Self::Output {
        self.side * self.side // 边长的平方
    }
}

// 定义圆形Output类型  get_area 方法
impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Circle<T> {
    type Output = f64;
    
    fn get_area(&self) -> Self::Output {
        (self.radius * self.radius).into() * std::f64::consts::PI  //  πr^2
    }
}

// 定义三角形Output类型  get_area 方法
impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Triangle<T> {
    type Output = f64;
    
    fn get_area(&self) -> Self::Output {
        (self.base * self.hight).into() * 0.5 // 1/2 * 底* 高
    }
}

fn main() {

    let s = Square {side: 10};
    println!("square: {}", s.get_area());
    
    let r = Circle {radius: 2};
    println!("circle: {}", r.get_area());

    let t = Triangle {base: 2, hight: 1};
    println!("Triangle: {}", t.get_area());
}