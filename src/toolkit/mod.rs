pub fn test<T>(ans: T, expect: T)
where
    T: std::cmp::PartialEq + std::fmt::Display
{
    let result;
    if ans == expect {
        result = "passed";
    } else { 
        result = "failed";
    }

    println!("Result of Test is: {ans}");
    println!("Expected: {expect}");
    
    println!("Test: {result}\n");
}

pub fn solution<T>(ans: T)
where
    T: std::fmt::Display
{
    println!("The Solution is: {ans}")
}