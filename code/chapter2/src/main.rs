fn main() {
    //ตัวอย่างการประกาศตัวแปรแบบ Immutable:
    let x = 5;
    println!("The value of x is: {}", x);

    //ตัวอย่างการประกาศตัวแปรแบบ Mutable:
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is now: {}", y);

    //ตัวอย่างการกำหนดชนิดข้อมูล:
    let z: i32 = 42; // กำหนดให้ตัวแปร z เป็นชนิด i32

    //ตัวอย่างการให้ Rust คิดชนิดข้อมูลให้:
    let name = "Alice"; // Rust ให้ name เป็น &str

    //ตัวอย่างการประกาศค่าคงที่ (Constants)
    const MAX_POINTS: u32 = 100_000;

    //ตัวอย่างการประกาศตัวแปรเงา (Shadowing)
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x); // ค่าของ x จะเป็น 12
}
