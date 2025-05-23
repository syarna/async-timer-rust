# Tutorial 10 - Syarna Savitri (2206083565)

## 🧪 Penjelasan Singkat Experiment 1.3

Pada eksperimen ini, saya menguji efek dari **multiple spawn** task asynchronous dan pengaruh dari penggunaan `drop(spawner)` terhadap jalannya executor dan proses task.

Kode yang digunakan merupakan executor sederhana yang menjalankan future yang kita spawn melalui `Spawner`. Task-task ini menggunakan mekanisme waker untuk memberi tahu executor bahwa future-nya siap untuk dipoll ulang.

---

## 🧠 Apa itu Spawner, Executor, dan Drop?

- **Spawner**: Menerima future dari program dan mengubahnya menjadi task yang bisa dijalankan. Task dikirim melalui channel ke executor.
- **Executor**: Loop utama yang memanggil `poll()` pada future dan mengeksekusi task sampai selesai. Jika future belum selesai, executor menunggu waker.
- **`drop(spawner)`**: Menutup channel pengirim task. Ini penting agar executor tahu bahwa tidak ada lagi task yang akan dikirim dan bisa berhenti.

---

## 🧪 Eksperimen: Menghapus dan Mengembalikan `drop(spawner)`

### ✅ Percobaan 1: Dengan `drop(spawner)`

> Program berhenti normal

📸 Screenshot:

![drop present](https://github.com/user-attachments/assets/5c7fbd20-0580-400b-afe4-fcfbd8a63413)

📋 Output:

![output normal](https://github.com/user-attachments/assets/09b86233-b0d8-4609-859c-c9dff44e801b)

**Penjelasan:**
- `drop(spawner)` menutup channel sehingga executor tahu tidak ada task baru yang dikirim.
- Program berjalan normal dan selesai dengan benar.

---

### ❌ Percobaan 2: Tanpa `drop(spawner)`

> Program menggantung

📸 Screenshot:

![no drop](https://github.com/user-attachments/assets/3174c900-7e2a-4073-ad3f-58d411a87245)

📋 Output:

![output hang](https://github.com/user-attachments/assets/0440f1aa-1f49-44b1-8fec-c117784c80cf)

**Penjelasan:**
- Channel tetap terbuka karena `drop(spawner)` dihapus.
- Executor menunggu task baru selamanya meski semua task sudah selesai.
- Program tidak pernah berhenti.

---

## 🔗 Korelasi antara Spawner, Executor, dan Drop

- Spawner membuat dan mengirim task.
- Executor menjalankan task hingga selesai.
- `drop(spawner)` adalah sinyal bahwa tidak ada task baru → executor bisa berhenti.
- Jika tidak di-drop, executor akan menggantung karena menunggu terus.

---

## 🧾 Kesimpulan Eksperimen 1.3

- Fungsi seperti `tokio::spawn` atau `thread::spawn` menjalankan future secara asynchronous.
- Spawner dan executor saling bekerja untuk mengatur task lifecycle.
- `drop(spawner)` sangat penting agar executor tahu kapan harus berhenti.

---

## 📡 Penjelasan Eksperimen 2.1: Broadcast Chat Server

Ini adalah aplikasi **chat siaran (broadcast chat)** berbasis WebSocket dan pemrograman asinkron di Rust. Komponen:
- 1 server WebSocket
- Beberapa client

### Cara kerja:
1. Klien mengetik pesan → dikirim ke server.
2. Server menerima dan **menyiarkan** pesan ke semua klien yang terhubung (termasuk pengirim).

Teknologi:
- `tokio::spawn`: Menangani banyak koneksi paralel.
- `tokio_websockets`: Library untuk WebSocket.
- `tokio::sync::broadcast`: Menyiarkan pesan ke semua client.

📸 Screenshot:

- **Server**
  ![server](https://github.com/user-attachments/assets/cb3209eb-c187-436e-9598-bec2d7f062b7)
  
- **Client 1**
  ![client1](https://github.com/user-attachments/assets/e52294f4-78ba-4a80-a171-abd045a5816c)
  
- **Client 2**
  ![client2](https://github.com/user-attachments/assets/ba11598b-4706-48e5-bb1b-3880f69cd593)
  
- **Client 3**
  ![client3](https://github.com/user-attachments/assets/e19ad952-fbad-4c5b-a390-16b77e9b82a6)

---

## 🔧 Eksperimen 2.2: Mengubah Port WebSocket

Perubahan:
- Port dari `2000` diubah ke `8080`

### Kode yang diubah:

- **Server**: binding diubah menjadi `127.0.0.1:8080`

```rust
let listener = TcpListener::bind("127.0.0.1:8080").await?;
```

📸 Screenshot:

![server port](https://github.com/user-attachments/assets/1a94905c-3594-406b-8c9c-4d3f99595bb6)

- **Client**: URI koneksi diubah ke `ws://127.0.0.1:8080`

```rust
let url = url::Url::parse("ws://127.0.0.1:8080")?;
```

📸 Screenshot:

![client port](https://github.com/user-attachments/assets/a18f918f-60e4-4c84-995b-69050a82dba8)

---

## ✅ Kesimpulan Eksperimen 2.2

- Modifikasi port berhasil.
- WebSocket tetap berfungsi dengan baik setelah perubahan port di sisi server dan client.

---
