mod tsp;
use tsp::{Matrix, print_matrix, tsp};

fn main() {
    println!("Masukkan jumlah kota: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Gagal membaca input.");
    let n: usize = input.trim().parse().expect("Input harus berupa angka.");

    println!("Apakah graf berarah? (y/n): ");
    let mut is_directed = String::new();
    std::io::stdin().read_line(&mut is_directed).expect("Gagal membaca input.");
    let is_directed = is_directed.trim().to_lowercase() == "y";

    // Nilai yang sangat besar untuk merepresentasikan tidak ada jalan
    let no_path_value = u32::MAX / 2; 
    
    let mut dist = vec![vec![0; n]; n];

    println!("Untuk menandai tidak ada jalur antara dua kota, masukkan -1");

    if is_directed {
        // Graf berarah
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    println!("Masukkan jarak dari kota {} ke kota {}: ", i+1, j+1);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Gagal membaca input.");
                    let distance: i32 = input.trim().parse().expect("Input harus berupa angka.");
                    if distance == -1 {
                        dist[i][j] = no_path_value;
                    } else {
                        dist[i][j] = distance as u32;
                    }
                }
            }
        }
    } else {
        // Graf tidak berarah
        for i in 0..n {
            for j in (i+1)..n {
                println!("Masukkan jarak antara kota {} dan kota {}: ", i+1, j+1);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Gagal membaca input.");
                let distance: i32 = input.trim().parse().expect("Input harus berupa angka.");
                if distance == -1 {
                    dist[i][j] = no_path_value;
                    dist[j][i] = no_path_value;
                } else {
                    dist[i][j] = distance as u32;
                    dist[j][i] = distance as u32;
                }
            }
        }
    }

    let matrix = Matrix { n, dist };
    
    // Tampilkan matriks jarak
    print_matrix(&matrix);
    
    let (min_cost, path) = tsp(&matrix);
    
    if min_cost >= (no_path_value as i32) {
        println!("Tidak ada jalur yang valid untuk mengunjungi semua kota dan kembali.");
    } else {
        println!("Biaya perjalanan minimum: {}", min_cost);
        println!("Jalur terbaik: {}", path.iter()
            .map(|&city| city.to_string())
            .collect::<Vec<String>>()
            .join(" → "));
        println!("Jumlah edge: {}", path.len() - 1);
    }
}