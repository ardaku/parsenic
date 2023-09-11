use parsenic::{
    error::{OverflowError, Uleb128Error},
    Read as _, Reader, Write as _, Writer,
};

#[test]
fn basic_parsing() {
    const HELLO_WORLD: &str = "Hello, world!";

    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.bytes([0, 1, 2, 3, 4, 5, 6, 7]);
    writer.bytes([0, 1, 2, 3, 4, 5, 6, 7]);
    writer.uleb128(HELLO_WORLD.len());
    writer.str(HELLO_WORLD);
    writer.u8(b'\0');
    writer.i8(-1);
    writer.u8(255);

    let mut reader = Reader::new(&buffer);

    assert_eq!([0, 1, 2, 3, 4, 5, 6, 7], reader.slice(8).unwrap());
    assert_eq!([0, 1, 2, 3, 4, 5, 6, 7], reader.array().unwrap());
    assert_eq!(
        HELLO_WORLD.len(),
        reader.uleb128::<u8>().unwrap().try_into().unwrap(),
    );
    assert_eq!(HELLO_WORLD, reader.str(HELLO_WORLD.len()).unwrap());
    assert_eq!(b'\0', reader.u8().unwrap());
    assert_eq!(255, reader.u8().unwrap());
    assert_eq!(-1, reader.i8().unwrap());
    reader.end().unwrap();
}

#[test]
fn reader() {
    let buffer = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = Reader::new(&buffer);

    reader.reader(0).unwrap().end().unwrap();

    let mut one = reader.reader(1).unwrap();

    assert!(reader.end().is_err());
    assert_eq!(1, one.u8().unwrap());
    one.end().unwrap();

    let mut three = reader.reader(3).unwrap();

    assert!(reader.end().is_err());
    assert_eq!(2, three.u8().unwrap());
    assert_eq!(3, three.u8().unwrap());
    assert_eq!(4, three.u8().unwrap());
    three.end().unwrap();

    let mut four = reader.reader(4).unwrap();

    assert!(reader.end().is_ok());
    assert_eq!(5, four.u8().unwrap());
    assert_eq!(6, four.u8().unwrap());
    assert_eq!(7, four.u8().unwrap());
    assert_eq!(8, four.u8().unwrap());
    four.end().unwrap();
    four.end().unwrap();
    reader.end().unwrap();
    reader.reader(0).unwrap().end().unwrap();
    assert!(reader.reader(1).is_err());
}

#[test]
fn le_parsing() {
    use parsenic::le::{Read as _, Write as _};

    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.u16(4_235);
    writer.u32(800_000_000);
    writer.u64(10_999_999_999_551_561);
    writer.u128(1_000_000_999_999_999_551_561);
    writer.i16(-4_235);
    writer.i32(800_000_000);
    writer.i64(-10_999_999_999_551_561);
    writer.i128(1_000_000_999_999_999_551_561);

    let mut reader = Reader::new(&buffer);

    assert_eq!(reader.u16().unwrap(), 4_235);
    assert_eq!(reader.u32().unwrap(), 800_000_000);
    assert_eq!(reader.u64().unwrap(), 10_999_999_999_551_561);
    assert_eq!(reader.u128().unwrap(), 1_000_000_999_999_999_551_561);
    assert_eq!(reader.i16().unwrap(), -4_235);
    assert_eq!(reader.i32().unwrap(), 800_000_000);
    assert_eq!(reader.i64().unwrap(), -10_999_999_999_551_561);
    assert_eq!(reader.i128().unwrap(), 1_000_000_999_999_999_551_561);

    reader.end().unwrap();
}

