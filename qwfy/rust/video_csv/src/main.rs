use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // 打开 CSV 文件
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);

    // 创建 CSV reader,并指定 CSV 文件有头部行
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    // 跳过第二行
    let _ = csv_reader.records().next();

    // 遍历剩余的 CSV 记录
    for result in csv_reader.records() {
        let record = result?;

        // 判断是否为空行
        if !record.iter().all(|field| field.trim().is_empty()) {
            // 只选择前 4 列
            let first_4_fields: Vec<&str> = record.iter().take(4).collect();

            // 处理前 4 列的数据
            println!("{:?}", first_4_fields);
        }
    }

    Ok(())
}