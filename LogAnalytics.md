#### Log Analytics
```
cd log_analytics
```

### Assumptions
the **test.log** file is in parent directory of log_analytics
Ignore requests that **does not** have any of the following HTTP VERBS.
```
GET
POST
PUT
OPTION
CONNECT
HEAD
PATCH
DELETE
TRACE
PROPFIND
```

#####  Find number of HTTP requests in the log using
```

./number_of_http_requests.sh
```

#####  Top 10 hosts issuing HTTP requests between the given dates
###### dates were hardcorded for simplicity
```

./top_10_hosts.sh
```

#####  Top 10 hosts issuing HTTP requests

```

./highest_traffic_country.sh
```