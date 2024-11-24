
//HackerRank #1
// fn firstEx() {
//     let array  = [1,2,3,4,5,6];
//     let mut sum = 0;
//     for i in array {
//         sum += i;
//     }
//     println!("Sum:{}", sum)
// }

// #2
// use std::io::{self};
// fn main() {
//     let mut Alice = Vec::new();
//     let mut Bob = Vec::new();

//     println!("Alice - enter num:");
//     let mut Alice_num = String::new();
//     io::stdin()
//         .read_line(&mut Alice_num)
//         .expect("failed to read number");

//     Alice = Alice_num
//         .trim()
//         .split_whitespace()
//         .filter_map(|x| x.parse::<i32>().ok())
//         .collect();

//     println!("Bob - enter num:");
//     let mut Bob_num = String::new();
//     io::stdin()
//         .read_line(&mut Bob_num)
//         .expect("incorrect number");
//     Bob = Bob_num
//         .trim() // remove space or white marks
//         .split_whitespace()
//         .filter_map(|x| x.parse::<i32>().ok())
//         .collect();
    
//     if Alice.iter().any(|&x|x < 1 || x > 100) || Bob.iter().any(|&x| x < 1 || x > 100) {
//         println!("incorrect number");
//         return;
//     }
//     let mut counter_Alice = 0;
//     let mut counter_Bob = 0;
    
//     for i in 0..Alice.len() {
//         for j in 0..Bob.len() {
//             if (i == j) {
//                 if Alice[i] > Bob[j] {
//                     counter_Alice += 1;
                   
//                 } else if Alice[i] == Bob[j] {
                   
//                     // No change to the counters when equal
//                 } else {
//                     counter_Bob += 1;
                    
//                 }
//             }
//         }
//     }

//         println!("Marks: Alice = {},  Bob = {}", counter_Alice, counter_Bob);
// }


//#3
// use std::io;

// fn main() {
//     let mut N = 0;
//     println!("Type the number of arrays: ");
    
//     let mut input_number = String::new();
//     io::stdin().read_line(&mut input_number).unwrap();
    
//     N = input_number.trim().parse().unwrap();
    
//     let mut arrays: Vec<Vec<i64>> = Vec::new();
    
//     for i in 0..N {
//         let mut array: Vec<i64> = Vec::new();
        
//         println!("print a number{}: ", i + 1);
        
//         loop {
//             let mut line = String::new();
//             io::stdin().read_line(&mut line).unwrap();
            
//             let line = line.trim();
            
//             if line.len() == 10 {
//                 let numbers: Vec<i64> = line.chars()
//                     .map(|c| c.to_digit(10).unwrap() as i64)
//                     .collect();
                
//                 array.extend(numbers);  
//                 break;  
//             } else {
//                 println!("Error");
//             }
//         }
        
//         arrays.push(array);
//     }

//     let result = add_arrays_at_same_index(&arrays);

//     println!("Result of adding corresponding elements as a single number: {}", result);
// }

// fn add_arrays_at_same_index(arrays: &[Vec<i64>]) -> i64 {
//     if arrays.is_empty() || arrays[0].len() != 10 {
//         panic!("Arrays must be of length 10.");
//     }

//     let mut result = vec![0; 10];  

//     for i in 0..10 {
//         for array in arrays {
//             result[i] += array[i]; 
//         }
//     }

//     let mut result_number = 0;
//     for (i, &digit) in result.iter().enumerate() {
//         result_number = result_number * 10 + digit;
//     }

//     result_number
// }


//#4
// use std::io;
// fn main() {
//     let mut size = String::new();
//     println!("Give the size of the matrix:");
//     io::stdin().read_line(&mut size).expect("Cannot read line");
//     let size: usize = size.trim().parse().expect("print correct num");

//     let mut matrix: Vec<Vec<i64>> = Vec::new();

//     println!("Type matrix elements:");
//     for i in 0..size {
//         let mut row = String::new();
//         io::stdin().read_line(&mut row).expect("Cannot read line");

//         let row: Vec<i64> = row
//             .trim()
//             .split_whitespace()
//             .map(|num| num.parse().expect("print corrcet num"))
//             .collect();

//         if row.len() != size {
//             panic!("Each row must have exactly {} elements.", size);
//         }

//         matrix.push(row);
//     }

//     let mut r_diagonal = 0;
//     let mut l_diagonal = 0;

//     for i in 0..size {
//         r_diagonal += matrix[i][i]; 
//         l_diagonal += matrix[i][size - i - 1]; 
//     }

//     let diff = (r_diagonal - l_diagonal).abs();

