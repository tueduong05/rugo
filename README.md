# Rugo - URL Shortening Service

Rugo is a simple URL Shortening Service made as a learning project.

## Development

### Requirements

To set up and run Rugo, you will need the following tools:

- Docker
- Docker Compose
- sqlx-cli

## Setup

Follow these steps to set up the development environment:

### 1. Clone the repository:

```
git clone https://github.com/tueduong05/rugo.git
cd rugo
```

### 2. Start the Docker containers:

```
docker-compose up -d
```

### 3. Create the database:

```
sqlx database create
```

### 4. Run the database migrations:

```
sqlx migrate run
```

## Run

Once the setup is complete, ensure the database is running and execute the following command to start the application:

```
cargo run
```

## API Documentation

### 1. Create Short URL

```
POST /shorten
```

#### Request Body

```json
{
  "url": "https://www.example.com/some/long/url"
}
```

#### Responses

- **201 Created:**

```json
{
  "id": "1",
  "url": "https://www.example.com/some/long/url",
  "shortCode": "abc123",
  "createdAt": "2021-09-01T12:00:00Z",
  "updatedAt": "2021-09-01T12:00:00Z"
}
```

- **400 Bad Request:**
  Returned in case of validation errors with error messages.

### 2. Retrieve Original URL

```
GET /shorten/{shortCode}
```

#### Responses

- **200 OK:**

```json
{
  "id": "1",
  "url": "https://www.example.com/some/long/url",
  "shortCode": "abc123",
  "createdAt": "2021-09-01T12:00:00Z",
  "updatedAt": "2021-09-01T12:00:00Z"
}
```

- **404 Not Found:**
  Returned if the short URL was not found.

### 3. Update Short URL

```
PUT /shorten/{shortCode}
```

#### Request Body

```json
{
  "url": "https://www.example.com/some/updated/url"
}
```

#### Responses

- **200 OK:**

```json
{
  "id": "1",
  "url": "https://www.example.com/some/updated/url",
  "shortCode": "abc123",
  "createdAt": "2021-09-01T12:00:00Z",
  "updatedAt": "2021-09-01T12:30:00Z"
}
```

- **400 Bad Request:**
  Returned in case of validation errors.

- **404 Not Found:**
  Returned if the short URL was not found.

### 4. Delete Short URL

```
DELETE /shorten/{shortCode}
```

#### Responses

- **204 No Content:**
  Returned if the short URL was successfully deleted.

- **404 Not Found:**
  Returned if the short URL was not found.

### 5. Get URL Statistics

```
GET /shorten/{shortCode}/stats
```

#### Responses

- **200 OK:**

```json
{
  "id": "1",
  "url": "https://www.example.com/some/long/url",
  "shortCode": "abc123",
  "createdAt": "2021-09-01T12:00:01Z",
  "updatedAt": "2021-09-01T12:00:00Z",
  "accessCount": 10
}
```

- **404 Not Found:**
  Returned if the short URL was not found.

## References

https://roadmap.sh/projects/url-shortening-service
