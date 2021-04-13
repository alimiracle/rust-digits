
pub fn min_digit(mut n: i32) -> i32 {
// min digit in the number
let mut r = 10;

while n != 0 {
if n%10 < r {
r=n%10;

}

    n /= 10;

}
r
}

pub fn max_digit(mut n: i32) -> i32 {
// max digit in the number
let mut r = 0;

while n != 0 {
if n%10 > r {
r=n%10;

}

    n /= 10;

}
r
}
pub fn replace_digit(mut n: i32, replaced: i32, replace_with: i32) -> i32 {
// replace digit in number
let radix = 10;

let mut reversed = 0;

while n != 0 {
if n%10 != replaced {

    reversed = reversed * radix + n % radix;

} else {
    reversed = reversed * radix + replace_with % radix;
}
    n /= radix;

}
reversed=reverse_n(reversed);
reversed

}
pub fn remove_n(mut n: i32, removed: i32) -> i32 {
//remove digit from number
let radix = 10;

let mut reversed = 0;

while n != 0 {
if n%10 != removed {

    reversed = reversed * radix + n % radix;
}
    n /= radix;

}
reversed=reverse_n(reversed);
reversed

}
pub fn cat_rite(mut n: i32, cat: i32) -> i32 {
// remove digit in rite in the number
let mut count = 0;
let mut nn = 1;
n = reverse_n(n);
while count != cat {
count = count +1;
nn=add_digit(nn, 0);
}
n = n%nn;
n
}
pub fn cat_left(mut n: i32, cat: i32) -> i32 {
// remove digit in left in the number
let mut count = 0;
let mut nn = 1;
while count != cat {
count = count +1;
nn=add_digit(nn, 0);
}
n = n%nn;
n
}

pub fn add_digit(mut n: i32, digit: i32) -> i32 {
// add digit to number

n=n*10;
n=n+digit;
n
}
pub fn count_n(mut n: i32) -> i32 {
// count the digits in the number
let mut count = 0;
if n <0 {
n=n*-1
}
while n>0 {
n=n/10;
count = count +1;
}

count
}
pub fn last_diget(mut n: i32) -> i32 {
// last digit in the number
if n <0 {
n = n*-1
}
n %10
}
pub fn first_diget(num: i32) -> i32 {
// first digit in the number

    if num/10 == 0 {
return num;

}
     first_diget(num/10)
}


pub fn reverse_n(mut n: i32) -> i32 {
// reverse the number
let radix = 10;

let mut reversed = 0;

while n != 0 {
    reversed = reversed * radix + n % radix;
    n /= radix;
}
reversed
}
fn main() {

println!("{}",replace_digit(1234, 2, 1))
}

