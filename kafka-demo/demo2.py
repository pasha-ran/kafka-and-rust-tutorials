from kafka import KafkaProducer
from kafka.errors import KafkaError
from json import dumps  
from kafka import KafkaConsumer

# create a producer. broker is running on localhost
producer = KafkaProducer(retries=5, bootstrap_servers=['localhost:9092'],value_serializer = lambda x:dumps(x).encode('utf-8')  )
# define the on success and on error callback functions
def on_success(record):
    print(record.topic)
    print(record.partition)
    print(record.offset)

def on_error(excp):
    log.error(excp)
    raise Exception(excp)
for n in range(10):
    producer.send('sample', {'key': n}).add_callback(on_success).add_errback(on_error)
# block until all async messages are sent
producer.flush()

consumer = KafkaConsumer('sample',
                         group_id='my-group', enable_auto_commit=False,
                         bootstrap_servers=['localhost:9092'],
           value_deserializer=lambda m: json.loads(m.decode('utf-8')))

for message in consumer:
    print(message.topic)
    print(message.partition)
    print(message.offset)
    print(message.key)