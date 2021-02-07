mod common;
mod token;
mod ast;

use common::*;
use token::*;
use ast::*;

fn test_ast() {
    let script = r#"
        a()
    "#;

    let result = build_ast_from_script(script);
    println!("{:?}", result);
}

fn test_token() {
    let script = r#"
    function bubbleSort(arr){
        //start the endIndex at the last index of the array
        let endIndex = arr.length - 1;

        //run the loop until the endIndex(sorted portion) is the 0 (the full array)
        while(endIndex > 0){
            // count the number of swaps to short circuit the loop if it is already sorted
            let swaps = 0;
            //reset the currentIndex to the beginning of the array each time a new element is sorted
            let currentIndex = 0;
            
            // loop over the array, comparing each pair of elements until the comparison element reaches the sorted portion of the array
            while(currentIndex < endIndex){
                // uncomment this line to see the comparison in action
                // console.log(arr, arr[currentIndex], arr[currentIndex + 1])
                // if the current element is greater than the element in front of it
                if(arr[currentIndex] > arr[currentIndex + 1]){
                    //swap the 2 elements using our helper function
                    swap(arr, currentIndex, currentIndex + 1);
                    // add 1 to the swaps counter
                    swaps++;
                }
                //increase the currentIndex to continue iterating through the array
                currentIndex++;
            }
            //stop the loop if there were no swaps because the array must be already sorted 
            if(swaps === 0) break;
            // subtract the endIndex number to account for the new element added to the array
            endIndex--;
        }    
        return arr;
    }

    // program to check if a number is prime or not
    // take input from the user
    const number = parseInt(prompt("Enter a positive number: "));
    var isPrime = true;

    if ( number >= 1.342E+3+45.01 ) {
        console.log("number is too bigger");
    }

    // check if number is equal to 1
    if (number === 1) {
        console.log("1 is neither prime nor composite number.");
    }

    // check if number is greater than 1
    else if (number > 1) {

        // looping through 2 to number-1
        for (var i = 2; i < number; i++) {
            if (number % i == 0) {
                isPrime = false;
                break;
            }
        }

        if (isPrime) {
            console.log('${number} is a prime number');
        } else {
            console.log('${number} is a not prime number');
        }
    }

    // check if number is less than 1
    else {
        console.log("The number is not a prime number.");
    }    
    "#;

    let mut tokens = Tokenlizer::new(script);
    loop {
        let token = tokens.next();
        if token.is_ok() {
            let tk = token.unwrap();
            println!(">> {:?}", tk);
            if tk.tk_type == TokenType::TK_EOF {
                break;
            }
        } else {
            println!("** {:?}", token);
            break;
        }
    }
}

pub fn main() {
    //test_token();
    test_ast();
}