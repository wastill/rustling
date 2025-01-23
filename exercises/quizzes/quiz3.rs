// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.
// 本测验测试:
// -泛型
// -特质
//
// 一个虚构的魔法学校有一个新的成绩单生成系统
// 在生锈!目前，系统仅支持创建报告卡，其中
// 学生的成绩用数字表示 (例如1.0 -> 5.5)。然而，该
// 学校还发布按字母顺序排列的等级 (A -> F-)，并且需要能够
// 打印两种类型的报告卡!
//
// 在结构 'ReportCard' 和impl中进行必要的代码更改
// 块支持字母顺序的报告卡，除了数字的。

use std::fmt;

// TODO: Adjust the struct as described above.
struct ReportCard {
    grade: Grade,
    student_name: String,
    student_age: u8,
}

enum Grade {
    Numeric(f32),
    Alphabetical(String),
}

// TODO: Adjust the impl block as described above.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grade::Numeric(num) => write!(f, "{}", num),
            Grade::Alphabetical(letter) => write!(f, "{}", letter),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: Grade::Alphabetical("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
