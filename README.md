# completion-judgment

## About

This is Completion Judgment Machine.

Judge whether your credits are enough to complete graduate or not.

:warning: Not judging you can write ShuRon or not.

## Usage

- Download `seiseki.csv` from [Twins](https://twins.tsukuba.ac.jp/campusweb/).
- And, type following.

```
$ cargo run ./seiseki.csv
```

- If completion requirement is changed, edit `src/rules.rs`.


## About CSV data

| Row | Contents   |
|-----|------------|
|  0  | 学籍番号   |
|  1  | 氏名       |
|  2  | 科目番号   |
|  3  | 科目コード |
|  4  | 科目名     |
|  5  | 単位数     |
|  6  | 総合評価   |
|  7  | 認定年度   |
|  8  | 科目区分   |
|  9  | 認定学期   |

- Now, ignoring "科目コード" and judging from "科目番号".
- English ver. has different specification from Japanese, so now not supported.
- If specification of CSV changed, then edit `mk_credit` in `src/rules.rs`.
