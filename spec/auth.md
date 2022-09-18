# Echo Server Authentication

## Relay <--> Echo Server

We use Ed25519 to verify that requests from the Relay are valid and intended for the Echo Server Instance that has received them.

### Implementation

Echo Server generates<sup>[[1]](#generating-keys)</sup> an Ed25519 key-pair and stores that in-memory - or in files specified using environment variables. The generated
public key is then sent to the Relay along with a cloud app project id and the public url (both from environment variables) so that
future requests to the relay can be signed and validated. The Echo Server instance also fetches the Relay's public key and caches it
so that requests received can be validated.

On start-up Echo Server sends [the register request](#post-pushservers) to the relay<sup>[[2]](#registering-with-relay)</sup>. This request is formatted using the
public key from the above key-pair as well as the provided `projectId` and `publicUrl` (both provided via Environment Variables).

### Relay Endpoints

#### GET `/public-key`
##### Response
> **Note**
> This is an example Ed25519 Public Key, this is not a valid Public Key for the Relay.
```
693a98827a9c7e8f818af53b9720671eb4d3075815a8c2c8f6d0da12ba1aba7a
```

#### POST `/push/servers`
##### Request
> **Note**
> This is an example request. The `projectId`, `publicKey` & `publicUrl` values are not valid.
```json
{
    "projectId": "83f11e753439fab08222b45e2d029eab",
    "publicKey": "d5aa4b55ecf4553c3ef8f8a945d9449394f0b3b7787af049d1d4828037465a4f",
    "publicUrl": "https://push.walletconnect.com",
    "echoServer": {
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

### Notes
#### Generating Keys
If an environment variable is provided to a path that exists Echo Server will treat that as the private key that it should use and
as such will not generate a key. It will instead read the provided file and attempt to parse that as an Ed25519 private key.

#### Registering with Relay
In this implementation the public key, is sent everytime a node starts up. This ensures that the relay always has up-to date
information for the Echo Server instance. If you have multiple Echo Server instances ensure that all the private keys are the same.