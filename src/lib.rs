use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
// use std::collections::Vec;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Student {
    admission: String,
    name: String,
    year: String,
    course: String,
    attendance: Vector<Attendance>,
}
 
 
 
impl Default for Student {
    fn default() -> Self {
        Student {
            admission: "".to_string(),
            name: "".to_string(),
            year: "".to_string(),
            course: "".to_string(),
            attendance: Vector::new(b"r".to_vec()),
        }
    }
}
 
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Attendance {
    date: String,
    attended: String,
}
 
#[derive( BorshDeserialize, BorshSerialize,  Debug)]
pub struct Course {
    course_id: u32,
    course_name: String,
    units: Vector<String>,
}
 
impl Default for Course {
    fn default() -> Self {
        Course {
            course_id: 0,
            course_name: "".to_string(),
            units: Vector::new(b"r".to_vec()),
        }
    }
}
 
#[near_bindgen]
pub struct Contract {
    // SETUP CONTRACT STATE
    students: Vector<Student>,
    courses: Vector<Course>,
}
 
#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new() -> Self {
        let students: Vector<Student> = Vector::new(b"r".to_vec());
        let courses: Vector<Course> = Vector::new(b"r".to_vec());
 
        Contract { students, courses }
    }
 
    // add course
    pub fn add_course(&mut self, course_name: String) {
        let id = self.courses.len() as u32;
        let new_course = Course {
            course_id: id,
            course_name: course_name.to_string(),
            units: Vector::new(b"r".to_vec()),
        };
        self.courses.push(&new_course);
        env::log_str("course added successfully");
    }
 
    pub fn get_course_length(&self) -> u64 {
        self.courses.len()
    }
 
    // add student
    pub fn add_student(
        &mut self,
        admission: String,
        name: String,
        year: String,
        student_course: String,
    ) {
        let courses = &mut self.courses;
        let mut counter = 0;
        courses.iter().for_each(|course| {
            if course.course_name == student_course {
                counter += 1;
            }
        });
 
        env::log_str(format!("====>> Counter {}",counter).as_str());
 
        if counter > 0 {
            let student1 = Student {
                admission: admission.to_string(),
                name: name.to_string(),
                year: year.to_string(),
                course: student_course.to_string(),
                attendance: Vector::new(b"r".to_vec()),
            };
            self.students.push(&student1);
            env::log_str("student added successfully");
        } else {
            env::log_str("course not found")
        }
    }
    pub fn get_stude_length(&self) -> u64 {
        self.students.len()
    }
 
    pub fn get_student(&self, admission: String) -> Option<Student> {
 
        let me: &Vector<Student> = &self.students;
        let todo = me.iter().find(|tod| tod.admission == admission);
        return todo;
 
        // let students: Vec<Student> = self
        //     .students
        //     .iter()
        //     .filter(|f| f.admission == admission)
        //     .collect();
 
        // let a_student = students.clone();
        // return a_student.get(0);
        // return a_student;
    }
 
    // monitor attendance
    pub fn monitor_attendance(&mut self, admission: String, date: String, attended: String) {
        // let  students = &mut self.students;
        // let mut student_count = 0;
        
        // students.iter().for_each(|student| {
        //     if student.admission == admission {
        //         // let a=self.students.get(index)
        //         student_count += 1;
                // let new_attendance = Attendance {
                //     date: date.to_string(),
                //     attended: attended.to_string(),
                // };
 
                // student.attendance.push(&new_attendance);
                // env::log_str("student attendance recorded successfully");
        //     }
        // });
 
        // if student_count < 1 {
        //     env::log_str("admission not found");
        // }
 
        let mut student_count = 0;
        for (pos, mut student) in self.students.iter().enumerate() {
            println!("Element at position {}: {:?}", pos, student);
            if student.admission == admission {
                student_count += 1;
                let new_attendance = Attendance {
                    date: date.to_string(),
                    attended: attended.to_string(),
                };
                student.attendance.push(&new_attendance);
                env::log_str("student attendance recorded successfully");
            }
        }
 
        if student_count < 1 {
            env::log_str("admission not found");
        }
    }
}
 
/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
 
// use the attribute below for unit tests
#[cfg(test)]
 
mod tests {
    use super::*;
 
 
    
 
    // TESTS HERE
    #[test]
    fn add_course() {
       
 
        let mut course1 = Contract::new();
        course1.add_course("IT".to_string());
        assert_eq!(course1.get_course_length(), 1);
    }
    #[test]
 
    fn add_student() {
      
        let mut course1 = Contract::new();
        course1.add_course("ICT".to_string());
        course1.add_student(
            "tmc/0074/019".to_string(),
            "moses".to_string(),
            "2022".to_string(),
            "ICT".to_string(),
        );
        assert_eq!(course1.get_stude_length(), 1)
    }
    #[test]
    #[should_panic]
    fn monitor_attendance() {
      
        let mut course1 = Contract::new();
        course1.add_course("ICT".to_string());
 
        course1.add_student(
            "tmc/0074/019".to_string(),
            "moses".to_string(),
            "2022".to_string(),
            "ICT".to_string(),
        );
 
        course1.monitor_attendance(
            "tmc/0074/019".to_string(),
            "2022".to_string(),
            "ICT".to_string(),
        );
 
         match  course1.students.get(0) {
            Some(v)=>{
                assert_eq!(v.attendance.len(), 1);
            }
            None=>{
                panic!("student not found")
            }
        };
        
    }
}
