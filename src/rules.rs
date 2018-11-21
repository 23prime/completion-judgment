use types::*;

use std::collections::HashMap;

pub type Require = HashMap<Group, CreditNum>;

// Rules
pub fn require() -> Require {
    let mut req = HashMap::new();;
    req.insert("基礎科目".to_string(), 1.0);
    req.insert("専攻共通 必修".to_string(), 1.0);
    req.insert("専攻共通 選択".to_string(), 4.0);
    req.insert("教科教育（数学教育）".to_string(), 6.0);
    req.insert("教科専門（数学）".to_string(), 12.0);
    req.insert("教科選択（研究）".to_string(), 6.0);
    req.insert("その他".to_string(), 0.0);
    
    return req;
}

pub fn mk_group(cd: &Code) -> Group {
    let (f, a)        = cd.split_at(3);
    let (b, cs)       = a.split_at(1);
    let cs: Vec<char> = cs.chars().collect();
    let c             = cs[0];

    if f != "01B" {
        return "その他".to_string();
    } else if a == "1001" {
         return "基礎科目".to_string();
    } else if a == "1011" {
         return "専攻共通 必修".to_string();
    } else if b == "2" {
         return "専攻共通 選択".to_string();
    } else if b != "6" {
         return "その他".to_string();
    } else if c == '1' {
         return "教科教育（数学教育）".to_string();
    } else if c == '5' {
         return "教科選択（研究）".to_string();
    } else {
        return "教科専門（数学）".to_string();
    }
}


// Parse Credits
fn find_term_index (s: &str, ss: &Vec<String>) -> usize  {
    let idx = ss.iter().position(|&ref ssi| ssi == &s);

    match idx {
        Some(i) => return i,
        None => panic!("Index not found")
    }
}

pub fn mk_credit(ss: &Vec<String>, ds: &Vec<String>) -> Credit {
    let cd    = &ds[find_term_index("科目番号", &ss)];
    let title = &ds[find_term_index("科目名", &ss)];
    let grade = &ds[find_term_index("総合評価", &ss)];

    return Credit {
        code:  cd.to_string(),
        title: title.to_string(),
        num:   ds[find_term_index("単位数", &ss)].parse().unwrap(),
        grade: grade.to_string(),
        group: mk_group(cd)
    }
}
