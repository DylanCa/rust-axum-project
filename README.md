# Rust Playground - Multiple projects to explore Rust ecosystem !

Multiple projects to learn Rust, axum and server-related libs in depth.

## List of objectives for this project:
_Each objective has a `-> (resulting explored concept list)` and a detailed list of sub-objectives._
- ✅ User able to create an account & login / logout -> (Middleware, Authentication, JWT, DB Access)
  1. ✅ Simple login/password authentication
  2. ✅ Save data in DB with encrypted password & salt
  3. ✅ Generate, validate & invalidate auth-token cookie
  4. ✅ Add `/user` endpoint with required auth
- ✅ Note app per user
  - ✅ DB storing, data relation
- ✅ Word-based URL redirection service
  - ✅ -> Generate shortened URL with words (like bit.ly/flamingo-test-ocean)
  - ✅ data transformation, redirection
- ✅ Setup Tracing
 

- 🔄 Chat app between 2 users & in a chatroom
  - ✅ Websockets / SocketIO connections
  - 🔄 User Authentication using same credentials as api
  - 🔄 DB message storing & recovering (when user joins a chatroom)
  - **_DB access seems quite difficult to properly handle with current tech stack (SocketIOxide + State + sqlx)_**


- Full server wipe every x hours
  - automation
- Dockerized, put on AWS and hosted & available online
  - Docker, AWS, Networking, hosting

## Goals for this Project
- Improve with Rust and WebServ concepts
- Learn Websockets / SocketIO
- Learn Axum, Tower, Tokio, Serde, Strum, Tracing, SocketIOxide and multiple popular crates.
