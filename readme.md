

# QMA
Simple command line tool for aggeregate structured log.


# Usage
Before runnning qma, prepare the yaml file which describe table definition config like below.
See here to detail.

## Query from log file.
``` bash
qma <CONFIG_PATH> <LOG_FILE_PATH>
```

## Query from stdout
``` bash
SOME COMMAND | qma <CONFIG_PATH>
```

# Installation


# Config file example.

```
// index = group key settings
index:
    name: method  // Column name on output table.
    accessor: httpRequest.requestMethod  // Json accessor

// fields: Columns on the 
fields:
  - name: latency  // Column table on output table.
    accessor: httpRequest.latency.  // Json accessor for target data.
    dtype: second  // data type.
    operation: average // Aggregation method.
  - name: method
    accessor: httpRequest.requestMethod
    dtype: string
    operation: count
``` 


## Field setting details.