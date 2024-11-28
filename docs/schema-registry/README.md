# Schema registry

⚠️ The support of the schema registry in Yozefu is **highly experimental**. Contributions and feedback are welcome:

| Types       |               Support |
| ----------- | :-------------------- |
| Json schema |          Experimental |
| Avro        |          Experimental |
| Protobuf    | Not supported for now |



You can configure the tool to use a schema registry. Open the configuration file `yozf configure`, and add a `schema_registry` entry to your cluster:
```json
{
    "clusters": {
        "localhost": {
            "url_template": "http://localhost:9000/ui/kafka-localhost-server/topic/{topic}/data?single=true&partition={partition}&offset={offset}",
            "schema_registry": {
              "url": "http://localhost:8081"
            },
            "kafka": {
              "bootstrap.servers": "localhost:9092",
              "security.protocol": "plaintext",
              "broker.address.family": "v4"
            }
        }
    }
}
```





## Basic auth

If the schema registry is protected by a basic authentication, you can add the `Authorization` header:

```json
{
    "schema_registry": {
        "url": "https://acme-schema-registry:8081",
        "headers": {
            "Authorization": "Basic am9obkBleGFtcGxlLmNvbTphYmMxMjM="
        }
    }
}
```


## Bearer token

```json
{
    "schema_registry": {
        "url": "https://acme-schema-registry:8081",
        "headers": {
            "Authorization": "Bearer <bearer-token>"
        }
    }
}

```




## Authentication methods per provider


| Nom                                                                                 | Authentication methods          |
| ----------------------------------------------------------------------------------- | ------------------------------- |
| [Confluent](https://confluent.cloud)                                                | Basic authentication<br>OAuth   |
| [Redpanda Cloud](https://docs.redpanda.com/current/manage/security/authentication/) | Basic authentication<br />OAuth |



