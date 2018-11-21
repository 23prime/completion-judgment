pub type CSV = Vec<Vec<String>>;

pub fn csv_reader(s: &str) -> CSV {
    let csv: CSV = s.split('\n')
                   .map(|x|
                       x.split(',')
                        .map(|y| y.trim_matches('"')
                        .trim().to_string())
                        .collect()
                   )
                   .collect();

    return csv;
}

#[allow(dead_code)]
pub fn csv_writer(csv: CSV) -> String {
    let v: Vec<String> =
        csv.iter()
           .map(|line| line.join(", "))
           .collect();
    let s = v.join("\n");

    return s;
}