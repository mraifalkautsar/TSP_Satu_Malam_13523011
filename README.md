# 13523011_TSP_Satu_Malam

## TSP Satu Malam
Program ini menyelesaikan permasalahan TSP menggunakan pendekatan pemrograman dinamis. Program diimplementasikan dalam bahasa Rust.

## Cara Menjalankan

1. Pastikan Rust sudah terinstal di sistem.
2. Kompilasi dan jalankan program:

   ```sh
   cargo run
   ```
3. Ikuti instruksi di konsol sesuai permintaan input.

## Cara Kerja
**Traveling Salesman Problem (TSP) Rust Implementation**

**Deskripsi Singkat**
Program ini mengimplementasikan algoritma Dinamis dengan Bitmask (DP + bitmask) untuk menyelesaikan masalah Traveling Salesman Problem (TSP). Program akan meminta input jumlah kota, apakah graf bersifat berarah atau tidak, serta jarak antar-kota. Setelah menyusun matriks jarak, program akan menghitung biaya terendah untuk mengunjungi semua kota dan kembali ke kota awal, lalu menampilkan jalur optimal.

---

## Cara Kerja Program

1. **Input Pengguna**

   * Program mulai dengan membaca jumlah kota (`n`).
   * Menanyakan apakah graf berarah (`y`/`n`).
   * Jika graf berarah, pengguna mengisi jarak dari kota `i` ke kota `j` untuk setiap pasangan `(i, j)` (i ≠ j).
   * Jika graf tidak berarah, pengguna hanya mengisi jarak antara kota `i` dan kota `j` untuk `i < j`, lalu secara otomatis disalin untuk `(j, i)`.
   * Untuk menandai tidak adanya jalur antara dua kota, pengguna memasukkan nilai `-1`, yang akan direpresentasikan dengan nilai sangat besar (`u32::MAX / 2`).

2. **Menyusun Matriks Jarak**

   * Matriks `dist` berukuran `n x n` diisi sesuai input.
   * Jika jarak = `-1`, maka `dist[i][j] = no_path_value` (nilai sangat besar) untuk menandai tidak ada jalur.
   * Setelah itu, fungsi `print_matrix` dipanggil untuk menampilkan matriks jarak dalam format tabel.

3. **Algoritma TSP (DP + Bitmask)**

   * Program menggunakan *dynamic programming* dengan bitmasking. Bitmask (`mask`) menyimpan kota-kota yang sudah dikunjungi.
   * Fungsi utama adalah `total_cost(mask, curr, n, &dist, memo, next)`:

     * Parameter `mask` adalah bitmask kota-kota yang sudah dikunjungi.
     * `curr` adalah indeks kota saat ini.
     * Jika semua kota sudah dikunjungi (`mask == (1 << n) - 1`), maka program mengembalikan jarak dari kota `curr` kembali ke kota awal (`0`).
     * Jika hasil untuk `(curr, mask)` sudah ada di `memo`, langsung dikembalikan.
     * Jika belum, coba setiap kota `i` yang belum dikunjungi (`(mask & (1 << i)) == 0`), hitung biaya untuk pergi ke `i` (`dist[curr][i] + total_cost(mask | (1 << i), i, ...)`), dan pilih yang paling kecil.
     * Simpan hasil minimum di `memo[curr][mask]` dan catat kota selanjutnya (`best_next`) di tabel `next[curr][mask]` untuk membangun jalur optimal.

4. **Membangun Jalur Optimal**

   * Setelah `total_cost(1, 0, ...)` menghasilkan biaya minimum, fungsi `construct_path(n, &next)` digunakan untuk membangun jalur:

     * Mulai dari kota `0` dengan `mask = 1` (hanya kota awal sudah dikunjungi).
     * Selama masih ada kota yang belum dikunjungi, lihat `next[curr][mask]` untuk mendapatkan kota berikutnya, tambahkan ke `path`, update `curr` dan `mask`.
     * Setelah mengunjungi semua kota, tambahkan kota `1` (kota awal) di akhir jalur.

5. **Output Hasil**

   * Jika biaya minimum (`min_cost`) lebih besar atau sama dengan `no_path_value`, berarti tidak ada rute valid untuk mengunjungi semua kota dan kembali, lalu program menampilkan pesan bahwa tidak ada jalur valid.
   * Jika ada, program menampilkan:

     * **Biaya perjalanan minimum** (`min_cost`).
     * **Jalur terbaik** dalam format deretan kota yang dipisah dengan `→`.
     * **Jumlah edge** (panjang jalur dikurangi 1).

---

## Penulis
Muhammad Ra'if Alkautsar / 13523011

