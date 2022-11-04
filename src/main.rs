fn main() {
 let     exists: bool = std::path::Path::new(".\\vector.txt").exists();
 let mut input : String = String::new()                               ;

 if exists {
  input = std::fs::read_to_string(".\\vector.txt").expect("Failed file reading!");

 }//if exists {

 loop {
  if exists {
   if input.trim().is_empty() {
    println!("\r\n\r\nvector:");

    std::io::stdin().read_line(&mut input).expect("Input failed");
   }//if input.trim().is_empty() {

  } else {//if exists {
   if !input.trim().is_empty() {
    input = String::new();

   }//if !input.trim().is_empty() {

   println!("\r\n\r\nvector:");

   std::io::stdin().read_line(&mut input).expect("Input failed");
  }//} else {//if exists {

  input = input.replace("\n", "");
  input = input.replace("\r", "");

  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   let mut closest: i32      = 0                                                      ;
   let mut equal  : bool     = false                                                  ;
   let mut integer: Vec<i32> = serde_json::from_str(&input[..]).expect("Wrong format");
   let mut minimum: i32      = 0                                                      ;
   let     size   : usize    = integer.len()                                          ;

   println!("\r\ntarget:");

   input = String::new();

   std::io::stdin().read_line(&mut input).expect("Input failed");

   input = input.replace("\n", "");
   input = input.replace("\r", "");

   let target: i32 = (&input[..]).trim().parse().expect("Wrong format");

   integer.sort();

   if size > 2usize {
    let n: usize = size - 2usize;

    for i in 0..n {
     let mut j: usize = i + 1usize;
     let mut k: usize = n + 1usize;

     while j < k {
      let sum: i32 = integer[i] + integer[j] + integer[k];

      if sum == target {
       equal = true;
 
       break;

      } else {//if sum == target {
       let difference: i32;

       if sum < target {
        difference = target - sum;

       } else {//if sum < target {
        difference = sum - target;

       }//} else {//if sum < target {

       if i == 0 && j == 1 && k == n + 1usize {
        closest = sum       ;
        minimum = difference;

       } else {//if i == 0 && j == 1 && k == n + 1usize {
        if difference < minimum {
         closest = sum       ;
         minimum = difference;
        }//if difference < minimum {
       }//} else {//if i == 0 && j == 1 && k == n + 1usize {

       if sum < target {
        j += 1;

       } else {//if sum < target {
        k -= 1;

       }//} else {//if sum < target {
      }//} else {//if sum == target {
     }//while j < k {

     if equal {
      break;

     }//if equal {
    }//for i in 0..n {
   }//if size > 2usize {

   if equal {
    println!("\r\nclosest:\r\n{:?}", target);

   } else {//if equal {
    println!("\r\nclosest:\r\n{:?}", closest);

   }//if equal {//} else {//if equal {
  }//} else {//if &input[..] == "exit" {

  if exists {
   break;

  }//if exists {
 }//loop {
}//fn main() {
