# Rust Beatport client

Beatport has an official v4 API, but little interest in hobbyists. You can authenticate at https://api.beatport.com/v4/docs/ and inspect the network traffice for a `POST /o/token`. The response will have the `access_token` you can use to authenticate requests.