//     println!("Right diagonal sum: {}", r_diagonal);
//     println!("Left diagonal sum: {}", l_diagonal);
//     println!("Absolute difference: {}", diff);
// }


// use std::io;


// fn main() {
//     let mut n = String::new();
//     println!("enter the number n: ");
//     io::stdin().read_line(&mut n).expect("error with reading line");
//     let n: usize = n.trim().parse().expect("print corrcet number");

//     let mut array = Vec::new();
//     println!("give elementss for array:");

//     for _ in 0..n {
//         let mut element = String::new();
//         io::stdin().read_line(&mut element).expect("error with read input line");
//         let element: i32 = element.trim().parse().expect("print correct element");
//         array.push(element);
//     }
//     println!("Array: {:?}", array);

//     let mut positive = 0;
//     let mut negative = 0;
//     let mut zero  = 0;

//     for &i in &array {
//         if i < 0 {
//             negative +=1;
//         }
//         else if i == 0 {
//             zero += 1;
//         }
//         else {
//             positive +=1;
//         }   
//     }

//     let result_positive = (positive as f64) / (n as f64);
//     let result_negative = (negative as f64) / (n as f64);
//     let result_zero = (zero as f64) / (n as f64);

//     println!("result: {:.6}, {:.6}, {:.6}", result_positive, result_negative, result_zero);
  
// }


// use std::io;

// fn main() {
//     println!("give size of staircase: ");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).expect("error with reading input size");
//     let size: usize = size.trim().parse().expect("print correct number");
//     println!("our staircase:");

//     for i in 1..=size {
//         let mut line = String::new();
//         for _ in 0..(size-i) {
//             line.push(' ');
//         }
//         for _ in 0..i {
//             line.push('#');
//         }

//         println!("{}", line);
//     }
// }



//7
// use std::io;

// fn main() {
//     println!("give size of array:");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).expect("error with reading input data");
//     let size: usize = size.trim().parse().expect("print correct size");


//     let mut array= Vec::new();
//     println!("cin array elements:");

//     for _ in 0..size {
//         let mut elements = String::new();
//         io::stdin().read_line(&mut elements).expect("cin corrcet elements in array");
//         let elements: i32 = elements.trim().parse().expect("error, give correct elements in array");
//         array.push(elements);
//     }
//     println!("Array: {:?}", array);

//     if array.len() < 2 {
//         println!("give more elements");
//     }

//     let mut minvalue = array[0];
//     let mut maxvalue = array[0];

//     for &i in &array {
//         if i < minvalue {
//             minvalue = i;
//         }
//         if i > maxvalue {
//             maxvalue = i;
//         }
//     }

//     let  sumtotal: i32 = array.iter().sum();
//     let summax = sumtotal - minvalue;
//     let summin = sumtotal - maxvalue;

//     println!("Sum without minelement: {}", summin);
//     println!("Sum without maxelement: {}", summax);
// }


//8
// use std::io;

// fn main() {
//     println!("cin size:");
//     let mut n_candles = String::new();
//     io::stdin().read_line(&mut n_candles).expect("error with reading input data");
//     let n_candles: usize = n_candles.trim().parse().expect("print correct number input");

//     let mut array_candles = Vec::new();
//     println!("give quantity of candles");
//     for _ in 0..n_candles {
//         let mut candles = String::new();
//         io::stdin().read_line(&mut candles).expect("problem with reading input");
//         let candles: i32 = candles.trim().parse().expect("give correct count candles");
//         array_candles.push(candles);
//     }
//     println!("Array_candles: {:?}", array_candles);

//     let mut max_height_candles = array_candles[n_candles-1];
//     let mut max_count = 0;
//     for &i in &array_candles {
//         if max_height_candles > i {
//             max_count += 1;
//         }
//         if max_height_candles == i {
//             max_count +=1;
//         }
//     }

//     println!("The quantity maxmimum height of candles is: {}", max_count);

// }

//9

// use std::io;

// fn main() {
//     println!("what is your time:");
//     let mut time_input = String::new();
//     io::stdin().read_line(&mut time_input).expect("error read input time!");

//     let time_input = time_input.trim();
    
//     if time_input.len() == 10 && (time_input[8..].to_uppercase() == "AM" || time_input[8..].to_uppercase() == "PM") {
//         println!("Time:  {}",time_input);
//     } else {
//         println!("wrong form of time");
//     }

//     let part_time = &time_input[..8];
//     let type_of_meredian = &time_input[8..];

//     let parts: Vec<&str> = part_time.split(':').collect();
//     if parts.len() != 3 {
//         println!("incorrect form time");
//         return;
//     }

