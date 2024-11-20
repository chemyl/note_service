cargo install sqlx-cli --features sqlite

cargo sqlx prepare 

create dir -> `migrations`

cargo sqlx migrate add create_notes_table
    add migration script

    add db file note_service.db
    add `.env`

cargo sqlx migrate run

cargo run

curl http://127.0.0.1:8080/notes
curl -X POST http://127.0.0.1:8080/notes -H "Content-Type: application/json" -d '{"title": "Test Note", "content": "This is a test note."}'