#[test]
fn be_parsing() {
    use parsenic::be::{Read as _, Write as _};

    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.u16(4_235);
    writer.u32(800_000_000);
    writer.u64(10_000_999_999_999_551_561);
    writer.u128(1_000_000_999_999_999_551_561);
    writer.i16(-4_235);
    writer.i32(800_000_000);
    writer.i64(-10_999_999_999_551_561);
    writer.i128(1_000_000_999_999_999_551_561);

    let mut reader = Reader::new(&buffer);

    assert_eq!(reader.u16().unwrap(), 4_235);
    assert_eq!(reader.u32().unwrap(), 800_000_000);
    assert_eq!(reader.u64().unwrap(), 10_000_999_999_999_551_561);
    assert_eq!(reader.u128().unwrap(), 1_000_000_999_999_999_551_561);
    assert_eq!(reader.i16().unwrap(), -4_235);
    assert_eq!(reader.i32().unwrap(), 800_000_000);
    assert_eq!(reader.i64().unwrap(), -10_999_999_999_551_561);
    assert_eq!(reader.i128().unwrap(), 1_000_000_999_999_999_551_561);

    reader.end().unwrap();
}

#[test]
fn le_reading() {
    use parsenic::le::Read as _;

    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.bytes(4_235u16.to_le_bytes());
    writer.bytes(800_000_000u32.to_le_bytes());
    writer.bytes(10_999_999_999_551_561u64.to_le_bytes());
    writer.bytes(1_000_000_999_999_999_551_561u128.to_le_bytes());
    writer.bytes((-4_235i16).to_le_bytes());
    writer.bytes(800_000_000i32.to_le_bytes());
    writer.bytes((-10_999_999_999_551_561i64).to_le_bytes());
    writer.bytes(1_000_000_999_999_999_551_561i128.to_le_bytes());

    let mut reader = Reader::new(&buffer);

    assert_eq!(reader.u16().unwrap(), 4_235);
    assert_eq!(reader.u32().unwrap(), 800_000_000);
    assert_eq!(reader.u64().unwrap(), 10_999_999_999_551_561);
    assert_eq!(reader.u128().unwrap(), 1_000_000_999_999_999_551_561);
    assert_eq!(reader.i16().unwrap(), -4_235);
    assert_eq!(reader.i32().unwrap(), 800_000_000);
    assert_eq!(reader.i64().unwrap(), -10_999_999_999_551_561);
    assert_eq!(reader.i128().unwrap(), 1_000_000_999_999_999_551_561);

    reader.end().unwrap();
}

#[test]
fn be_reading() {
    use parsenic::be::Read as _;

    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.bytes(4_235u16.to_be_bytes());
    writer.bytes(800_000_000u32.to_be_bytes());
    writer.bytes(10_999_999_999_551_561u64.to_be_bytes());
    writer.bytes(1_000_000_999_999_999_551_561u128.to_be_bytes());
    writer.bytes((-4_235i16).to_be_bytes());
    writer.bytes(800_000_000i32.to_be_bytes());
    writer.bytes((-10_999_999_999_551_561i64).to_be_bytes());
    writer.bytes(1_000_000_999_999_999_551_561i128.to_be_bytes());

    let mut reader = Reader::new(&buffer);

    assert_eq!(reader.u16().unwrap(), 4_235);
    assert_eq!(reader.u32().unwrap(), 800_000_000);
    assert_eq!(reader.u64().unwrap(), 10_999_999_999_551_561);
    assert_eq!(reader.u128().unwrap(), 1_000_000_999_999_999_551_561);
    assert_eq!(reader.i16().unwrap(), -4_235);
    assert_eq!(reader.i32().unwrap(), 800_000_000);
    assert_eq!(reader.i64().unwrap(), -10_999_999_999_551_561);
    assert_eq!(reader.i128().unwrap(), 1_000_000_999_999_999_551_561);

    reader.end().unwrap();
}

#[test]
fn uleb128() {
    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    writer.uleb128(77u32);
    writer.uleb128(777u32);
    writer.uleb128(7777u32);
    writer.uleb128(77777u32);
    writer.uleb128(77777u32);

    let mut reader = Reader::new(&buffer);

    assert_eq!(reader.uleb128::<u8>().unwrap(), 77);
    assert_eq!(reader.uleb128::<u16>().unwrap(), 777);
    assert_eq!(reader.uleb128::<u16>().unwrap(), 7777);
    assert_eq!(reader.uleb128::<u32>().unwrap(), 77777);
    assert_eq!(
        reader.uleb128::<u16>().unwrap_err(),
        Uleb128Error::Overflow(OverflowError),
    );

    reader.end().unwrap();
}
