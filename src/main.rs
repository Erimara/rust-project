

fn main() {
    let mut choose_fn = String::new();
    println!("Choose a function to run: 1: for calculator. 2: for something else:");
    std::io::stdin().read_line(& mut choose_fn).unwrap();
    let choice:i16 = choose_fn.trim().parse().expect("Don't...");

    if choice == 1 {
        calculator()
     } else if choice == 2 {
         
     }

} 
    
        

fn calculator() {
    println!("You chose the calculator");
    let mut line = String::new();
    let mut calc_a = String::new();
    let mut calc_b = String::new();
    
    println!("1:Addition, 2;Subraktion, 3:Divison, 4:Multiplication");
    std::io::stdin().read_line(&mut line).unwrap();
    let x = line.trim().parse().expect("input not int");
    
    
     if 1==x {
        println!("You chose Addition");
        println!("Enter two numbers: ");
        std::io::stdin().read_line(&mut calc_a).unwrap();
        let a:f32 = calc_a.trim().parse().expect("something went wrong");
    
        std::io::stdin().read_line(&mut calc_b).unwrap();
        let b:f32 = calc_b.trim().parse().expect("something went wrong");
        
        println!("{}",b+a);
    
    } else if 2==x {
        println!("You chose Subtraktion");
        println!("Enter two numbers: ");
    
        std::io::stdin().read_line(&mut calc_a).unwrap();
        let a:i32 = calc_a.trim().parse().expect("something went wrong");
        
    
        std::io::stdin().read_line(&mut calc_b).unwrap();
        let b:i32 = calc_b.trim().parse().expect("something went wrong");
    
        println!("{}", a-b)
        
    } else if 3==x {
        println!("You chose Divison");
        println!("Enter two numbers: ");
        std::io::stdin().read_line(&mut calc_a).unwrap();
        let a:f32 = calc_a.trim().parse().expect("something went wrong");
    
        std::io::stdin().read_line(&mut calc_b).unwrap();
        let b:f32 = calc_b.trim().parse().expect("something went wrong");
    
        println!("{}", a/b);
    
        
    } else if 4==x {
        println!("You chose Multiplication");
        println!("Enter two numbers: ");
        std::io::stdin().read_line(&mut calc_a).unwrap();
        let a:f32 = calc_a.trim().parse().expect("something went wrong");
    
        std::io::stdin().read_line(&mut calc_b).unwrap();
        let b:f32 = calc_b.trim().parse().expect("something went wrong");
    
        println!("{}", a*b);
    } else {
        println!("None eligible option");
    }
    
    }
