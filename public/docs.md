# Token

Getting a token is easy! Make a POST request to `/token` with the bike-id as the
message body.

Tokens are signed, base64 encoded strings. Decrypted tokens are of the form:
```TOK=<bike-id>-<timestamp>```

Where:

- `<bike-id>` is identical to the id passed in to
the original request.
- `<timestamp>` is the number of seconds from Jan 1 1970.


eg. `TOK=BIKE001-1234567890`

# Timestamp

**Not for Production use**

The current system time of the server can be checked with GET `/timestamp`.

# Public Key
 
**Not for Production use**
 
To get a copy of this server\'s public key. Make a GET request to `/cert`.
 
# Testing

**Not for Production use**
 
To get back an invalid token (of an expected format for testing purposes), Make
a GET request to `/test`. The token will be formed with bike id: INVALID-TEST-BIKEID and timestamp 0.
 
