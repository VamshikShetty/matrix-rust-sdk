# matrix-rust-sdk
Matrix client bindings generated by OpenAPI generator using OAS 3.0.0 files 


## Auto generating RUST sdk using [Openapi Generator](https://github.com/OpenAPITools/openapi-generator)
Structure of matrix rust SDK project
```
<path>/
	matrix-rust-sdk/
		openapi-config/
		openapi-specification/
        sdk/
            src/
                api/      ( API methods )
                models/   ( request and response structures)
                lib.rs
        samples/
	openapi-generator/
```
### Command to generate the RUST SDK on our own
```
cd <path>/openapi-generator/

java -jar modules/openapi-generator-cli/target/openapi-generator-cli.jar generate -i ..\matrix-rust-sdk\openapi-specification\client-server.yaml -c ..\matrix-rust-sdk\openapi-config\client-server.json -g rust-server -o ..\matrix-rust-sdk\matrix

```

## Prerequisite: packages need by RUST SDK generated by openapi-generator

## Installing matrix client
```

```

## Running a test samples
1. cargo run --example login_get
2. cargo run --example login_put "\<username\>" "\<password\>"