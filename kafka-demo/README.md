1. download a fresh kafka install, then untar in directory
https://kafka.apache.org/downloads

2. bin/zookeeper-server-start.sh config/zookeeper.properties 
3. bin/kafka-server-start.sh config/server.properties

4. bin/kafka-console-producer.sh --broker-list localhost:9092 --topic topic1

5. bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic topic1

6. start MONGO server if applicable
