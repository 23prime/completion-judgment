#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};

pub type Code      = String; //科目番号
pub type Title     = String; // 科目名
pub type CreditNum = f64;    // 単位数
pub type Grade     = String; // 評価
pub type Group     = String; // 科目群

#[derive(PartialEq, Clone)]
// Display の auto derive ができないっぽい
pub struct Credit {
     pub code:  Code,
     pub title: Title,
     pub num:   CreditNum,
     pub grade: Grade,
     pub group: Group
}

impl Display for Credit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Credits {{\n  code: {},\n  title: {},\n  num: {},\n  grade: {},\n  group: {}\n}}",
            self.code, self.title, self.num, self.grade, self.group
        )
    }
}


pub type Credits = Vec<Credit>;
