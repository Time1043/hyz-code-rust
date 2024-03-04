enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // 用结构体存储枚举相关数据
    let home = IpAddr {
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };
}

fn route(ip_kind: IpAddrKind) {}
