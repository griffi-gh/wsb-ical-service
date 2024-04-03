# wsb-ical-service

Calendar provider for upcoming WSB Merito lectures based on the mobile app API.\
Mostly a proof-of-concept thing for now, but provides some extra features such as:

- Teams joinup links directly in events, no need to log into the web portal every time!
- Higer reliability, as the official ical service frequently goes down for maitenance

(In the future, it may store/cache historical data, as it's regurarly purged from the official ical export feature.)

Requires authentication based on the api key.\
As this project is not "production-ready" yet, it must be pasted into the database manually!

The oauth token can be acquired by doing a `POST` request to `https://oauth.wsb.poznan.pl/token` with following JSON data:

```json
{
  "grant_type": "password",
  "username": "<username>",
  "password": "<password>",
  "client_id": "OhK2xohyuphi5aephoo3uquichooxuu0mohbaixuNgieD8yeiziequai4iqu4thesh3oongeinae1osu",
  "client_secret": "ohKungiifiepeejivoazoothoo4quieB5aen8chiesiPee1voZ9uTahs9heLah5Ai3Shurohsh6ceeSh"
}
```

(please note that client id and secret strings may change in the future, they can be easily extracted from the Android app by a simple text search)

You need both `access_token` and `refresh_token`, the software will refresh the token automatically as it expires.
`identity` field can be set to any string (preferably uuid), but if you're hosting this service publicly, make sure to keep it secret as it will allow anyone to view your upcoming lectures (or spam the server on your behalf :p).

After setting everything up, calendar should be accessible on:
`http://127.0.0.1:3030/api/v1/ical?identity=<identity>`