//     let mut hours: u32 = match parts[0].parse() {
//         Ok(h) => h,
//         Err(_) => {
//             println!("incorrect value hours");
//             return;
//         }
//     };

//     let mut minutes: u32 = match parts[0].parse() {
//         Ok(m) => m,
//         Err(_) => {
//             println!("incorrect value minutes");
//             return;
//         }
//     };

//     let mut seconds: u32 = match parts[0].parse() {
//         Ok(m) => m,
//         Err(_) => {
//             println!("incorrect value seconds");
//             return;
//         }
//     };

//     if (hours > 12 || minutes > 59 || seconds > 59) {
//         println!("incorrect hours or minutes or seconds");
//         return;
//     }

//     if type_of_meredian.to_uppercase() == "AM" {
//         if hours == 12 {
//             hours = 0;
//         }
//     } 
//     else if type_of_meredian.to_uppercase() == "PM" {
//         if hours != 12 {
//             hours +=12;
//         }
//     }
//     println!("Time in format 24 hours {:02}:{:02}:{:02}", hours, minutes, seconds);
// }


//#10

// use std::io;

// fn main() {
//     println!("print count of grade for students");
//     let mut grade_count = String::new();
//     io::stdin().read_line(&mut grade_count).expect("error read given grade count");
//     let grade_count: usize = grade_count.trim().parse().expect("print correct number input");

//     println!("give grades:  ");
//     let mut grades = Vec::new();
//     for _ in 0..grade_count {
//         let mut grade = String::new();
//         io::stdin().read_line(&mut grade).expect("error with read grades");
//         let grade: i32 = grade.trim().parse().expect("give correct grades");
//         grades.push(grade);
//     }
//     println!("Array_candles: {:?}", grades);

//     let limit = 40;
//     let mut new_grade= Vec::new();
//     let mut new_grades_values = 0;
//     let array_round: [i32;2] = [1,2];
//     for &i in &grades {
//         if i < limit {
//             println!("{} is a failing grade", i);
//         }
//         else {
//             for j in array_round {
//                 if (i + j) % 5 == 0 {
//                     new_grades_values = i +j; 
//                     new_grade.push(new_grades_values);
//                 } else if (i - j) % 5 == 0 {
//                     new_grades_values = i - j; 
//                     new_grade.push(new_grades_values);
//                 } 
//             }
//         } 
//     }

//     println!("New Grades: {:?}", new_grade);
// }


// #11

// use std::io;

// fn count_fruits(s: i32, t: i32, a: i32, b:i32, m: usize, n: usize, apples: Vec<i32>, oranges: Vec<i32>) {
//     let mut apples_count_in_house: i32 = 0;
//     let mut oranges_count_in_house: i32 = 0;

//     for i in 0..apples.len() {
//         let apple_position = a + apples[i];
//         if apple_position >= s && apple_position <= t {
//             apples_count_in_house += 1;
//         }
//     }

//     for i in 0..oranges.len() {
//         let orange_position = b + oranges[i];
//         if orange_position >= s && orange_position <= t {
//             oranges_count_in_house += 1;
//         }
//     }

//     println!("apples_count_in_house: {}", apples_count_in_house);
//     println!("apples_count_in_house: {}", apples_count_in_house);
// }
// fn main() {

//     let mut input_data = String::new();

//     io::stdin().read_line(&mut input_data).unwrap();
//     let s_t: Vec<i32> = input_data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//     let s = s_t[0];
//     let t = s_t[1];
//     input_data.clear();  

//     io::stdin().read_line(&mut input_data).unwrap();
//     let a_b: Vec<i32> = input_data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//     let a = a_b[0];
//     let b = a_b[1];
//     input_data.clear(); 

//     io::stdin().read_line(&mut input_data).unwrap();
//     let m_n: Vec<usize> = input_data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//     let m = m_n[0];
//     let n = m_n[1];
//     input_data.clear(); 

//     io::stdin().read_line(&mut input_data).unwrap();
//     let apples: Vec<i32> = input_data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//     input_data.clear(); 

//     io::stdin().read_line(&mut input_data).unwrap();
//     let oranges: Vec<i32> = input_data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//     input_data.clear(); 


//     count_fruits(s,t,a,b,m,n,apples,oranges);
// }

// #12

// use std::io;

// fn distance_first_kangaroo(x1: i32, v1: i32, step: i32) -> i32 {
//      x1 + v1 * step
// }

// fn distance_second_kangaroo(x2: i32, v2: i32, step: i32) -> i32 {
//     x2 + v2 * step
// }

