# Axum playground

## Postgresql

* Install on Debian with `apt install postgresql`.
* Connect with the `postgres` user: `sudo su - postgres`.
* Then run these commands:

```
createuser -DRS your_username
createdb -O your_username your_dbname
```

It's important that `your_username` is your unix username.

* Come back at your normal user, and add this to your `.env` file:

```
DATABASE_URL=postgres:///your_dbname
```

(Note: `postgres:your_dbname` would mostly work except for https://github.com/SeaQL/sea-orm/issues/2647)

## Run

Run with `cargo run`.

Then going to http://localhost:3000 will list all the users from your local
database. So make sure to add a few!
