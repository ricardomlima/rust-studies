enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));
}