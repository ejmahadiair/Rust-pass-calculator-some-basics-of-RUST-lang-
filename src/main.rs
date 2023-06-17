#![allow(unused)]

use std::io;
fn gradeCalculator(mark: i32) -> String {
    let mut grade: String = String::new();
   if mark > 100 {
        grade = "There has no grade false mark enroled".to_string();
   }
   else if mark >= 80 {
        grade = "A+".to_string();
   }
   else if mark >= 75 {
        grade = "A".to_string();
   }
   else if mark >= 70 {
        grade = "B".to_string();
   }
   else if mark >= 60 {
        grade = "C".to_string();
   }
   else if mark >= 50 {
        grade = "D".to_string();
   }
   else if mark >= 40 {
        grade = "E".to_string();
   }else {
        grade = "F".to_string();
   }
grade
}
fn main() {
    println!("Please enter your age: ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Age needed!");

    let mut age: i32 = age.trim()
                    .parse()
                    .expect("Please enter your age");

    println!("Age is : {}", age);
    
    if age<18{
        println!("You are not eligible to admit in university");
    }else{
        println!("Please enter number of completed courses: ");
        let mut courses = String::new();
        io::stdin().read_line(&mut courses)
                    .expect("Courses needed!");
        let mut courses: i32 = courses.trim()
                        .parse()
                        .expect("Please enter number of completed courses!");
        println!("Courses: {}", courses);
        println!("Each course has 3 subjects..");
        match courses {
            1 =>{
                println!("You have just completed one course so you are in 1st semester");
            },
            2 => println!("You have just completed two courses so you are in 2nd semester"),
            3 => println!("You have just completed three courses so you are in 3rd semester"),
            4 => println!("You have just completed four courses so you are in 4th semester"),
            5 => println!("You have just completed five courses so you are in 5th semester"),
            _ => println!("Eathier your are not enrolled student nither your are in 1st semester")
        };


         println!("Do you like to know the grade of your each subjec? (Y/y)/(N/n)");
         let mut choose = String::new();
         io::stdin().read_line(&mut choose)
                    .expect("Choose needed!");
        let choose = choose.trim();
        println!("Thank you for choose {}", choose);
        let mut x = 'c';
        match choose {
            "Y" | "y"=> {
                let mut totalMark = 0;
                for i in 1..=3{
                    println!("Please Enter {}st subject mark",i);
                    let mut mark = String::new();
                    io::stdin().read_line(&mut mark).expect("mark needed!");
                    let mut mark: i32 = mark.trim().parse::<i32>().expect("Integer mark required");
                    println!("{}st subject mark is: {}",i,mark);
                    let grade = gradeCalculator(mark);
                    println!("You got {} in {}st subject",grade,i);
                    totalMark+=mark;
                }
                println!("Your Total Mark is: {}",totalMark);
                println!("Do you want to know your total grade?");
                let mut tChoose = String::new();
                io::stdin().read_line(&mut tChoose).expect("tChoose needed!");
                let tChoose = tChoose.trim();
                match tChoose {
                    "Y" | "y" => {
                        let totalMark = totalMark/3;
                        let grade = gradeCalculator(totalMark);
                        println!("Your total grade is: {}",grade);
                    },
                    _ => println!("See you again..."),
                }
                println!("Thank you for use pass Calculator!")
            },
            _ => println!("Thank you"),
        }


    }

}
