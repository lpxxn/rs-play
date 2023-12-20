use std::{env};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("current path: {}", env::current_dir().unwrap().to_str().unwrap());
    let mut f = File::open("./example/test.md").await?;
    let mut buffer = [0; 5];
    // 由于buffer的长度限制，当次的read 调用最多可以从文件中读取10个字节的数据
    let n = f.read(&mut buffer).await?;
    println!("The Bytes: {:?}", &buffer[..n]);


    // 上面代码很清晰，但是大家可能会疑惑 b"some bytes" 是什么意思。这种写法可以将一个 &str 字符串转变成一个字节数组：&[u8;10]，然后 write 方法又会将这个 &[u8;10] 的数组类型隐式强转为数组切片: &[u8]。
    let n = f.write(b"hello world abc").await?;
    println!("Write {} bytes", n);
    write_file().await?;
    // read_to_end
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    // convert the buffer to a string
    let s = String::from_utf8(buffer).unwrap();
    println!("The Bytes: {:?}", s);
    Ok(())
}

async fn write_file() -> io::Result<()> {
    let mut f = File::create("./example/test.md").await?;
    f.write_all(b"write all alphabet\n").await?;
    let mut reader: &[u8] = b"some bytes";
    io::copy(&mut reader, &mut f).await?;
    Ok(())
}