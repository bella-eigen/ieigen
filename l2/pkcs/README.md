# Simple PK Cache Service
Simple local public key cache service on sqlite, with ecies inside.

## Test
`yarn test`

## Usage
```
nohup yarn start &

#query
curl -XGET -H "Content-Type:application/json"  --url "localhost:3000/store?digest=1"

#query all
curl -XGET -H "Content-Type:application/json"  --url "localhost:3000/stores"

# add
curl -XPOST -H "Content-Type:application/json"  --url "localhost:3000/store" -d '{"digest":"1", "public_key":"pk"}'
```

# TODO
1. Access control
