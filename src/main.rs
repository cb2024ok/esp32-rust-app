use std::time::Duration;

use embedded_graphics::style::PrimitiveStyleBuilder;
use sh1106::{prelude::*, Builder};

//use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::prelude::*;

// 0.6.2 버전용 임포트
use embedded_graphics::{
    fonts::{Font6x12, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, Rectangle}, // 도형 도구
    style::TextStyle,
};

//use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    let p = Peripherals::take().unwrap();
    let mut i2c = I2cDriver::new(
        p.i2c0,
        p.pins.gpio21,         // SDA
        p.pins.gpio22,         // SCL
        &I2cConfig::default(), //new().baudrate(Hertz(400_000).into()),
    )?;

    println!("Scanning I2C bus...");

    for addr in 0x00..=0x7F {
        let buf = [0u8];
        let result = i2c.write(addr, &buf, 1000);
        if result.is_ok() {
            println!("Found device at 0x{:02X}", addr);
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    // I2C 인터페이스 & 디스플레이 빌더
    // 주소는 0x3C가 기본입니다. (안 나오면 0x3D로 시도)
    let mut display: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c)
        .into();

    // 3. 디스플레이 초기화
    display
        .init()
        .map_err(|e| anyhow::anyhow!("Init error: {:?}", e))?;
    display.clear();

    // 4. 그리기 스타일 설정
    let text_style = TextStyle::new(Font6x12, BinaryColor::On);
    let text_off_style = TextStyle::new(Font6x12, BinaryColor::Off);

    // 텍스트 그리기
    /*
    Text::new("Rust SH1106 OK!", Point { x: 0, y: 16 })
        .into_styled(text_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("Draw error"))?;

    Text::new("ESP32-S3 Running", Point { x: 0, y: 40 })
        .into_styled(text_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("Draw error"))?;
    */
    // 1. 상단 타이틀 바 (역상 효과)
    // 화면 상단에 꽉 찬 직사각형을 그리고 그 위에 글자를 씁니다.
    let header_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build();

    // 2. 사각형 그리기 (Coord 대신 [x, y] 배열 사용)
    // Rectangle::new(좌측상단_좌표, 우측하단_좌표)
    Rectangle::new(Point { x: 0, y: 0 }, Point { x: 127, y: 13 })
        .into_styled(header_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("Header Draw Error"))?;

    Text::new(" SYSTEM MONITOR ", Point { x: 10, y: 2 })
        .into_styled(text_off_style) // 글자를 Off(검정)로 설정
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("text Draw Error"))?;

    // 2. 중앙 데이터 영역 (구분선과 아이콘 대용 문자)
    Text::new("> CPU: 42%", Point { x: 5, y: 25 })
        .into_styled(text_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("text Draw Error"))?;

    Text::new("> RAM: 1.2G", Point { x: 5, y: 40 })
        .into_styled(text_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("text Draw Error"))?;

    // 3. 하단 장식선
    let line_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    Line::new(Point { x: 0, y: 55 }, Point { x: 127, y: 55 })
        .into_styled(line_style)
        .draw(&mut display)
        .map_err(|_| anyhow::anyhow!("text Draw Error"))?;

    // 버퍼 전송
    display
        .flush()
        .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

    println!("Success: Display updated.");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
