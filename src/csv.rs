pub type CSV = Vec<Vec<String>>;

pub fn csv_reader(s: &str) -> CSV {
    let csv = s.split('\n').map(|x| x.split(',').map(|y| y.trim_matches('"').trim().to_string()).collect()).collect::<CSV>();

    return csv;
}

pub fn csv_writer(csv: CSV) -> String {
    let v = csv.iter().map(|line| line.join(", ")).collect::<Vec<String>>();
    let s = v.join("\n");

    return s;
}