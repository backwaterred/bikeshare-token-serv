## Users

### Token Acquisition

This endpoint allows users to make a POST request to `/token` with the bike-id as the
message body or a GET request to `/token/<bike-id>`.

Tokens are signed, base64 encoded strings. Decrypted tokens are of the form:
```TOK=<bike-id>-<timestamp>```

Where:

- `<bike-id>` is identical to the id passed in to
the original request.
- `<timestamp>` is the number of seconds from Jan 1 1970.


eg. `TOK=BIKE001-1234567890`

## Management

A bikeshare operator will need to audit the logs on their fleet. These endpoints
may be helpful in doing so.

### Audit

The endpoint `GET /admin/audit/<token>/<duration>` allows admin users to check whether the time
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
