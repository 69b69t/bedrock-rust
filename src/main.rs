use bedrockgen::BedrockGenerator;
use bedrockgen::overworld::OverworldBedrock;
//use bedrockgen::nether::floor::NetherBedrock;
use bedrockgen::nether::roof::NetherBedrock;

fn similarity_check(first: &[bool], second: &[bool]) -> f64 {
    let length = first.len();
    assert_eq!(length, second.len());

    let mut counter = 0;
    for i in 0..length {
        if first[i] == second[i] {
            counter += 1;
        }
    }

    (counter as f64) / (length as f64)
}

fn main() {
    let test = OverworldBedrock::new(0);
    println!("{}", test.xr.high);
}

/*
pairs from andrew
3387475 11495857
14391324 19455217
15330447 23598152
12707052 24163979
2210519 26160814
10605566 27291435
3351313 28018135
22930187 29437475
21888275 30384970
28178283 31347541
6245239 31735938
24064455 31930598
2630413 32604855
20122818 34220079
1548823 37085592
21814723 37097492
23641604 37103284
18770175 38280548
18277579 38346655
31288489 39126476
*/
