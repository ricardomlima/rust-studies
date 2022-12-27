enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    /*
     * Here we use the Option enum to say that we can either return an f64
     * or None
     */
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddrEnum::V4(127, 0, 0, 1);
    let _loopback = IpAddrEnum::V6(String::from("::1"));

    // using the built-in Option enum
    let mut _absent_number: Option<f64> = None;
    _absent_number = divide(10.0, 5.0);

    // This is how we assign when using Optional enum
    // using the Some variant
    let _absent_variable: Option<String> = Some(String::from("This is a string"));
}