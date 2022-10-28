Notes:
database:

- Database connection pooling, openning a connection to the database is expensive , we can use connection pool to keep connections open and use then.
- using connection pooling increased performance from 200 request per second to ~7600 request per second
- When using Arrays as NOT NULLABLE in sql migration , Diesel tend to generate schema Array<Nollable<Text>>, need to manually change it to Array<Text>

authentication:

- In actix-web_grants , roles are permission claims with prefix "ROLE_"
