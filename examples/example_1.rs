use gmath_rs::vector::Vector;

#[allow(non_snake_case)]
fn aV(a: f64, v: Vector<f64>) -> Vector<f64> {
    v * a
}

fn main() {
    let p1 = Vector::new2((1.0, 0.5));
    let p2 = Vector::new2((2.4, 3.9));
    let p3 = Vector::new2((1.0, 0.5 - 1.0E-16));
    // ここで桁落ち誤差に入る模様
    let p4 = Vector::new2((1.0, 0.5 - 1.0E-17));

    println!("加算:{} + {} -> {}", p1, p2, p1 + p2);
    println!("減算:{} - {} -> {}", p1, p2, p1 - p2);
    println!("外積:{} * {} -> {}", p1, p2, (p1 * p2).v().2);
    println!("外積:{} * {} -> {}", p2, p1, (p2 * p1).v().2);
    println!("内積:({} . {} -> {})", p1, p2, p1.dot(p2));
    println!("内積:({} . {} -> {})", p2, p1, p2.dot(p1));

    println!("大きさ^2:({} . {} -> {})", p1, p1, p1.dot(p1));

    println!("右項実数積:{} * {} -> {}", p1, 1.5, p1 * 1.5);
    println!(
        "右項実数積(反復):{} * {} * {} -> {}",
        p1,
        1.5,
        1.1,
        p1 * 1.5 * 1.1
    );

    println!("関数:{} * {} + {} -> {}", 1.5, p1, p2, aV(1.5, p1) + p2);
    println!("メソッド:{} * {} + {} -> {}", 1.5, p1, p2, 1.5 * p1 + p2);

    println!("比較:{} == {} -> {}", p1, p2, p1 == p2);
    println!("比較:{} != {} -> {}", p1, p2, p1 != p2);
    println!("比較:{} == {} -> {}", p1, p1, p1 == p1);

    println!("E-16");
    println!("比較(桁落ち確認):{} == {} -> {}", p1, p3, p1 == p3);
    println!("E-17");
    println!("比較(桁落ち確認):{} == {} -> {}", p1, p4, p1 == p4);
}
