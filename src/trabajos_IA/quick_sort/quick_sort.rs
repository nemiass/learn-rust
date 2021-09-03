
fn part(vector: &mut Vec<i64>, low: usize, hight: usize) -> usize {
    let pivot = vector[0];
    let mut p_left = low + 1;
    let mut p_right = hight;
    loop {
        while p_left <= p_right && vector[p_left] <= pivot {
            p_left += 1;
        };
        while p_left <= p_right && vector[p_right] >= pivot {
            p_right -= 1;
        };
        if p_right < p_left {
            break;
        } else {
            let tmp = vector[p_left];
            vector[p_left] = vector[p_right];
            vector[p_right] = tmp;
        }
    }
    let tmp = vector[low];
    vector[low] = vector[p_right];
    vector[p_right] = tmp;
    p_right
}

fn qs(vector: &mut Vec<i64>, low: usize, hight: usize) {
    if low < hight {
        let pivot = part(vector, low, hight);
        qs(vector, low, pivot - 1);
        qs(vector, pivot + 1, hight);
    } 
}

fn quick_sort(vector: &Vec<i64>) -> Vec<i64> {
    let mut vector = vector.to_vec();
    let hight = vector.len() - 1;
    qs(&mut vector, 0, hight);
    vector.to_vec()
}

fn main() {
    let ejemplo: Vec<i64> = vec![9, 1, 3, 10, 2, 500];
    let res = quick_sort(&ejemplo);
    println!("\nVector original: {:?},\nVector QuickSort: {:?}\n", ejemplo, res);
}
