# Rust Playground

Multiple projects to learn Rust, axum and server-related libs in depth.

## List of objectives for this project:
_Each objective has a `-> (resulting explored concept list)` and a detailed list of sub-objectives._
- User able to create an account & login / logout -> (Middleware, Authentication, JWT, DB Access)
  1. Simple login/password authentication
  2. Generate, validate & invalidate auth-token cookie
  3. Add `/user` endpoint with required auth
  4. Save data in DB with encrypted password & salt
  5. Modify logic to use a JWT instead
- Todo app per user
  - DB storing, data relation
- Chat app between 2 users & in a chatroom
  - Websockets, Authentication, fast DB access
- URL redirection service
  - data transformation, redirection
- Full server wipe every x hours
  - automation
- Dockerized, put on AWS and hosted & available online
  - Docker, AWS, Networking, hosting

_List is not exhaustive, other objectives might be added_

## Goals for this Project
- Improve with Rust and WebServ concepts
- Learn Websockets
- Learn Axum, Tower, Tokio, Serde, Strum and multiple popular crates.
