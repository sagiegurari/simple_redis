var searchIndex = JSON.parse('{\
"simple_redis":{"doc":"simple_redisSimple and resilient redis client based on …","i":[[0,"client","simple_redis","clientImplements the redis client capabilities.",null,null],[3,"Client","simple_redis::client","The redis client which enables to invoke redis operations.",null,null],[11,"is_connection_open","","Returns true if the currently stored connection is valid, …",0,[[["client",3]]]],[11,"quit","","Closes the internal connection to redis. The client can …",0,[[["client",3]],["redisemptyresult",6]]],[11,"run_command","","Invokes the requested command with the provided arguments …",0,[[["client",3],["vec",3]],[["fromredisvalue",8],["redisresult",6]]]],[11,"run_command_from_string_response","","invokes the run_command and returns typed result",0,[[["client",3],["vec",3]],[["redisresult",6],["fromstr",8]]]],[11,"run_command_empty_response","","invokes the run_command but returns empty result",0,[[["client",3],["vec",3]],["redisemptyresult",6]]],[11,"run_command_string_response","","invokes the run_command but returns string result",0,[[["client",3],["vec",3]],["redisstringresult",6]]],[11,"run_command_bool_response","","invokes the run_command but returns bool result",0,[[["client",3],["vec",3]],["redisboolresult",6]]],[11,"subscribe","","Subscribes to the provided channel. Actual subscription …",0,[[["client",3]],["redisemptyresult",6]]],[11,"psubscribe","","Subscribes to the provided channel pattern. Actual …",0,[[["client",3]],["redisemptyresult",6]]],[11,"is_subscribed","","Returns true if subscribed to the provided channel.",0,[[["client",3]]]],[11,"is_psubscribed","","Returns true if subscribed to the provided channel …",0,[[["client",3]]]],[11,"unsubscribe","","Unsubscribes from the provided channel.",0,[[["client",3]],["redisemptyresult",6]]],[11,"punsubscribe","","Unsubscribes from the provided channel pattern.",0,[[["client",3]],["redisemptyresult",6]]],[11,"unsubscribe_all","","Unsubscribes from all channels.",0,[[["client",3]],["redisemptyresult",6]]],[11,"fetch_messages","","Fetches the messages from any of the subscribed channels …",0,[[["client",3],["fnmut",8],["fnmut",8]],["redisemptyresult",6]]],[5,"create","","Constructs a new redis client. The redis connection …",null,[[],[["client",3],["rediserror",4],["result",4]]]],[11,"auth","","See redis AUTH command.",0,[[],["redisemptyresult",6]]],[11,"echo","","See redis ECHO command.",0,[[],["redisstringresult",6]]],[11,"publish","","See redis PUBLISH command.",0,[[],["redisemptyresult",6]]],[11,"get","","See redis GET command.",0,[[["client",3]],[["redisresult",6],["fromstr",8]]]],[11,"get_string","","See redis GET command. This function will always return a …",0,[[["client",3]],["redisstringresult",6]]],[11,"set","","See redis SET command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"setex","","See redis SETEX command.",0,[[["redisarg",8]],["redisemptyresult",6]]],[11,"setnx","","See redis SETNX command.",0,[[["redisarg",8]],["redisemptyresult",6]]],[11,"getset","","See redis GETSET command.",0,[[["redisarg",8]],[["fromstr",8],["redisresult",6]]]],[11,"getset_string","","See redis GETSET command.",0,[[["redisarg",8]],["redisstringresult",6]]],[11,"del","","See redis DEL command.",0,[[],["redisemptyresult",6]]],[11,"exists","","See redis EXISTS command.",0,[[],["redisboolresult",6]]],[11,"expire","","See redis EXPIRE command.",0,[[],["redisemptyresult",6]]],[11,"pexpire","","See redis PEXPIRE command.",0,[[],["redisemptyresult",6]]],[11,"persist","","See redis PERSIST command.",0,[[],["redisemptyresult",6]]],[11,"rename","","See redis RENAME command.",0,[[],["redisemptyresult",6]]],[11,"renamenx","","See redis RENAMENX command.",0,[[],["redisemptyresult",6]]],[11,"append","","See redis APPEND command.",0,[[],["redisemptyresult",6]]],[11,"incr","","See redis INCR command.",0,[[],["redisresult",6]]],[11,"incrby","","See redis INCRBY command.",0,[[["redisarg",8]],["redisresult",6]]],[11,"incrbyfloat","","See redis INCRBYFLOAT command.",0,[[["redisarg",8]],["redisresult",6]]],[11,"strlen","","See redis STRLEN command.",0,[[],["redisresult",6]]],[11,"keys","","See redis KEYS command.",0,[[],[["vec",3],["redisresult",6]]]],[11,"hget","","See redis HGET command.",0,[[["client",3]],[["redisresult",6],["fromstr",8]]]],[11,"hget_string","","See redis HGET command.",0,[[["client",3]],["redisstringresult",6]]],[11,"hgetall","","See redis HGETALL command.",0,[[["client",3]],[["redisresult",6],["hashmap",3]]]],[11,"hset","","See redis HSET command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"hsetnx","","See redis HSETNX command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"hdel","","See redis HDEL command.",0,[[["client",3]],["redisemptyresult",6]]],[11,"hexists","","See redis HEXISTS command.",0,[[["client",3]],["redisboolresult",6]]],[11,"hkeys","","See redis HKEYS command.",0,[[],[["vec",3],["redisresult",6]]]],[11,"hvals","","See redis HVALS command.",0,[[],[["vec",3],["redisresult",6]]]],[11,"lset","","See redis LSET command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"lindex","","See redis HGET command.",0,[[["client",3]],[["redisresult",6],["fromstr",8]]]],[11,"lindex_string","","See redis HGET command.",0,[[["client",3]],["redisstringresult",6]]],[11,"llen","","See redis LLEN command.",0,[[["client",3]],["redisresult",6]]],[11,"lpop","","See redis LPOP command.",0,[[["client",3]],[["redisresult",6],["fromstr",8]]]],[11,"lpush","","See redis LPUSH command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"lpushx","","See redis LPUSHX command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"lrange","","See redis LRANGE command.",0,[[["client",3]],[["vec",3],["redisresult",6]]]],[11,"lrem","","See redis LREM command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"ltrim","","See redis LTRIM command.",0,[[["client",3]],["redisemptyresult",6]]],[11,"rpop","","See redis RPOP command.",0,[[["client",3]],[["redisresult",6],["fromstr",8]]]],[11,"rpush","","See redis RPUSH command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"rpushx","","See redis RPUSHX command.",0,[[["client",3],["redisarg",8]],["redisemptyresult",6]]],[11,"sadd","","See redis SADD command.",0,[[["client",3]],["redisresult",6]]],[11,"scard","","See redis SCARD command.",0,[[["client",3]],["redisresult",6]]],[11,"sdiff","","See redis SDIFF command.",0,[[["client",3],["vec",3]],[["vec",3],["redisresult",6]]]],[11,"sismember","","See redis SISMEMBER command.",0,[[["client",3]],["redisboolresult",6]]],[11,"smembers","","See redis SMEMBERS command.",0,[[["client",3]],[["vec",3],["redisresult",6]]]],[11,"smove","","See redis SMOVE command.",0,[[["client",3]],["redisemptyresult",6]]],[11,"srem","","See redis SREM command.",0,[[["client",3]],["redisemptyresult",6]]],[11,"zadd","","See redis ZADD command.",0,[[["client",3]],["redisresult",6]]],[11,"zrange","","See redis ZRANGE command.",0,[[["client",3]],[["vec",3],["redisresult",6]]]],[0,"types","simple_redis","typesDefines the various types and aliases used or …",null,null],[4,"RedisError","simple_redis::types","Holds the error information",null,null],[13,"RedisError","","Root redis error",1,null],[13,"Description","","Description text of the error reason",1,null],[8,"RedisArg","","Defines a redis command argument",null,null],[6,"Message","","PubSub message",null,null],[6,"RedisResult","","Redis result which either holds a value or a Redis error",null,null],[6,"RedisEmptyResult","","Holds empty result or error",null,null],[6,"RedisStringResult","","Holds string result or error",null,null],[6,"RedisBoolResult","","Holds bool result or error",null,null],[3,"Interrupts","","Enable to modify blocking operations.",null,null],[12,"stop","","Notify blocking operation to stop",2,null],[12,"next_polling_time","","Next polling time in millies",2,null],[11,"new","","Returns a new instance.",2,[[],["interrupts",3]]],[6,"RedisError","simple_redis","Error Type",null,null],[6,"Message","","PubSub message",null,null],[6,"Interrupts","","Blocking operations interrupts",null,null],[6,"RedisResult","","Redis result which either holds a value or a Redis error",null,null],[5,"create","","Constructs a new redis client. The redis connection …",null,[[],[["client",3],["rediserror",6],["result",4]]]],[11,"from","simple_redis::client","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","simple_redis::types","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"clone","","",2,[[],["interrupts",3]]],[11,"default","","",2,[[],["interrupts",3]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats the value using the given formatter.",1,[[["formatter",3]],[["result",4],["error",3]]]],[11,"source","","",1,[[],[["option",4],["error",8]]]]],"p":[[3,"Client"],[4,"RedisError"],[3,"Interrupts"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);