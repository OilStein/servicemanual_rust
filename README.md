# servicemanual_rust

## Synopsis

Simple REST API that writes maintenance tasks and list them.
- Application can create, list, modify and delete maintenance tasks
- Application can create and list devices
- Application can get task(s) by device id
- Application orders tasks by severity and date
## Used Tech
- Rust
- Actix - Web framework
- SurrealDB - Database

## REST endpoints

- get /device/{id}
- get post - /devices
- get / post / put / delete - /maintenance/{id}
- get /maintenances
- get /maintenances/{did}

## HOW TO USE

First you need to install Rust to run application.
Just follow the instructions: link here

To run application go to root folder with command line and type 'cargo run'

Application populates database with mock data. Database runs in localhost:8080

When application is running you can use Postman, Thunder Client or similar for calls
When querying with id, take only the uuid part from id. SurrealDB ids are in form of "device:uuid"

Example querys:

Returns all devices in json
'GET localhost:8080/devices' 

Return one device. Remember only uuid like part in ids place
'GET localhost:8080/device/{id}' 

Creates device in database
'POST localhost:8080/device'

Add json in body:
'{
  "name": "Kone EMAD",
  "year": 2015,
  "model": "Elevator",
}'

"id" is optional in json.


## Missing parts for better app
- Better SurrealQL querys
- Enum types for severity and status
- Date in local time, now in utc form..