// fn verify(distance_second_kangaroo: i32, distance_first_kangaroo: i32) -> bool {
//    distance_first_kangaroo == distance_second_kangaroo
// }


// fn main() {
//     println!("print points for two kangaroos: x1 v1  x2 v2  ");
//     let mut points = String::new();
//     io::stdin().read_line(&mut points).unwrap();
//     let point: Vec<i32> = points.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
//     let x1 = point[0];
//     let v1 = point[1];
//     let x2 = point[2];
//     let v2 = point[3];

//     let mut step = 0;
//     let max_step = 100;

//     while step <= max_step {
//         let distance_first = distance_first_kangaroo(x1, v1, step);
//         let distance_second = distance_second_kangaroo(x2, v2, step);

//         if verify(distance_second, distance_first) {
//             println!("YES");
//             return;
//         } 
//         step += 1;
//     }
//     println!("NO");

// }

// # 13

// use std::io;


// fn main() {
//     println!("print n-number for work in two arrays: ");
//     let mut n = String::new();
//     io::stdin().read_line(&mut n).unwrap();
//     let n: i32 = n.trim().parse().unwrap();

//     let first_array = vec![2,4];
//     let second_array = vec![16,32,96];

//     let mut count = 0;
//     for i in 1..n {
//         let mut first_condition = true;
//         for &j in &first_array {
//             if i % j != 0 {
//                 first_condition = false;
//                 break;
//             }
//         }

//         let mut second_condition = true;
//         for &j in &second_array {
//             if i & j != 0 {
//                 second_condition = false;
//                 break;
//             }
//         }

//         if first_condition && second_condition {
//             count += 1;
//         }
//     }

//     println!("count for both conditions is: {}", count);
// }


// #14

// use std::io;

// fn main() {

//     println!("Enter the number of scores:");
//     let mut n = String::new();
//     io::stdin().read_line(&mut n).unwrap();
//     let n: usize = n.trim().parse().expect("Please enter a valid number");

//     println!("Enter the scores:");
//     let mut scores = Vec::new();
//     for _ in 0..n {
//         let mut score = String::new();
//         io::stdin().read_line(&mut score).unwrap();
//         let score: i32 = score.trim().parse().expect("Please enter a valid score");
//         scores.push(score);
//     }

//     println!(
//         "{:<10} {:<10} {:<10} {:<10} {:<10} {:<5} {:<5}",
//         "Game", "Score", "Minimum", "Maximum", "Count", "Min", "Max"
//     );

//     let mut max_score = scores[0];
//     let mut min_score = scores[0];
//     let mut max_breaks = 0;
//     let mut min_breaks = 0;

//     for idx in 0..scores.len() {
//         let score = scores[idx];

//         if score > max_score {
//             max_score = score;
//             max_breaks += 1;
//         }

//         if score < min_score {
//             min_score = score;
//             min_breaks += 1;
//         }
    
//         println!(
//             "{:<10} {:<10} {:<10} {:<10} {:<10} {:<5} {:<5}",
//             idx + 1,
//             score,
//             min_score,
//             max_score,
//             idx + 1,
//             min_breaks,
//             max_breaks
//         );
//     }
// }


// #15

// use std::io;

// fn main() {
//     println!("Print len of chocolate bar:");
//     let mut n = String::new();
//     io::stdin().read_line(&mut n).unwrap();
//     let n: usize = n.trim().parse().expect("Print valid len");

//     println!("Print count d-day:");
//     let mut d = String::new();
//     io::stdin().read_line(&mut d).unwrap();
//     let d: usize = d.trim().parse().expect("Print valid len");

//     println!("Print count m-month:");
//     let mut m = String::new();
//     io::stdin().read_line(&mut m).unwrap();
//     let m: usize = m.trim().parse().expect("Print valid len");

//     println!("Fill your chocolate bar with digitals:");
//     let mut chocolate_bar = Vec::new();
//     for _ in 0..n {
//         let mut count_digitals = String::new();
//         io::stdin().read_line(&mut count_digitals).unwrap();
//         let count_digitals: usize = count_digitals.trim().parse().expect("Enter correct digits for bar");
//         chocolate_bar.push(count_digitals);
//     }

//     let mut count = 0;

//     for i in 0..=n - m {
//         let sum: usize = chocolate_bar[i..i + m].iter().sum();
//         if sum == d {
//             count += 1;
//         }
//     }

//     println!("Count: {}", count);
// }


// #16

// use std::io;


