# usimpleflake

A simple distributed id generator for the lazy. It's a simple Python module written in Rust based on [SawdustSofware's](https://github.com/SawdustSoftware/simpleflake).

# Usage

```
import usimpleflake
    
def print_key() -> None:
    distributed_id: int = usimpleflake.simpleflake()
    print(distributed_id)
    
def print_key_with_time() -> None:
    from time import time
    distributed_id: int = usimpleflake.simpleflake(time())
    print(distributed_id)
```
