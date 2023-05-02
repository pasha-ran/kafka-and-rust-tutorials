from time import sleep  
from json import dumps  
from kafka import KafkaProducer  

my_producer = KafkaProducer(  
    bootstrap_servers = ['localhost:9092'],  
    value_serializer = lambda x:dumps(x).encode('utf-8')  
    )  

for n in range(10):  
    my_data = {'num' : "hello " + str(n)}  
    my_producer.send('testnum', value = my_data)  
    sleep(0.25)  