// fn find_pair(array: &Vec<i32>, k: i32) -> Vec<(i32, i32)> {
//     let mut pairs = Vec::new();
//     for i in 0..array.len() {
//         for j in i+1..array.len() {
//             if (array[i] + array[j]) % k == 0 {
//                 pairs.push((array[i], array[j]));
//             } 
//         }
//     }
//     pairs
// } 

// fn main() {
//     println!("size of array: ");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).unwrap();
//     let size: usize = size.trim().parse().expect("enter valid data in int type");

//     println!("print divisible num k: ");
//     let mut k = String::new();
//     io::stdin().read_line(&mut k).unwrap();
//     let k: i32 = k.trim().parse().expect("enter valid data in int type");

//     println!("filling array: ");
//     let mut array = Vec::new();
//     for _ in 0..size {
//         let mut element = String::new();
//         io::stdin().read_line(&mut element).unwrap();
//         let element: i32 = element.trim().parse().expect("error data");
//         array.push(element);
//     }

//     let pairs = find_pair(&array, k);
//     if pairs.is_empty() {
//         println!("no pairs");
//     }
//     else {
//         for (i,j) in pairs {
//             println!("pairs: {}, {}", i, j);
//         }
//     }
// }

// #17

// use std::io;

// fn main() {
//     println!("size of array: ");
//      let mut size = String::new();
//      io::stdin().read_line(&mut size).unwrap();
//      let size: usize = size.trim().parse().expect("enter valid data in int type");

//      println!("filling array: ");
//     let mut array = Vec::new();
//     for _ in 0..size {
//         let mut element = String::new();
//         io::stdin().read_line(&mut element).unwrap();
//         let element: i32 = element.trim().parse().expect("error data");
//         array.push(element);
//     }

//     let mut max = 0;
//     let mut frequent = -2;
//     for i in 0..array.len() {
//         let bird = array[i];
//         let mut count = 0;
//         for &j in &array {
//             if j == bird {
//                 count +=1;
//             }
//         }
//         if count > max || (count == max && bird < frequent) {
//             max = count;
//             frequent = bird;
//         }
//     }

//     println!("most frequent bird: {}", frequent);
// }

// #18

// use std::io;

// fn main() {
//     println!("size of array: ");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).unwrap();
//     let size: usize = size.trim().parse().expect("enter valid data in int type");

//     println!("decline dish number: ");
//     let mut decline = String::new();
//     io::stdin().read_line(&mut decline).unwrap();
//     let decline: usize = decline.trim().parse().expect("enter valid data in int type");

//     println!("filling array: ");
//     let mut array = Vec::new();
//     for _ in 0..size {
//         let mut cost_element = String::new();
//         io::stdin().read_line(&mut cost_element).unwrap();
//         let cost_element: i32 = cost_element.trim().parse().expect("error data");
//         array.push(cost_element);
//     }

//     if decline < array.len() {
//         array.remove(decline);
//     } 
//     else {
//         println!("inccorect index");
//     }
//     let mut price = 0;
//     for &i in &array {
//         price += i;
//     }

//     println!("Price: {}", price/2);

// }

// # 19

// use std::io;

// fn main() {
//     println!("size of array: ");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).unwrap();
//     let size: usize = size.trim().parse().expect("enter valid data in int type");

//     println!("filling array: ");
//     let mut array = Vec::new();
//     for _ in 0..size {
//         let mut element = String::new();
//         io::stdin().read_line(&mut element).unwrap();
//         let element: i32 = element.trim().parse().expect("error data");
//         array.push(element);
//     }

//     let mut pairs = 0;
//     while !array.is_empty() {
//         let sock = array[0];
//         let mut count = 0;
//         let mut i = 0;
//         while i < array.len() {
//             if array[i] == sock {
//                 count += 1;
//                 array.remove(i);
//             }
//             else {
//                 i += 1;
//             }
//         }
//         pairs += count / 2;
//     }

//     println!("All pairs socks: {}", pairs);

// }


// use std::cmp::min;

// fn minimum_turns(n: usize, p: usize) -> usize {
//     let from_front = p / 2;
//     let from_back = (n / 2) - (p / 2);
    
//     min(from_front, from_back) 
// }

// fn main() {
//     println!("enter number of pages:");
//     let mut n = String::new();
//     std::io::stdin().read_line(&mut n).unwrap();
//     let n: usize = n.trim().parse().expect("Invalid input!");

//     println!("enter page to turn:");
//     let mut p = String::new();
//     std::io::stdin().read_line(&mut p).unwrap();
//     let p: usize = p.trim().parse().expect("Invalid input!");

//     let result = minimum_turns(n, p);

//     println!("number of pages to turn: {}", result);

// }
