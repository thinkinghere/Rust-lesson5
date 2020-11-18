#[derive(Debug)]
// 枚举信号灯 Red Green Yellow
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// 定义信号灯的时间
trait Light {
    fn time(&self) -> i32;
}

impl Light for TrafficLight{
    fn time(&self) -> i32 {
        // 模式匹配
        return match self {
            TrafficLight::Red => 30,  // 红灯30s
            TrafficLight::Yellow => 5, // 黄灯5s
            TrafficLight::Green => 45, // 绿灯45s
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("If traffic light is {:?},it will {:?} seconds",red, red.time());
    println!("If traffic light is {:?},it will {:?} seconds",green, green.time());
    println!("If traffic light is {:?},it will {:?} seconds",yellow, yellow.time());
}
