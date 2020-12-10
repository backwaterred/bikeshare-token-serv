<img src="http://3.bp.blogspot.com/-SK8CvVbZrT0/TqlFEjIS4XI/AAAAAAAAA6s/wM2FyKrlWjk/s1600/PA240019.JPG" alt="bike rider on playground equipment"
	title="Wicket Riding" height="200" />

## Users

### Token Acquisition

Use endpoints `GET /user/token<bike-id>` and `POST /user/token` to get a new
token. Post request must contain a bikeid in the request body.

Where:

- `<bike-id>` is identical to the id passed in to
the original request.
- `<timestamp>` is the number of seconds from Jan 1 1970.

Returns:

- 200 Ok: With a token in the response body.
- 5** Internal Server Error: When unable to acquire system time.

#### Token Format
Tokens are signed (by the server's private RSA key), base64 encoded strings.

Decrypted tokens are of the form:
```TOK=<bike-id>-<timestamp>```

Example decrypted token: `TOK=BIKE001-1234567890`

### Finalize Rental

Complete the rental by submitting a time to `PUT /user/finalize/<duration>` with
the token in the request body.

Where:

- `<duration>` is the length of the ride in seconds.

Returns:

- 200: Ok When the ride is finalized successfully.
- 403: Forbidden When the ride has already been finalized.
- 400: Bad Request When no record of the token is found on the server.

## Management

A bikeshare operator will need to audit the logs on their fleet. These endpoints
may be helpful in doing so.

### Audit

The endpoint `PUT /admin/audit/<duration>` allows admin users to check whether the time
of the rental stored on the on the server under the given token (reported by the
user) match the given time (taken from logs on the bike). The response text indicates audit success or
failure.

Where:

- `<token>` is the signed, base64 token used to rent the bike.
- `<duration>` is a string (part of the URI) indicating the integer number of
  seconds of the rental.
  
### Summary

`GET /admin/summary` returns a report of all tokens issued since the server came
online.

## Development

The following may be useful for development.

### Timestamp

The current system time of the server can be checked with GET `/timestamp`.

### Public Key
 
To get a copy of this server\'s public key. Make a GET request to `/cert`.
 
### Testing

To get back an invalid token (of an expected format for testing purposes), Make
a GET request to `/test`. The token will be formed with bike id: INVALID-TEST-BIKEID and timestamp 0.
 
```TOK=INVALID-TEST-BIKEID-0```
