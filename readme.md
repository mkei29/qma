

# Qma
Simple cli tool to aggeregate structured log.
![example](https://user-images.githubusercontent.com/78252208/172173859-78aab528-3c22-4ba0-8179-bd23e8f81846.jpg)

# Usage


## 1. Write table definition. 
Before execute ```qma``` command, prepare the yaml file which describe table you want to output. See "config file details" section for detail.

## 2. Execute command
Execute ```qma``` command. You can use either stdin or file for input.

### Query from log file.
``` bash
qma <CONFIG_PATH> <LOG_FILE_PATH>
```

### Query from stdout
``` bash
SOME COMMAND | qma <CONFIG_PATH>
```

![Example Gif](https://user-images.githubusercontent.com/78252208/172173169-3fabb424-5e99-4ddd-93d5-85d6e55dd5b4.gif)

# Installation
You need either cargo to install qma.

Using cargo
```bash
cargo install qma
```

Using homebrew (only for OSX)
```
brew tap toritoritori29/tools
brew install qma
```

# Config file details

```yaml
# global setting
order_by: count  # (optional). Specify the field name you want to order by.
order: asc  # (optional). Specify order you want to sort.
# index = grouping key settings
index:
    name: method  # Column name on output table.
    accessor: httpRequest.requestMethod  # Json accessor

// fields: Columns list. See `Field setting details` section for details.
fields:
  - name: latency  # Column table on output table.
    accessor: httpRequest.latency.  # Json accessor for target data.
    dtype: second  # data type.
    operation: average # Aggregation method.
  - name: method
    accessor: httpRequest.requestMethod
    dtype: string
    operation: count
``` 

## Global settings
* `index` Grouping key settings. See the 'index' section for detail.
* `fields` Field list to display. See the `field section for detail.
* `order_by` (optional) The field name you want to order by.
* `order` (optional) Order you want to sort.
* `output_format` (optional) Table style you want show. `csv` or `markdown`. Default is `csv`.

## Index settings

* `name` The field name to be displayed in the output table.
* `accessor` Accessor to json property. Join properties with dots like "aaa.bbb.ccc".

## Field settings 

* `name` The field name to be displayed in the output table.
* `accessor` Accessor to json property. Join properties with dots like "aaa.bbb.ccc".
* `dtype` Data type. Choose from `string`, `integer`, `float`, `seconds`. If you specfiy `seconds`, the target data must be numeric data which have 's' or 'sec' as a suffix.
* `operation` Aggregation method. See 'Aggregation methods' section for detail.


# Aggregation methods

|method    |description     |available dtype|
|:---------|:---------|:---------|
|average| Average field| integer, float, seconds|
|count| Count valid data | string, integer, float, seconds|
# Licence
This project is under the MIT license.

