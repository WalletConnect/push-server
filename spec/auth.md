# Echo Server Authentication

## Relay <--> Echo Server

We use Ed25519 to verify that requests from the Relay network are valid and intended for the Echo Server Instance that has received them.

### Implementation

Echo Server generates an Ed25519 key-pair and stores that in-memory - or in files specified using environment variables. The generated
public key is then sent to the Relay network along with a cloud app project id and the public url (both from environment variables) so that
future requests to the relay can be signed and validated. The Echo Server instance also fetches the Relay network's public key and caches it
so that requests received can be validated.

### Relay Endpoints

#### GET `/public-key`
##### Response
> **Note**
> This is an example Ed25519 Public Key, this is not a valid Public Key for the Relay Network.
```
693a98827a9c7e8f818af53b9720671eb4d3075815a8c2c8f6d0da12ba1aba7a
```

#### POST `/push/servers`
##### Request
> **Note**
> This is an example request. The `project_id`, `public_key` & `public_url` values are not valid.
```json
{
    "project_id": "83f11e753439fab08222b45e2d029eab",
    "public_key": "d5aa4b55ecf4553c3ef8f8a945d9449394f0b3b7787af049d1d4828037465a4f",
    "public_url": "https://push.walletconnect.com",
    "echo_server": {
      "version": "0.1.0",
      "git": "d0be36e9007ef73e0dafb8d2d9f3172c4d9f8333"
    }
}
```
##### Response

> **Note**
> Responses haven't been confirmed yet.

```json
{
}
```