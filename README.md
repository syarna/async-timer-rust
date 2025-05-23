# Tutorial 10 - Syarna Savitri (2206083565)

## Penjelasan Singkat Tutorial 1.3

Pada eksperimen ini, saya menguji efek dari **multiple spawn** task asynchronous dan pengaruh dari penggunaan `drop(spawner)` terhadap jalannya executor dan proses task.

Kode yang digunakan merupakan sebuah executor sederhana yang menjalankan future yang kita spawn melalui `Spawner`. Task-task ini menggunakan mekanisme waker untuk memberi tahu executor bahwa future-nya siap untuk dipoll ulang.


## Apa itu Spawner, Executor, dan Drop?

- **Spawner** adalah komponen yang bertugas untuk menerima future dari program dan mengubahnya menjadi task yang bisa dijalankan. Spawner kemudian mengirimkan task tersebut ke channel agar executor bisa mengambil dan menjalankannya.

- **Executor** adalah loop utama yang menerima task dari channel dan menjalankan task tersebut. Executor memanggil `poll()` pada future untuk menjalankan task sampai selesai. Jika future belum selesai, executor akan menunggu waker membangunkannya kembali.

- **Drop(spawner)** menutup channel pengirim task. Ini sangat penting karena tanpa menutup channel, executor akan terus menunggu task baru dan tidak akan pernah berhenti. Dengan `drop(spawner)`, channel tertutup dan executor bisa selesai ketika semua task sudah selesai diproses.

---

## Eksperimen: Menghapus dan Mengembalikan `drop(spawner)`


### Percobaan 1: Dengan `drop(spawner)`

1. Screenshot output saat `drop(spawner)` ada (program berhenti normal).
<img width="568" alt="Screenshot 2025-05-23 at 16 25 23" src="https://github.com/user-attachments/assets/5c7fbd20-0580-400b-afe4-fcfbd8a63413" />


Output konsol:

<img width="762" alt="Screenshot 2025-05-23 at 16 24 28" src="https://github.com/user-attachments/assets/09b86233-b0d8-4609-859c-c9dff44e801b" />


**Penjelasan:**

- `drop(spawner)` menutup channel task sender sehingga ketika semua task sudah selesai, executor menyadari tidak ada task baru yang akan datang dan bisa selesai.
- Program berjalan dengan baik, menampilkan semua pesan sesuai urutan, lalu berhenti dengan benar.


### Percobaan 2: Tanpa `drop(spawner)`

2. Screenshot output saat `drop(spawner)` dihapus (program menggantung).
<img width="566" alt="Screenshot 2025-05-23 at 16 29 17" src="https://github.com/user-attachments/assets/3174c900-7e2a-4073-ad3f-58d411a87245" />


Output konsol:

<img width="766" alt="Screenshot 2025-05-23 at 16 29 46" src="https://github.com/user-attachments/assets/0440f1aa-1f49-44b1-8fec-c117784c80cf" />


**Penjelasan:**

- Karena spawner tidak di-`drop`, channel pengirim task tetap terbuka.
- Executor terus menunggu task baru yang bisa dikirim ke channel tersebut, sehingga `executor.run()` tidak pernah selesai.
- Akibatnya, program tidak pernah berhenti walaupun semua task sudah selesai.
- Ini menunjukkan bahwa menutup channel sangat penting agar executor tahu kapan harus selesai.


## Korelasi Antara Spawner, Executor, dan Drop

- **Spawner** membuat dan mengirim task ke executor melalui channel.
- **Executor** menjalankan task sampai selesai.
- **Drop(spawner)** menutup channel yang menandakan tidak ada lagi task yang akan datang.
- Jika channel tetap terbuka, executor akan terus menunggu dan tidak selesai, menyebabkan program menggantung.
- Jika channel ditutup (dengan `drop(spawner)`), executor bisa mengenali akhir tugas dan berhenti.


## Kesimpulan

- Fungsi **`tokio::spawn`** atau `thread::spawn` akan menjalankan future atau thread secara paralel secara asynchronous.
- **Spawner** dan **executor** saling bergantung untuk mengatur lifecycle task asynchronous.
- **Drop** pada spawner (channel sender) adalah sinyal penting untuk menandakan tidak ada lagi task baru sehingga executor bisa berhenti dengan benar.
- Memahami lifecycle ini sangat penting dalam membuat executor dan runtime asynchronous custom.

## Eksperimen 2.1: Kode Asli dan Cara Menjalankannya

Ini adalah aplikasi chat siaran (broadcast chat) yang dibangun menggunakan pemrograman asinkron di Rust dengan dukungan WebSocket. Aplikasi terdiri dari satu server dan beberapa klien yang dapat saling mengirim dan menerima pesan secara real-time.

## 🔧 Cara Menjalankan

1. Ketik pesan di salah satu terminal klien.

Saat salah satu klien mengetik pesan, pesan tersebut dikirim ke server. Server kemudian menerima pesan itu dan **mendistribusikannya (broadcast)** ke semua klien yang sedang terhubung, termasuk pengirimnya sendiri.

Ini menunjukkan implementasi **pemrograman asinkron** yang nyata, di mana banyak koneksi WebSocket dapat ditangani secara bersamaan dengan:
- `tokio::spawn` untuk mengelola koneksi secara paralel.
- `tokio_websockets` sebagai pustaka WebSocket.
- `tokio::sync::broadcast` untuk menyiarkan pesan ke banyak penerima.

Berikut tangkapan layar saat menjalankan 1 server dan 3 klien:
- Tampilan dari sisi Server

<img width="812" alt="Screenshot 2025-05-23 at 17 07 40" src="https://github.com/user-attachments/assets/cb3209eb-c187-436e-9598-bec2d7f062b7" />

- Tampilan dari sisi Client 1

<img width="800" alt="Screenshot 2025-05-23 at 17 08 14" src="https://github.com/user-attachments/assets/e52294f4-78ba-4a80-a171-abd045a5816c" />

- Tampilan dari sisi Client 2

<img width="802" alt="Screenshot 2025-05-23 at 17 08 43" src="https://github.com/user-attachments/assets/ba11598b-4706-48e5-bb1b-3880f69cd593" />

- Tampilan dari sisi Client 3

<img width="802" alt="Screenshot 2025-05-23 at 17 09 28" src="https://github.com/user-attachments/assets/e19ad952-fbad-4c5b-a390-16b77e9b82a6" />









