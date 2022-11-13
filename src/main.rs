fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 loop {
  println!("\r\n\r\nvector:");

  let mut input: String = request();

  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   let mut closest: i32      = 0                                                      ;
   let mut equal  : bool     = false                                                  ;
   let mut integer: Vec<i32> = serde_json::from_str(&input[..]).expect("Wrong format");
   let mut minimum: i32      = 0                                                      ;
   let     size   : usize    = integer.len()                                          ;

   println!("\r\ntarget:");

   input = request();

   if &input[..] == "exit" {
    break;   

   } else {//if &input[..] == "exit" {
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
  }//} else {//if &input[..] == "exit" {
 }//loop {
}//fn main() {
