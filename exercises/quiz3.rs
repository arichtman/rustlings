// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
use std::marker::PhantomData;

// TypeState pattern
pub struct AlphaReport {}
pub struct NumericReport {}

// Generic struct implementation
pub struct ReportCard<T> {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    // use PhantomType bound to the generic type
    report_type: PhantomData<T>,
}

// plain implementation for numeric
impl ReportCard<NumericReport> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

// Jazzier implementation for lettered reports
// Could probably do without the separate method but eh, it's clearer
impl ReportCard<AlphaReport> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            &self.get_alpha_grade()
        )
    }

    // The static lifetime here smells a bit
    // Maybe the solution was to do an assignment and then to_owned it or somehow pass ownership out of this method
    fn get_alpha_grade(&self) -> &'static str {
        // #![feature(exclusive_range_pattern)]
        // #![cfg_attr(feature = "my_feature", feature(exclusive_range_pattern))]
        match self.grade {
            // Little bit verbose, we could have maybe made this less repetitive
            // Also the spec seems off, it says the range is 1 - 5.5 but there's no way to fit minuses in there, yet there's an F-?
            // Also oof but I gotta put decimals on everything else be even more verbose let they be considered integers
            // I have given up for now on getting exclusive range patterns working.
            0.0..=0.5 => "F",
            0.5..=1.0 => "F+",
            1.0..=1.5 => "D",
            1.5..=2.0 => "D+",
            2.0..=2.5 => "C",
            2.5..=3.0 => "C+",
            3.5..=4.0 => "B",
            4.0..=4.5 => "B+",
            4.5..=5.0 => "A",
            5.0..=5.5 => "A+",
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard::<NumericReport> {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            report_type: PhantomData,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard::<AlphaReport> {
            grade: 5.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            report_type: PhantomData,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
