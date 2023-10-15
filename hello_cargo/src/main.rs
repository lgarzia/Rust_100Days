#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("rect1 is {:?}", rect1.area());

    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}