# การติดตั้ง Rust บน macOS

ยินดีต้อนรับสู่การติดตั้ง Rust บน macOS! ขั้นตอนนี้จะพาไปดูวิธีการติดตั้ง Rust อย่างง่ายดาย เพื่อเริ่มต้นการพัฒนาโปรแกรมด้วย Rust บน macOS

## ขั้นตอนที่ 1: การติดตั้ง Homebrew (ถ้ายังไม่มี)

Homebrew เป็นตัวจัดการแพ็กเกจที่สะดวกมากสำหรับ macOS ถ้ายังไม่ได้ติดตั้ง Homebrew สามารถติดตั้งได้โดยการเปิด Terminal แล้วรันคำสั่งต่อไปนี้:

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

รอจนการติดตั้งเสร็จสิ้น หลังจากนั้นสามารถใช้ Homebrew ในการติดตั้งซอฟต์แวร์อื่น ๆ ได้ง่ายดาย

## ขั้นตอนที่ 2: การติดตั้ง rustup
เปิด Terminal แล้วรันคำสั่งนี้เพื่อดาวน์โหลดและติดตั้ง rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

หลังจากการติดตั้งเสร็จสิ้นแล้ว ปิด Terminal เดิมแล้วเปิดใหม่อีกครั้ง เพื่อให้การตั้งค่า Path ใหม่มีผล จากนั้นรันคำสั่งต่อไปนี้เพื่อตรวจสอบการติดตั้ง Rust:

```sh
rustc --version
```
ถ้าเห็นเวอร์ชันของ Rust แสดงขึ้นมา แสดงว่าติดตั้งสำเร็จแล้ว!

##  ขั้นตอนที่ 3: การตั้งค่า Visual Studio Code
หากต้องการใช้ Visual Studio Code (VS Code) เป็นเครื่องมือพัฒนา สามารถติดตั้งส่วนขยาย Rust เพื่อช่วยในการพัฒนาได้ง่ายขึ้น:

ดาวน์โหลดและติดตั้ง Visual Studio Code
เปิด VS Code และไปที่ Extensions (กด Ctrl+Shift+X)
ค้นหา "Rust" และติดตั้งส่วนขยายที่ชื่อว่า "rust-analyzer"

"หน้าถัดไป: การติดตั้ง Rust บน macOS"