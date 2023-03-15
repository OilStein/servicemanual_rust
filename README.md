# servicemanual_rust

## Synopsis

Simple REST API that writes maintenance tasks and list them.
- Application can create, list, modify and delete maintenance tasks
- Application can create and list devices
- Application can get task(s) by device id
- Application orders tasks by severity and date
## Used Tech
- [Rust](https://www.rust-lang.org/learn/get-started)
- [Actix](https://actix.rs/)
- [SurrealDB](https://surrealdb.com/)

## REST endpoints

General rule: When getting one thing endpoint is /device or /maintenance. When getting multiple things endpoint is /devices or /maintenances

- get /device/{id}
- get post - /device
- get / post / put / delete - /maintenance/{id}
- get /maintenances
- get /maintenances/{did}

## HOW TO USE

First you need to install Rust to run application.
Just follow the instructions: link here

To run application go to root folder with command line and type `cargo run` and press enter.

Application populates database with mock data. 
Application runs in localhost:8080

When application is running you can use Postman, Thunder Client or similar for calls
When querying with id, take only the uuid part from id. SurrealDB ids are in form of "table:id"

## Example querys:

### Returns all devices in json

`GET localhost:8080/devices` 

### Return one device. 

`GET localhost:8080/device/{id}` 

### Creates device in database
`POST localhost:8080/device`
with json body:
```json
{
  "name": "Kone EMAD",
  "year": 2015,
  "model": "Elevator"
}
```
"id" is optional in json when creating devices or tasks.
Written id will be overwritten.

### Returns json list of all tasks. Ordered by severity and date
`GET localhost:8080/maintenances`

### Returns list of tasks with corresponding device id
`GET localhost:8080/maintenances/{did}`

### Returns a single task
`GET localhost:8080/maintenance/{id}`

Creates a task and returns it
`POST localhost:8080/maintenance`
with json body 
```json
{
  "did": "device id",
  "desc": "Electrical fault in the door",
  "severity": "important"
}
```

### Updates the task and returns it
`PUT localhost:8080/maintenance/{id}`
with json body 
```json
{
  "desc": "Fixed. Rats in the shaft",
  "severity": "important",
  "status": "closed"
}
```
All json record are optional on update call

### Deletes the task and return id
`DELETE localhost:8080/maintenance/{id}`

## Known issues
- Doesn't check id existence in database, when posting
## Missing parts for better app
- Better SurrealQL querys
- Enum types for severity and status
- Date in local time, now in utc form..
- Tests. Compiler warns a lot of possible runtime errors.