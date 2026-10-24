// shree ganeshaya namahha
// This is the first rust programm 
//A simple option based calculator

use std::io;




// declareing all the usefull funcitons 

//add funtion
fn add_num(num_1 :i8,num_2 :i8)-> i8{
    let result:i8;
    result = num_1+num_2;
    return result;
}

//subtract funtion
fn subtract_num(num_1 :i8,num_2 :i8)-> i8{
    let result:i8;
    result = num_1-num_2;
    return result;
}

//multiply funtion
fn mul_num(num_1 :i8,num_2 :i8)-> i8{
    let result:i8;
    result = num_1*num_2;
    return result;
}

//divide funtion
fn div_num(num_1 :i8,num_2 :i8)-> i8{
    let result:i8;
    result = num_1/num_2;
    return result;
}

//remainder funtion
fn remain_num(num_1 :i8,num_2 :i8)-> i8{
    let result:i8;
    result = num_1%num_2;
    return result;
}



fn show_options(choice:i8 ,num_1:i8 , num_2:i8){
    println!("Welcome to the Rusty Calculator");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Remainder");
    println!("Please enter the number of the opration u want to perform (Currently support one opration at a time)");

    if choice==1{
        println!("U have selected add opration");
        println!("addition of {} and {} is {}",num_1,num_2,add_num(num_1,num_2));
    }
    if choice==2{
        println!("U have selected subtract opration");
        println!("subtraction of {} and {} is {}",num_1,num_2,subtract_num(num_1,num_2));
    }
    if choice==3{
        println!("U have selected multiply opration");
        println!("multiplication of {} and {} is {}",num_1,num_2,mul_num(num_1,num_2));
    }
    if choice==4{
        println!("U have selected divide opration");
        println!("division of {} and {} is {}",num_1,num_2,div_num(num_1,num_2));
    }
    if choice==5{
        println!("U have selected remainder opration");
        println!("remainder of {} and {} is {}",num_1,num_2,remain_num(num_1,num_2));
    }
}



// Calling all the functions to oprate on the numbers when needed
fn main(){
    //defining the variables
    let mut input_num1 = String::new();
    let mut input_num2 = String::new();
    let mut input_choice = String::new();

    //takeing the user input for the numbers 
    println!("Enter the first number:");
    io::stdin().read_line(& mut input_num1).expect("Failed to read the input please try again");
    let num_1: i8 = input_num1.trim().parse().expect("Please enter the valid number");

    println!("Enter the second number:");
    io::stdin().read_line(& mut input_num2).expect("Failed to read the input please try again");
    let num_2: i8 = input_num2.trim().parse().expect("Please enter the valid number");


    //takeing the user input for the choise of the 
    println!("Choose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Remainder");
    io::stdin().read_line(& mut input_choice).expect("Failed to read the input please try again");
    let choice: i8 = input_choice.trim().parse().expect("Please enter the valid number");

    show_options(choice, num_1, num_2);
}