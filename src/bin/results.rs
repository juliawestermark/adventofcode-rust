use aoc2022::{
    ANSI_BOLD,
    ANSI_RESET,
    COLOR_GOLD,
    COLOR_LIGHT_GRAY,
    print_ending,
    lines_from_file,
    get_days_in_december,
};

fn color_star(done: bool) -> String {
    let color = if done {COLOR_GOLD} else {COLOR_LIGHT_GRAY};
    let star = format!("{}*{}", color, ANSI_RESET);
    star
}

fn print_stars(day: i32, stars: i32) {
    let star1 = color_star(stars > 0);
    let star2 = color_star(stars > 1);
    let message = format!("     Day {:02}) {}{}", day, star1, star2);
    println!("{}", message );
}


fn main() {
    println!();
    println!("{}--- 🎄 RESULT 🎄 ---{}", ANSI_BOLD, ANSI_RESET);
    println!();
    
    let filename = "results.txt";
    let input = lines_from_file(filename);
    let mut nbr_of_stars = 0;
    let days_in_december = get_days_in_december();
    
    if days_in_december > 0 {
        for line in input {
            let vec: Vec<i32> =
                line.split(" ").map(|s| s.parse().unwrap()).collect();
            let day = vec[0];
            if day > days_in_december {
                break
            }
            nbr_of_stars += vec[1];
            print_stars(day, vec[1]);
            println!();
        }
        
        println!();
        if nbr_of_stars == 0 {
            println!("🎄 You can do it! 🎄");
        }
        else if nbr_of_stars == 1 {
            println!("{}       {} star {}", COLOR_GOLD, nbr_of_stars, ANSI_RESET);
        }
        else {
            println!("{}      {}  stars {}", COLOR_GOLD, nbr_of_stars, ANSI_RESET);
        }
        println!();
    }  
        
        print_ending();
    }
