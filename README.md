# WEATHER BOYZ

[WeatherXM Hackathon](https://plgenesis.devspot.app/en?activeTab=overview&challenge=weather-xm)

## The One Sentence Description

A web3 application that allows users to buy weather insurance using weatherXM data to verify weather conditions and execute smart contracts.

## Setting up the Database

1. Download Docker

2. Create a docker-compose.yml file in `backend/docker/`. Use the provided `docker-compose.yml.sample` file and change the password as need be.

3. Create a `.env` file in `/backend`. Add the following lines to it:

   ```
   DATABASE_URL=postgres://<username>:<password>@<host>:<port>/<database>
   JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
   ```

   Replacing the placeholders with the values in your `docker-compose.yml` file.

   For example, this would be value when we use the default values from the `docker-compose.yml.sample` file.

   ```
   DATABASE_URL=postgres://postgres:password@localhost:5432/weather-boyz-db
   JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
   ```

4. Cd into `backend/docker/` and run `docker-compose up -d`.
   Running that command with the `-d` flag means that it will be detached from the terminal and run in the background.

5. Install the sqlx-cli. You can do this globally via:

   ```
   cargo install sqlx-cli --no-default-features --features native-tls,postgres
   ```

   For our database, we will be using postgres.

6. Ensure your terminal is within the `/backend` folder. Assuming you have created the docker container, you can now run `sqlx database create` to create our database. This will create our database.

7. Finally run `sqlx migrate run`. This will run the migration files within `backend/migrations`.

> If you want to look at the Database in Docker, search for "postgres" in the searchbar at the top (If your container is active, you should be able to find an option to activate a terminal for it.) or use the 'Exec' option if you click on your used container. From there, you can sign into the postgres db with `psql -U USERNAMEHERE`.
