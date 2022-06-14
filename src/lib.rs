use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")]
pub struct Student{
    admission: String,
    name: String,
    year: String,
    course: String,
    attendance: Vec<Attendance>,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")]
pub struct Attendance {
    date: String,
    attended: String,
}


#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")]
pub struct Course{
    courseId: u32,
    course_name: String,
    units: Vec<String>,
}

#[near_bindgen]
pub struct Contract {
    // SETUP CONTRACT STATE
    students: Vec<Student>,
    courses: Vec<Course>,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new() -> Self{
        let students: Vec<Student> = Vec::new(); // vec![];
        let courses: Vec<Course> = Vec::new();

        Contract {
            students,
            courses,
        }
    }

    // add course
    pub fn add_course(&mut self, course_name: String){
        let id = self.courses.len() as u32;
        let new_course = Course {
            courseId: id,
            course_name: course_name.to_string(),
            units: vec![],
        };
        self.courses.push(new_course);
        env::log(b"course added successfully");
    }

    pub fn get_course_length(&self)->usize{
        self.courses.len()
    }

    // add student
    pub fn add_student(&mut self, admission: String, name: String, year: String, student_course: String){
        let courses = &mut self.courses;
        let mut counter = 0;
        courses.into_iter().for_each(|course|{
            if course.course_name == student_course{
                counter += 1;
            }
        });

        if counter > 0{
            let student1 = Student {
                    admission: admission.to_string(),
                    name: name.to_string(),
                    year: year.to_string(),
                    course: student_course.to_string(),
                    attendance: vec![],
                };
            self.students.push(student1);
            env::log_str("student added successfully");
        }else{
            env::log_str("course not found")
        }
    }   

    // monitor attendance
    pub fn monitor_attendance(&mut self, admission: String, date: String, attended: String){
        let students = &mut self.students;
        let mut student_count = 0;
        students.into_iter().for_each(|student|{
            if student.admission == admission{
                student_count += 1;
                let new_attendance = Attendance {
                    date: date.to_string(), 
                    attended: attended.to_string(),
                };

                student.attendance.push(new_attendance);
                env::log_str("student attendance recorded successfully");
            }
        });

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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn add_course(){
        let mose = AccountId::new_unchecked("moses.testnet".to_string());
        let context = get_context(mose.clone());
        testing_env!(context.build());

        let mut course1 = Contract::new();
        course1.add_course("IT".to_string());
        assert_eq!(course1.get_course_length(), 1);

    }
}

