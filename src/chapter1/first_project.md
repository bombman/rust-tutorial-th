ยินดีต้อนรับสู่โปรเจ็กต์แรกของคุณกับ Rust! ในบทนี้ เราจะมาดูวิธีการสร้างและรันโปรเจ็กต์แรกกัน เริ่มกันเลย!

## ขั้นตอนที่ 1: การสร้างโปรเจ็กต์ใหม่

1. เปิด Terminal (macOS) หรือ PowerShell/Command Prompt (Windows)
2. รันคำสั่งนี้เพื่อสร้างโปรเจ็กต์ใหม่:

   ```sh
   cargo new hello_rust
   cd hello_rust
   ```
โครงสร้างโปรเจ็กต์
หลังจากสร้างโปรเจ็กต์ใหม่ด้วย cargo new จะเห็นโครงสร้างไฟล์ดังนี้:
  
```rust
hello_rust
├── Cargo.toml
└── src
    └── main.rs
```

Cargo.toml: ไฟล์นี้ใช้สำหรับการจัดการ dependencies และการตั้งค่าโปรเจ็กต์
src/main.rs: ไฟล์นี้เป็นจุดเริ่มต้นของโปรแกรม Rust

แก้ไข main.rs
เปิดไฟล์ src/main.rs จะเห็นโค้ดเริ่มต้นดังนี้:

 ```rust
fn main() {
    println!("Hello, world!");
}
```
โค้ดนี้จะพิมพ์ข้อความ "Hello, world!" ออกมาบนหน้าจอเมื่อรันโปรแกรม

## ขั้นตอนที่ 2: การรันโปรเจ็กต์
1 เพื่อรันโปรเจ็กต์ของคุณ รันคำสั่งต่อไปนี้ใน Terminal หรือ PowerShell/Command Prompt:

```sh
cargo run
```
2 ควรเห็นผลลัพธ์ดังนี้:

```sh
Compiling hello_rust v0.1.0 (file:///path/to/hello_rust)
 Finished dev [unoptimized + debuginfo] target(s) in 1.23s
  Running `target/debug/hello_rust`
Hello, world!
```  
โปรแกรมได้รันสำเร็จและพิมพ์ข้อความ "Hello, world!" ออกมาบนหน้าจอ
## การเปลี่ยนแปลงข้อความ
ลองแก้ไขไฟล์ src/main.rs เพื่อพิมพ์ข้อความอื่น:
```rust
fn main() {
    println!("Hello, Rust!");
}
```
บันทึกการเปลี่ยนแปลงแล้วรันโปรแกรมอีกครั้งด้วยคำสั่ง cargo run ควรเห็นข้อความ "Hello, Rust!" แสดงขึ้นมาแทน
## สรุป
ในบทนี้ คุณได้เรียนรู้วิธีการสร้างโปรเจ็กต์ Rust ใหม่และรันโปรแกรมเบื้องต้น ตอนนี้พร้อมที่จะสำรวจและเรียนรู้เพิ่มเติมเกี่ยวกับ Rust แล้ว

