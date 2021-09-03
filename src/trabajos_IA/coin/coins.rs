
fn resolve_coins(coins: &Vec<i64>, mut obj: i64) -> Vec<i64> {
    let mut resolved_coins = Vec::new();
    for coin in coins {
        while obj >= *coin {
            obj -= *coin;
            resolved_coins.push(*coin);
        }
    }
    resolved_coins
}

fn main() {
    let coins = vec![25, 20, 10, 5];
    let obj = 30;

    let res = resolve_coins(&coins, obj);
    println!("Vector de monedas: {:?}\nObj: {}\nRespuesta: {:?}", coins, obj, res);
}
