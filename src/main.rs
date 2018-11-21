#![allow(dead_code, unused_imports, unused_variables)]

mod csv;
mod rules;
mod types;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use csv::*;
use rules::*;
use types::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename          = &args[1];
    let mut f             = File::open(filename).expect("file not found");
    let mut contents      = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let cds = parse_csv(&contents);
    let req = require();

    println!("[修了要件]");

    for (grp, cdn) in &req {
        print_group_num(grp.clone(), cdn.clone());
    }
    
    let sum_req: f64 = req.iter().map(|(grp, cdn)| cdn.clone()).sum();
    println!("計: {} 単位", sum_req);

    println!("\n[あなたの修得した単位]");

    let grps = group_list(&req);
    for grp in grps {
        print_group_num2(&cds, grp);
    }

    let sum_cdn = count_credit_num(&cds);
    println!("計: {} 単位", sum_cdn);

    let rlt = get_result(&cds, &req);
    println!("\n{}", rlt);

    println!("\n不足:");

    let shorts = short_list(judge_list(&cds, &req));
    if shorts.is_empty() {
        println!("無し！");
    } else {
        for (grp, cdn) in shorts {
            print_group_num(grp, cdn);
        }
    }
}

fn print_group_num(grp: Group, cdn: CreditNum) -> () {
    println!("{}: {} 単位", grp, cdn);
}

fn print_group_num2(cds: &Credits, grp: Group) -> () {
    let fcds  = filter_group(cds.clone(), &grp);
    let count = count_credit_num(&fcds);

    println!("{}: {} 単位", grp, count);
}

fn parse_csv(s: &str) -> Credits {
    let csv        = csv_reader(s);
    let &ref terms = &csv[0];
    let datas      = &csv[1..csv.len() - 1];
    let cds        = datas.iter().map(|ds| mk_credit(terms, ds)).collect();
    
    return cds;
}


// Count available number of Credit
// isAvailable :: Grade -> Bool
fn is_available(grd: &Grade) -> bool {
    let eval = match &grd[..] {
        "A+" => true,
        "A"  => true,
        "B"  => true,
        "C"  => true,
        "P"  => true,
        _    => false
    };
    
    return eval;
}

fn available_credit_num(cd: &Credit) -> CreditNum {
    if is_available(&cd.grade) {
        return cd.num;
    } else {
        return 0.0;
    }
}

fn count_credit_num(cds: &Credits) -> CreditNum {
    let cdn: f64 = cds.iter().map(|&ref cd| available_credit_num(cd)).sum();

    return cdn;
}

// Judge
fn filter_group(mut cds: Credits, grp: &Group) -> Credits {
    cds.retain(|cd| &cd.group == grp);

    return cds;
}

// Difference between Yours and Required in a Gruop:
//   <Required Number of Credits> - <Yours>
fn difference (cds: Credits, grp: &Group, req: Require) -> (&Group, CreditNum) {
    let cds     = filter_group(cds, &grp);
    let req_num = req[grp];
        // Using get: -> Some(Group)
        // let req = req.get(&grp); 
    let num     = count_credit_num(&cds);

    return (grp, req_num - num);
}

// Make all Group list.
fn group_list(req: &Require) -> Vec<Group> {
    let grps: Vec<Group> = req.iter().map(|(k, _)| k.clone()).collect();

    return grps;
}

// Check difference in a Group.
fn judge_group(cds: Credits, grp: Group, req: Require) -> (Group, bool, CreditNum) {
    let (_, diff) = difference(cds, &grp, req);

    if diff > 0.0 {
        return (grp, false,  diff);
    } else {
        return (grp, true,  diff);
    }
}

// Take judgeGroup to each Gruops.
// And make list which have result of judgeGroup.
fn judge_list(cds: &Credits, req: &Require) -> Vec<(Group, bool, CreditNum)> {
    let grps = group_list(&req);
    let jv   = grps.iter().map(|&ref grp| judge_group(cds.clone(), grp.clone(), req.clone())).collect();

    return jv;
}

// judgeList have only True; You have enough Credits in all Groups.
// Then True.
fn judge(cds: &Credits, req: &Require) -> bool {
    let jv = judge_list(cds, req);
    let jdg = jv.iter().map(|(_, b, _)| b.clone()).all(|b| b);

    return jdg;
}

// Result whether shuryo or not.
fn get_result(cds: &Credits, req: &Require) -> String {
    let rlt = if judge(cds, req) {
        "修了です．おめでとう🎉🎉🎉"
    } else {
        "留年！ｗ"
    };

    return "結果: ".to_string() + rlt;
}

// If exist Groups which have short of Credits, make there's list.
fn short_list(mut gbcs: Vec<(Group, bool, CreditNum)>) -> Vec<(Group, CreditNum)> {
    gbcs.retain(|(g, b, c)| !b);
    let gcs = gbcs.iter().map(|(g, _, c)| (g.clone(), c.clone())).collect();

    return gcs;
}
