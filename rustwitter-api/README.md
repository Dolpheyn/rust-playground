# Rustwitter API

Backend for twitter application written in Rust
## Crates used
1. ```Tide```
A HTTP server with async/await
2. ```async-std``` 
Crate that tide depends on for async/await features
3. ```sqlx```
For database connection, using the postgres feature to connect to psql db.
4. ```dotenv```
To load environment variable from ```.env``` file
5. ```pretty_env_logger```

6. ```chrono```
For DateTime in Rust
7. ```log```

8. ```serde```

9. ```serde_json```

10. ```uuid```

11. ```thiserror```
Provides macro to cleanly implement application's own error type.
So we can ```?``` at all errors woohooo!!
