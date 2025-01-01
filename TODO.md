- Dự án về cái gì: Một ứng dụng từ điển trên web được xây dựng bằng thuần HTML/CSS/JS. Cho phép người dùng tra từ tiếng việt và trả ra nghĩa tiếng anh tương ứng + 10 câu ví dụ của từ.
- MVP: Một ứng dụng hiển thị trên web, có thể tra từ và trả ra nghĩa từ + 10 câu ví dụ
- sprinkles: Không có
- Khi nào dự án sẽ hoàn thành: Sau khi xong hết các nhiệm vụ MVP (Cuối tháng 12 - 31/12)


## Quy trình làm việc

### TODO
- Chức năng tìm kiếm từ - Trả kết quả dưới dạng JSON từ API - MVP 
- Đọc dữ liệu từ API - MVP
- Xử lý Input đầu vào từ người dùng - Thiết kế thanh tìm kiếm - MVP
- Hiển thị từ từ JSON - MVP
- Bảng hiển thị nghĩa của từ - MVP
- Thêm Audio lấy từ Forvo - sprinkles

### DOING


### DONE

### BUGS/IDK HOW TO FIX

## Phần mềm xây dựng database cho từ điển - viết bằng Rust
Định nghĩa: Là một phần mềm được viết bằng Rust (Sử dụng Library cho JSON Reader và Processor) để xử lý dữ liệu từ từ điển từ vựng + sentence dataset.
MVP: một phần mềm xử lý các phần dữ liệu cho sẵn và gộp nó vào rồi chia ra theo bảng chữ cái -> API (Mỗi một glossary bao gồm một từ tiếng việt: định nghĩa Tiếng Anh và 10 câu ví dụ)
Nice to have: không có
Bao giờ xong: chưa biết

Functions:
- Đọc dữ liệu JSON từ từ điển từ vựng -> Đọc tất luôn 
- Tìm câu ví dụ cho từng từ -> input từ -> output: &str chứa một json object -> Đổi thành một kiểu data riêng -> create_individual_glossary(word: &str, definiton: &str, sentences: &Vec<String>)
- Chia glossary theo chữ cái đầu tiên - API Generator #1 (json từ -> ghi vào tệp tương ứng theo bảng chữ cái) -> Tạo folder cho api, tạo từng tệp một dựa trên phần phân loại bảng chữ cái (tạo thành một function riêng) -> filter_character(first_char: &str) -> &str (the file name)
- Đọc từ kho dữ liệu câu để tìm câu ví dụ () -> Đọc bằng BufReader, và nếu tìm được đủ số dòng thì dừng loop -> find_example_sentences(word: &str) -> Vec<String>

inside the loop of glossary, we will:

add_glossaries_to_api() -> anyhow::Result<()>


1. từ được loop đến sẽ được build thành một String và ghi thẳng vào trong tệp data mới luôn
2. 1 function xử lý câu, 



### HELPER

```rust
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    let path = "...";
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("unable to open file");
    let mut f = BufWriter::new(f);

    for i in 1..100 {
        write!(f, "{}", i).expect("unable to write");
    }

}

```

```rust
// BufRead::lines makes it easy to read a file one line at a time:

use std::io::{self, BufRead};
let mut lock = io::stdin().lock();
for line in lock.lines() {
    process(&line?);
}
```
```rust
// An alternative is to use a workhorse String in a loop over BufRead::read_line:

use std::io::{self, BufRead};
let mut lock = io::stdin().lock();
let mut line = String::new();
while lock.read_line(&mut line)? != 0 {
    process(&line);
    line.clear();
}
```

```rust
pub fn get_file_content(file_name: &str) -> Result<Vec<Line>> {
    let file_handler =
        File::open(file_name).context(format!("Could not read from file: {file_name}"))?;
    let reader = BufReader::new(file_handler);

    let lines = reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(line_number, line)| Line::new(&line, line_number.saturating_add(1)))
        .collect();

    Ok(lines)
}
```