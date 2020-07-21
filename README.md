This repository is a working example of a Rust+Rocket HTTP server used for asking a question on differentiated replies.

# TL;DR

 1: How do I validate fields in a POSTed Json document like the [`FromForm` trait](https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html) ?
 2: How do I respond a differentiated Json document (perhaps none at all) and HTTP code based on the validation done in 1) ?

# Background
Validating the fields in a form submission is supported nicely in the "Rocket-way" as described [here](https://rocket.rs/v0.4/guide/requests/#forms) 
and [here](https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html) 
but when it comes to validating submitted json documents the documentation is not as helpfull, I have found. As a matter of fact, I have been unable to figure out how to validate fields in a json document in a "Rocket Paradigm" conformant way, which I why I ask question 1).

Depending on the result of such a validation, I might respond different response codes and, more importantly, with Json documents of differenc schemas.

# Example


## What I Have
This repository contains a working server that takes a message document as POST and responds a message document back
```rust
struct Message {
    id: Option<u8>,
    contents: String
}
```

If no id is provided, in the submitted document, an id of 2 is set in the response and the responded message tells you what you said like this:

```bash
$ curl -D - -H "Content-Type: application/json" -d '{ "contents":"hello"}' -X POST http://localhost:8000/message

HTTP/1.1 200 OK
Content-Type: application/json
Server: Rocket
Content-Length: 39
Date: Tue, 21 Jul 2020 07:46:33 GMT

{"id":2,"contents":"You said: 'hello'"}
```

If you provide an id, the responded id is doubled like this


```bash
$ curl -D - -H "Content-Type: application/json" -d '{"id": 44, "contents":"hello"}' -X POST http://localhost:8000/message

HTTP/1.1 200 OK
Content-Type: application/json
Server: Rocket
Content-Length: 40
Date: Tue, 21 Jul 2020 07:47:07 GMT

{"id":88,"contents":"You said: 'hello'"}
```

## What I Would Like

Now, the id 42 is reserved, so submitting an id of 42 should reply the following:

```bash
$ curl -D - -H "Content-Type: application/json" -d '{"id": 42, "contents":"hello"}' -X POST http://localhost:8000/message

HTTP/1.1 400 Bad Request
Content-Type: application/json
Server: Rocket
Content-Length: 45
Date: Tue, 21 Jul 2020 07:47:07 GMT

{"reason" : "42 is reserved for special use"}
```
A http status code of 400 and a document of different schema.


Assuming that messages are actually stored somewhere, and you try to submit an id that is already taken, (which we hardcode to 27 for the sake of example) I would like an empty document returned and a 409 status code like this:

```bash
$ curl -D - -H "Content-Type: application/json" -d '{"id": 27, "contents":"hello"}' -X POST http://localhost:8000/message

HTTP/1.1 409 Conflict
Server: Rocket
Content-Length: 0
Date: Tue, 21 Jul 2020 07:47:07 GMT
```
# How do I achieve that...
... in a Rocked aligned way?
