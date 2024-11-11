struct Student {
    name: String,
    career: u16,
    final_grade: u8,
    semester: u8,
}
impl Student {
    fn display(&self) -> String {
        format!(
            "Nombre: {}, Codigo de carrera: {}, Nota final: {}, Semestre: {}",
            self.name, self.career, self.final_grade, self.semester
        )
    }
}
fn main() {
    let std1 = Student {
        name: String::from("Alexis"),
        career: 550,
        final_grade: 100,
        semester: 6,
    };

    let std2 = Student {
        name: String::from("Pepe"),
        final_grade: 85,
        ..std1
    };

    let std3 = Student {
        name: String::from("Juan"),
        final_grade: 60,
        ..std1
    };

    let students: [Student; 3] = [std1, std2, std3];

    for student in &students {
        println!("{}", student.display())
    }
}
