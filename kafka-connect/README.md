
# Kafka Connect Setup

### *For use with snarkVMkafka fork that emits kafka metrics*.


The setup flow is as follows (preferably on a VM instance):
* Ensure that the SnarkVM dependency in the main `Cargo.toml` is set to the kafka-queue-test branch of github.com/miazn/snarkvmkafka
* `docker-compose up -d` in this directory to spin up kafka, zookeeper, and kafka-connect. Kafka-connect is simply an intermediary between the kafka producers and our BigQuery data warehouse
* Wait a minute or two, it takes a second to spin up all the containers. Run `docker ps` to ensure that you've ended up with 3 containers: `kafka`, `zookeeper`, and `kafka-connect`. (Sometimes the kafka-connect container crashes) If kafka-connect is not listed after `docker ps`, start it manually by running `docker-compose up -d kafka-connect`. It should now be running.
* To create the kafka-connect endpoint that sends data to our listening pub/sub topic in Google Cloud Platform, run this command:
```
curl -X POST -H "Content-Type: application/json" --data @pubsub-sink.json http://localhost:8083/connectors
```
* If you need to adjust the config, `vim pubsub-sink.json` to ensure that everything is up to date and aligns with what topics you want to ingest. 

* _Make sure you replace the cps.topic with the pub/sub topic name of your choosing_

* Now go back to the main `snarkOSkafka` repo, and `cargo install --path .` If you are using the metrics crate with grafana as well, `cargo build --features metrics`. 

* Now you can start a node as normal, using `./run-client.sh --private-key $PRIVATE KEY` and the metrics should start going to the topics.
* If you want to view the data in the kafka producer to ensure data is going through, run `kafkacat -b localhost:9092 -t $TOPIC_NAME`





