# [Rust Calendar](https://github.com/ArtyomZaycev/rust-calendar) Frontend

Frontend can be built for both web (using Trunk and WASM) and native.

## Modules

### [Application](/src/app/)<br>
High-level module with definitions for controlling the application itself. CalendarApp controls the UI and manages user input.

### [Database](/src/db/)<br>
Module designed to ease access to the server. DbConnector struct manages requests to the server (sending the request, receiving the result and converting it into the needed struct).

### [State](/src/state/)<br>
Module contains everything related to high-level state data management; it provides structures that store loaded records and give the ability to load/change the data.

UserState stores every loaded record related to 1 user.<br>
State stores every loaded record related to the logged-in user. Including AdminState (if user has an Admin or SuperAdmin role) and UserState of other users, if logged in user was given access to them.<br>
StateUpdater singleton gives the ability to reactively react to changes in State.<br>
StateTable stores data related to 1 table and provides functions for CRUD operations. It does not define the CRUD operation function if this request is not implemented.<br>

### [UI](/src/ui/)<br>
Module contains definitions of some basic and advanced app-specific UI widgets. It implements a pop-up window management system for controlling which pop-ups are open at this time, giving easy access to their data and the ability to close them from any part of the code.

## Used technologies

* [Trunk](https://trunk-rs.github.io/trunk/) - WASM web application bundler.
* [egui](https://www.egui.rs/) - immediate mode GUI library.
* [reqwest](https://docs.rs/reqwest/latest/reqwest/) - HTTP client.
* [tokio](https://tokio.rs/) - asynchronous runtime library.
