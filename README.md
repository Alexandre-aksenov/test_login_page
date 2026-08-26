# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

# Structure of the project 
The global structure is described as three interacting components:
* **Frontend**: each function with attribute `#[component]` in `src/main.rs` defines a webpage or global structure of the website. They can call the functions in `src/lib.rs` or those at the beginning of `src/main.rs`. They are compiled to Web-Assembly, run in the browser and are responsible for:
    * hashing the password during an attempt to login or sign up;
    * the mini-game (available in the 2nd part of the component `Login`, which opens after the user signs in).

* **Middleware** : transmits the calls from Frontend to the Database. It can be found in functions with attribute `#[server()]` in `main.rs`, compiled to regular binary and reveals useful for:
    * communicating with Database as the SQL-client libraries (such as `postgres`, `tokio-postgres`) cannot be compiled to WASM;
    * obfuscating the queries for a potential attacker: as the SQL-queries are formed in Middleware, they are absent in the requests posted by the Frontend (browser).

* **Backend**: SQL-server (RDBMS: PostgreSQL 17), which should be initialized by the scripts in folder `Database` before starting the App, and is hosted locally at `http://localhost:5433`. It is responsible for keeping the information about users, connections, the users' progress.

# Object of the mini-game.

The mini-game is meant to be replaceable by a version of the game of chess endgames (implemented in pure frontend), available at: `https://endgame-wasm-minimal-holy-acorn-9027.fly.dev/`.

After logging in, the player is presented with the FEN encoding of a position, and is prompted to write the first move (extendable to three moves). If the move is correct (`c6c7`), he is presented with the opponent's reply and has the option to save his progress. The button Save inserts the corresponding row the Database.

Further options should be added soon:
* load a saved game, 
* record the achievement of completing a level,
* pass to the 2nd and 3rd level after completing the previous one.

In the full game, the game should reply no matter whether the player's move is correct or not. The player will be presented with a chessboard, and will be able to save the progress toward a dead end.

# Possible improvements.

The current prototype shows the technical possibilities, but
the following additions seem necessary:
* Interaction frontend-backend:
    * load a saved game (Frontend button and backend function);
    * record completing a level (backend procedure);
    * record a name for each level to be shown to the user.
* Backend:
    * Periodic cleaning the old connections;
    * a more secure login procedure (hashing on both sides or Zero-Knowledge verification);
    * analytic reports on recent connections (frequency, levels, number of attempts, technical features ...);
    * users `admin` and `game` (with access to onlyDB procedures and functions) for enhanced security on DB side.


