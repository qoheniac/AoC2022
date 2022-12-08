use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let forrest: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let row_number = forrest.len();
    let column_number = forrest[0].len();

    let mut max_score = 0;
    for i in 0..row_number {
        for j in 0..column_number {
            let height = forrest[i][j];
            
            // look up
            let mut up_score = 0;
            for k in (0..i).rev() {
                up_score += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            // look left
            let mut left_score = 0;
            for k in (0..j).rev() {
                left_score += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }

            // look right
            let mut right_score = 0;
            for k in (j+1)..column_number {
                right_score += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }

            // look down
            let mut down_score = 0;
            for k in (i+1)..row_number {
                down_score += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            let score = up_score * left_score * right_score * down_score;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{}", max_score)
}
