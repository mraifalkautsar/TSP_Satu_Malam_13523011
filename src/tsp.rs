pub struct Matrix {
    pub n: usize, 
    pub dist: Vec<Vec<u32>>, 
}

pub fn print_matrix(matrix: &Matrix) {
    println!("\nMatriks Jarak:");
    print!("    ");
    for i in 0..matrix.n {
        print!("{:4}", i+1);
    }
    println!();

    for i in 0..matrix.n {
        print!("{:2}  ", i+1);
        for j in 0..matrix.n {
            if matrix.dist[i][j] == u32::MAX / 2 {
                print!("  - "); // Tidak ada jalur
            } else {
                print!("{:4}", matrix.dist[i][j]);
            }
        }
        println!();
    }
    println!();
}

pub fn total_cost(mask: usize, curr: usize, n: usize, dist: &Vec<Vec<u32>>, 
             memo: &mut Vec<Vec<i32>>, next: &mut Vec<Vec<usize>>) -> i32 {
    // Base case: semua kota telah dikunjungi
    if mask == (1 << n) - 1 {
        next[curr][mask] = 0; // Kembali ke kota awal
        return dist[curr][0] as i32;
    }

    // Kondisi ada pada memo
    if memo[curr][mask] != -1 {
        return memo[curr][mask];
    }

    let mut ans = i32::MAX;
    let mut best_next = 0;

    // Coba kunjungi semua kota
    for i in 0..n {
        if (mask & (1 << i)) == 0 {
            let new_cost = dist[curr][i] as i32 + total_cost(mask | (1 << i), i, n, dist, memo, next);
            if new_cost < ans {
                ans = new_cost;
                best_next = i;
            }
        }
    }

    // Setor hasil pada memo dan kembali
    memo[curr][mask] = ans;
    next[curr][mask] = best_next; // Simpan kota berikutnya untuk jalur optimal
    ans
}

pub fn construct_path(n: usize, next: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut path = Vec::with_capacity(n + 1);
    let mut curr = 0;
    let mut mask = 1; 
    
    path.push(curr + 1);
    
    // Bangun jalur dengan mengikuti pointer 'next'
    for _ in 0..n-1 {
        let next_city = next[curr][mask];
        curr = next_city;
        mask |= 1 << curr;
        path.push(curr + 1); 
    }
    
    path.push(1); 
    path
}

pub fn tsp(matrix: &Matrix) -> (i32, Vec<usize>) {
    let n = matrix.n;
    let mut memo = vec![vec![-1; 1 << n]; n];
    let mut next = vec![vec![0; 1 << n]; n]; // Untuk menyimpan kota berikutnya
    
    // Mulai dari kota 0, dengan pada awalnya hanya kota 0 yang dikunjungi
    let min_cost = total_cost(1, 0, n, &matrix.dist, &mut memo, &mut next);
    
    // Bangun jalur dari tabel 'next'
    let path = construct_path(n, &next);
    
    (min_cost, path)
}