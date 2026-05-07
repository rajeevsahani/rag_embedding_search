# 🚀 RAG Embedding Search Engine in Rust

A high-performance Retrieval-Augmented Generation (RAG) embedding search engine built with Rust.

This project provides:

- ⚡ Fast vector-based semantic search
- 🧠 Embedding generation service integration
- 🗄️ Database-backed retrieval
- 🧩 Modular Rust architecture
- 🔍 REST API search endpoint
- 📦 Clean scalable backend structure

---

# 📁 Project Structure

```bash
src/
├── clients/
│   ├── embedding_client.rs
│   └── mod.rs
│
├── config/
│   ├── db.rs
│   ├── log.rs
│   └── mod.rs
│
├── errors/
│   ├── app_error.rs
│   └── mod.rs
│
├── handlers/
│   ├── mod.rs
│   └── search_handler.rs
│
├── models/
│   ├── mod.rs
│   └── search.rs
│
├── repositories/
│   ├── mod.rs
│   └── search_repository.rs
│
├── services/
│   ├── cache.rs
│   ├── mod.rs
│   └── search_service.rs
│
└── main.rs
```

---

# ⚙️ Features

## ✅ Semantic Search

Uses embeddings to retrieve semantically relevant documents instead of keyword-only matching.

---

## ✅ Modular Architecture

The project follows clean backend separation:

- Handlers → API layer
- Services → Business logic
- Repositories → Database operations
- Clients → External API integrations

---

## ✅ Embedding Service Integration

`embedding_client.rs` handles communication with embedding generation APIs.

---

## ✅ Error Handling

Centralized application error management using custom Rust error types.

---

## ✅ Caching Layer

Basic caching support included for improved retrieval performance.

---

# 🛠️ Tech Stack

- Rust
- Axum
- Tokio
- Serde
- PostgreSQL
- Vector Embeddings
- REST APIs

---

# 🚀 Getting Started

## 1️⃣ Clone Repository

```bash
git clone https://github.com/rajeevsahani/rag_embedding_search.git
cd rag_embedding_search
```

---

## 2️⃣ Run Project

```bash
cargo run
```

---

# 📦 Build

```bash
cargo build --release
```

---

# 🔍 Example API Flow

```text
User Query
    ↓
Embedding Generation
    ↓
Vector Similarity Search
    ↓
Top Relevant Results
    ↓
Response Returned
```

---

# 📚 Future Improvements

- Hybrid search (BM25 + Vector)
- Redis caching
- Streaming responses
- OpenAI integration
- pgvector optimization
- Authentication
- Docker deployment
- Kubernetes scaling

---

# 🧠 Why Rust for RAG?

Rust provides:

- Memory safety
- High performance
- Low latency
- Concurrency without runtime overhead
- Excellent backend scalability

Perfect for production-grade AI retrieval systems.

---

# 👨‍💻 Author

Rajeev Sahani

GitHub:  
https://github.com/rajeevsahani

---

# ⭐ Support

If you found this project useful:

- Star the repository
- Fork the project
- Contribute improvements

---

# 📄 License

MIT License